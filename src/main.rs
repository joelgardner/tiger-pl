extern crate libc;
use std::ffi::CString;

mod nodes;
use nodes::ParseContext;

extern "C" {
    fn parse_file(filename: *const libc::c_char, astx: *const ParseContext);
}

fn main() {
    let filename = "test/test2.tig";

    let result = unsafe {
        let ctx = ParseContext { ast: None };
        parse_file(CString::new(filename).unwrap().as_ptr(), &ctx);
        ctx.ast.expect("Could not get AST, see yyerror.")
    };

    println!("{:#?}", result);
}
