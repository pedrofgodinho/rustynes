use std::str::FromStr;
use proc_macro2::TokenStream;
use crate::{disassemble_op2, instruction_match2};

#[test]
fn test_macro() {
    let stream = TokenStream::from_str("
        opcode {
                bcc: 0x90 => Relative (2);
        }
    ").unwrap();

    let output = disassemble_op2(stream);

    println!("{}", output);
    println!("{:?}", output);
}