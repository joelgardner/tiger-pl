extern crate libc;
use parse_context::{get_ast_file, get_ast_string};

pub mod nodes;
pub mod parse_context;

fn main() {
    let file = "samples/test2.tig";
    println!("{:#?}", get_ast_file(file));

    let tiger_code = "let var a : int := 12345 in a := 54321; end";
    println!("{:#?}", get_ast_string(tiger_code));
}
