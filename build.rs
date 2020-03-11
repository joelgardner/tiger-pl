extern crate cc;
use std::process::Command;

fn main() {
    Command::new("sh")
        .arg("-c")
        .arg("bison --defines=src/parser.tab.h --output=src/parser.tab.c src/parser.y")
        .output()
        .expect("Failed to build Bison file parse.y!");

    Command::new("sh")
        .arg("-c")
        .arg("flex --outfile=src/lex.yy.c src/lexer.l")
        .output()
        .expect("Failed to build Flex file lexer.l!");

    cc::Build::new()
        .file("src/parser.tab.c")
        .file("src/lex.yy.c")
        .compile("parser.a");
}
