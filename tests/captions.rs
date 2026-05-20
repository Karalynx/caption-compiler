
use std::env::temp_dir;

use caption_compiler::{captions::*, cli::{self, Compile, CompileError, DescribeError}};

#[test]
fn compile() {
    let temp_dir = temp_dir();
    let compile_args = Compile { verbose: false, output: Some(temp_dir) };

    // Success
    assert!(cli::compile("tests/compilation/valid_english.txt", compile_args.clone()).is_ok());
    
    // Success, caption size equals 8192 bytes
    assert!(cli::compile("tests/compilation/valid_length_equals_8192.txt", compile_args.clone()).is_ok());

    // Error, missing quote
    assert_eq!(
        cli::compile("tests/compilation/invalid_format.txt", compile_args.clone()),
        Err(CompileError::Parse(ClosedCaptionError::Format(
            CaptionParseError::UnexpectedEof { expected: TokenKind::String }
        )))
    );

    // Error, missing 'Tokens'
    assert_eq!(
        cli::compile("tests/compilation/invalid_format2.txt", compile_args.clone()),
        Err(CompileError::Parse(ClosedCaptionError::Format(
            CaptionParseError::LiteralMismatch { line: 4, expected: "Tokens".into(), found: "{".into() }
        )))
    );

    // Error, caption size exceeds 8192 bytes
    assert_eq!(
        cli::compile("tests/compilation/invalid_length_exceeds_8192.txt", compile_args.clone()),
        Err(CompileError::Parse(ClosedCaptionError::CaptionLength { id: "barn.bunyip".into() }))
    );

    // Empty file
    assert_eq!(
        cli::compile("tests/compilation/invalid_empty.txt", compile_args.clone()),
        Err(CompileError::Parse(ClosedCaptionError::Format(
            CaptionParseError::UnexpectedEof { expected: TokenKind::String }
        )))
    );
}

#[test]
fn describe() {
    // Valid .DAT file
    assert!(cli::describe("tests/description/valid_english.dat").is_ok());

    // Invalid vccd
    assert_eq!(
        cli::describe("tests/description/invalid_vccd.dat"),
        Err(DescribeError::InvalidVCCD { found: 1145258752 })
    );

    // Invalid version
    assert_eq!(
        cli::describe("tests/description/invalid_version.dat"),
        Err(DescribeError::InvalidVersion { found: 2 })
    );

    // Missing captions
    assert!(cli::describe("tests/description/invalid_missing_data.dat").is_err());

    // Empty file
    assert!(cli::describe("tests/description/invalid_empty.dat").is_err());
}