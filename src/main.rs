extern crate libc;
use std::ffi::CString;

mod nodes;
use nodes::ParseContext;

extern "C" {
    fn parse_file(filename: *const libc::c_char, astx: *const ParseContext);
    fn parse_string(input: *const libc::c_char, astx: *const ParseContext);
}

fn main() {
    let result = unsafe {
        let ctx = ParseContext { ast: None };
        // let filename = "test/test2.tig";
        // parse_file(CString::new(filename).unwrap().as_ptr(), &ctx);

        let tiger_code = "let var a : int := 12345 in a := 54321; end";
        parse_string(CString::new(tiger_code).unwrap().as_ptr(), &ctx);
        ctx.ast.expect("Could not get AST, see yyerror.")
    };

    println!("{:#?}", result);
}
