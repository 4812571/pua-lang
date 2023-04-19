#![no_main]

use libfuzzer_sys::fuzz_target;
use pua_lang::lexer;
use pua_lang::parser;

fuzz_target!(|data: &[u8]| {
    let program = String::from_utf8_lossy(data);
    let lexer = lexer::Lexer::new(&program);
    let mut parser = parser::Parser::new(lexer);
    let _ = parser.parse();
});