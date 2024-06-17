
mod header;
mod entry;

use std::{fs::File, io::{self, BufWriter, Read, Seek, Write}, mem, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use encoding_rs::UTF_16LE;
use encoding_rs_io::DecodeReaderBytesBuilder;
use keyvalues_parser::*;
use entry::CaptionEntry;
use header::Header;

#[derive(Parser, Debug)]
#[command(author = "Kara")]
#[command(name = "Caption Compiler")]
#[command(about = "Compiles and describes Valve's closed captions", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    task: Task,

    #[arg(short, long, help = "Input filepath")]
    input: PathBuf,
}

#[derive(Subcommand, Debug, Default)]
enum Task {
    #[clap(name = "compile", about = "Compiles to .DAT file")]
    Compile(Compile),

    #[clap(name = "describe", about = "Describes .DAT file")]
    #[default] Describe
}

#[derive(Args, Debug, Default)]
pub struct Compile {
    /// Verbose output
    #[arg(short, long, help = "Verbose output")]
    verbose: bool,

    /// Output filepath
    #[arg(short, long, help = "Output filepath")]
    output: Option<PathBuf>,
}

fn compile(in_filepath: PathBuf, comp_args: Compile) -> io::Result<()> {
    let mut out_filepath = comp_args.output.unwrap_or(in_filepath.clone());
    out_filepath.set_extension("dat");

    let caption_file = File::open(in_filepath)?;
    let mut caption_rdr = DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::UTF_16LE))
        .build(caption_file);
    
    let mut contents = String::new();
    caption_rdr.read_to_string(&mut contents)?;
    
    let vdf = Vdf::parse(&contents).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
    let keyvalues = vdf.value.get_obj()
        .and_then(|lang| lang.get_key_value("Tokens"))
        .and_then(|tokens| tokens.1[0].get_obj())
        .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Invalid closed caption format"))?;

    if comp_args.verbose {
        println!("Parsed {} tokens\n\n", keyvalues.len());
    }

    let out_file = File::create(&out_filepath)?;
    let mut wrt = BufWriter::new(out_file);
    wrt.seek(io::SeekFrom::Start(mem::size_of::<Header>() as u64))?;

    
    let mut header = Header::default();
    let mut caption_data = CaptionEntry::default();

    let mut caption_buf = Vec::<u16>::with_capacity(keyvalues.len() * 0xfff);
    let mut dir_size = 0i32;

    for (key, value) in keyvalues.iter()
        .map(|(key, value)| (key.to_lowercase(), value))
        .filter(|(key, _)| !key.starts_with("[english]")) {
        
        let mut caption = value[0].get_str()
            .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Values must be string literals"))
            .map(|str| {
                let mut caption: Vec<u16> = str.encode_utf16().collect();
                caption.push(0);

                caption
            })?;
        
        if (caption.len() << 1) > header.block_size as usize {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Caption length exceeds {} bytes: {:?}", header.block_size, String::from_utf16(&caption))))
        }

        caption_data.length = (caption.len() as u16) << 1;
        if (caption_data.offset + caption_data.length) > header.block_size as u16 {
            
            let leftover = header.block_size as u16 - caption_data.offset;
            caption_buf.resize(caption_buf.len() + (leftover >> 1) as usize, 0);
            
            header.block_count += 1;
            caption_data.offset = 0;
        }

        caption_data.crc32 = crc32fast::hash(key.as_bytes());
        caption_data.block = header.block_count;

        if comp_args.verbose {
            println!("Writing caption data for {:?}\n{}\n", String::from_utf16(&caption).unwrap(), caption_data);
        }

        wrt.write_all(unsafe {mem::transmute::<&CaptionEntry, &[u8; mem::size_of::<CaptionEntry>()]>(&caption_data)})?;     

        caption_buf.append(&mut caption);
        caption_data.offset += caption_data.length;
        dir_size += 1;
    }

    let dict_padding = 512 - ((mem::size_of::<Header>() as i32 + dir_size * 12) % 512);
    header.block_count += 1;
    header.dir_size = dir_size;
    header.data_offset = (mem::size_of::<Header>() as i32 + dir_size * 12) + dict_padding;

    if comp_args.verbose {
        println!("Padding dictionary with {} zeroes\n", dict_padding);
    }

    wrt.write_all(&vec![0u8; dict_padding as usize])?;

    if comp_args.verbose {
        println!("Writing caption strings of length {}\n", caption_buf.len() << 1);
    }

    let buf = mem::ManuallyDrop::new(caption_buf);
    wrt.write_all(unsafe { &Vec::<u8>::from_raw_parts(buf.as_ptr() as *mut u8, buf.len() << 1, buf.capacity() << 1)})?;
    
    let leftover = header.block_size as u16 - caption_data.offset;
    if comp_args.verbose {
        println!("Padding caption strings with {} zeroes\n", leftover);
    }

    wrt.write_all(&vec![0u8; leftover as usize])?;

    if comp_args.verbose {
        println!("Writing header\n{}\n", header);
    }

    wrt.seek(io::SeekFrom::Start(0))?;
    wrt.write_all(unsafe {mem::transmute::<&Header, &[u8; mem::size_of::<Header>()]>(&header)})?;
    
    if comp_args.verbose {
        println!("Successfully compiled to {:?}\n", out_filepath);
    }

    Ok(())
}

fn describe(in_filepath: PathBuf) -> io::Result<()> {
    let mut dat_file = File::open(in_filepath)?;

    let header = Header::from_reader(&mut dat_file)?;

    let mut dir_pos = mem::size_of::<Header>();
    for _ in 0..header.dir_size {
        let entry = CaptionEntry::from_reader(&mut dat_file)?;
        dir_pos += mem::size_of::<CaptionEntry>();
        

        dat_file.seek(io::SeekFrom::Start((header.data_offset + (entry.block * header.block_size) + entry.offset as i32) as u64))?;
        
        let mut contents = vec![0u8; entry.length as usize];
        dat_file.read_exact(&mut contents)?;
        
        dat_file.seek(io::SeekFrom::Start(dir_pos as u64))?;

        
        let (str, _, _) = UTF_16LE.decode(&contents);
        println!("Caption: {:?}\n{}", str, entry);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    match args.task {
        Task::Compile(comp_args) => compile(args.input, comp_args),
        Task::Describe => describe(args.input)
    }
}