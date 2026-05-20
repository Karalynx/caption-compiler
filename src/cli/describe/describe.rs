
use core::mem;
use std::{fs, io::{self, Cursor, Read, Seek}, path::Path};

use encoding_rs::UTF_16LE;

use crate::cli::{CaptionEntry, DescribeError, Header};

pub fn describe<P: AsRef<Path>>(filepath: P) -> Result<(), DescribeError> {
    let contents = fs::read(filepath)?;
    let mut rdr = Cursor::new(contents);

    let header = Header::from_reader(&mut rdr)?;

    let mut dir_pos = mem::size_of::<Header>() as u64;
    for _ in 0 .. header.dir_size {
        let entry = CaptionEntry::from_reader(&mut rdr)?;
        dir_pos += mem::size_of::<CaptionEntry>() as u64;
        
        let offset = header.data_offset + (entry.block * header.block_size) + entry.offset as i32;
        rdr.seek(io::SeekFrom::Start(offset as u64))?;
        
        let mut contents = vec![0u8; (entry.length - 2) as usize]; // Emit NULL
        rdr.read_exact(&mut contents)?;

        rdr.seek(io::SeekFrom::Start(dir_pos))?;

        println!(
            "Caption: {:?}\n{}",
            UTF_16LE.decode_without_bom_handling(&contents).0,
            entry
        );
    }

    Ok(())
}
