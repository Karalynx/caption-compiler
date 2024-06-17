include!("../src/main.rs");

use crate::header::*;

use std::io::Cursor;

#[test]
fn version() {
    // Valid version
    assert!(Version::new(1).is_some());

    // Invalid version
    assert!(Version::new(0).is_none());
}

#[test]
fn vccd() {
    // Valid VCCD
    assert!(VCCD::new(1145258838).is_some());

    // Invalid VCCD
    assert!(VCCD::new(0).is_none());
}

#[test]
fn header() {
    // Valid header
    assert!(Header::from_reader(&mut Cursor::new([86, 67, 67, 68,  1, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0])).is_ok());

    // Invalid version
    assert!(Header::from_reader(&mut Cursor::new([86, 67, 67, 68,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0])).is_err());

    // Invalid VCCD
    assert!(Header::from_reader(&mut Cursor::new([0, 0, 0, 0,  1, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0])).is_err());

    // 4 bytes short
    assert!(Header::from_reader(&mut Cursor::new([86, 67, 67, 68,  1, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0])).is_err());

    // Buffer empty
    assert!(Header::from_reader(&mut Cursor::new([])).is_err());
}

#[test]
fn caption_entry() {
    // Valid entry
    assert!(CaptionEntry::from_reader(&mut Cursor::new([0; 12])).is_ok());

    // 2 bytes short
    assert!(CaptionEntry::from_reader(&mut Cursor::new([0; 10])).is_err());

    // Buffer empty
    assert!(CaptionEntry::from_reader(&mut Cursor::new([])).is_err());
}

#[test]
fn caption_compile() {
    // Success
    assert!(compile("tests/compilation/valid_english.txt".into(), Default::default()).is_ok());

    // Success, ignores captions starting with [english]
    assert!(compile("tests/compilation/valid_russian.txt".into(), Default::default()).is_ok());
    
    // Dir size must be 2
    let mut russian_dat = File::open("tests/compilation/valid_russian.dat").unwrap();
    let header = Header::from_reader(&mut russian_dat).unwrap();
    assert_eq!(header.dir_size, 2);

    // Success, caption size equals 8192 bytes
    assert!(compile("tests/compilation/valid_length_equals_8192.txt".into(), Default::default()).is_ok());

    // Error, missing quote
    assert!(compile("tests/compilation/invalid_format.txt".into(), Default::default()).is_err());

    // Error, missing 'Tokens'
    assert!(compile("tests/compilation/invalid_format2.txt".into(), Default::default()).is_err());

    // Error, caption size exceeds 8192 bytes
    assert!(compile("tests/compilation/invalid_length_exceeds_8192.txt".into(), Default::default()).is_err());

    // Empty file
    assert!(compile("tests/compilation/invalid_empty.txt".into(), Default::default()).is_err());
}

#[test]
fn caption_describe() {
    // Valid .DAT file
    assert!(describe("tests/description/valid_english.dat".into()).is_ok());

    // Invalid vccd
    assert!(describe("tests/description/invalid_vccd.dat".into()).is_err());

    // Invalid version
    assert!(describe("tests/description/invalid_version.dat".into()).is_err());

    // Missing captions
    assert!(describe("tests/description/invalid_missing_data.dat".into()).is_err());

    // Empty file
    assert!(describe("tests/description/invalid_empty.dat".into()).is_err());
}