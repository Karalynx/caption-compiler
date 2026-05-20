
use std::{fs::{self, File}, io::{self, BufWriter, Seek, Write}, mem, path::Path};

use encoding_rs::UTF_16LE;
use packbytes::ToBytes;

use crate::{captions::ClosedCaptions, cli::{CaptionEntry, Compile, CompileError, Header}};

fn write_captions(captions: &ClosedCaptions, out_path: &Path, verbose: bool) -> io::Result<()> {
    let mut header = Header {
        vccd: Header::VCCD, version: Header::VERSION,
        block_count: 0, block_size: Header::BLOCK_SIZE,
        dir_size: 0, data_offset: 0
    };

    let out_file = File::create(&out_path)?;
    let mut wrt = BufWriter::new(out_file);
    wrt.seek(io::SeekFrom::Start(mem::size_of::<Header>() as u64))?;


    let mut caption_buf = Vec::<u8>::with_capacity(captions.tokens.len() * 0xFFF);
    let mut last_offset = 0u16;

    for (hash, val) in captions.tokens.iter() {
        let mut caption = Vec::with_capacity((val.len() + 1) << 1);
        caption.extend(val.encode_utf16().flat_map(|x| x.to_le_bytes()));
        caption.extend_from_slice(&[0, 0]); // Add NULL terminator

        if (last_offset + caption.len() as u16) > header.block_size as u16 {
            
            let leftover = header.block_size as u16 - last_offset;
            caption_buf.resize(caption_buf.len() + leftover as usize, 0);
            
            header.block_count += 1;
            last_offset = 0;
        }

        let entry = CaptionEntry {
            crc32: *hash,
            length: caption.len() as u16,
            block: header.block_count,
            offset: last_offset
        };

        last_offset += entry.length;
        header.dir_size += 1;

        if verbose {
            println!("Writing caption data for {val:?}\n{entry}\n");
        }

        wrt.write_all(&entry.to_le_bytes())?;
        
        caption_buf.append(&mut caption);
    }

    let offset = mem::size_of::<Header>() as i32 + header.dir_size * mem::size_of::<CaptionEntry>() as i32;
    let dict_padding = 512 - (offset % 512);
    
    header.block_count += 1;
    header.data_offset = offset + dict_padding;

    if verbose {
        println!("Padding dictionary with {dict_padding} zeroes\n");
    }

    wrt.write_all(&vec![0u8; dict_padding as usize])?;

    if verbose {
        println!("Writing caption strings of length {}\n", caption_buf.len());
    }

    wrt.write_all(&caption_buf)?;
    
    let leftover = header.block_size as u16 - last_offset;
    if verbose {
        println!("Padding caption strings with {leftover} zeroes\n");
    }

    wrt.write_all(&vec![0u8; leftover as usize])?;

    if verbose {
        println!("Writing header\n{header}\n");
    }

    wrt.seek(io::SeekFrom::Start(0))?;
    wrt.write_all(&header.to_le_bytes())?;
    
    if verbose {
        println!("Successfully compiled to \"{}\"\n", out_path.to_string_lossy());
    }

    Ok(())
}

pub fn compile<P: AsRef<Path>>(filepath: P, args: Compile) -> Result<(), CompileError> {
    let filepath = filepath.as_ref();
    let out_path = match args.output {
        Some(mut x) => {
            x.push(filepath.file_stem().unwrap_or_default());
            x.set_extension("dat");
            x
        },
        None => filepath.with_extension("dat")
    };

    let bytes = fs::read(filepath)
        .map_err(|err| CompileError::Read(err))?;
    
    let contents = UTF_16LE.decode_with_bom_removal(&bytes).0;
    
    let captions = ClosedCaptions::parse(&contents)?;
    if args.verbose {
        println!("Parsed {} tokens\n\n", captions.tokens.len());
    }

    write_captions(&captions, &out_path, args.verbose).map_err(|err| {
        CompileError::Write(err)
    })
}
