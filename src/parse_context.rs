use nodes::Expr;
use std::ffi::CString;

extern "C" {
    #[allow(improper_ctypes)]
    fn parse_file(filename: *const libc::c_char, astx: *const ParseContext);

    #[allow(improper_ctypes)]
    fn parse_string(input: *const libc::c_char, astx: *const ParseContext);
}

pub fn get_ast_file(filename: &str) -> ParseContext {
    let ctx = ParseContext { ast: None };
    ctx.parse_file(filename);
    ctx
}

pub fn get_ast_string(tiger_code: &str) -> ParseContext {
    let ctx = ParseContext { ast: None };
    ctx.parse_string(tiger_code);
    ctx
}

#[derive(Debug)]
#[repr(C)]
pub struct ParseContext {
    pub ast: Option<Box<Expr>>,
}

impl ParseContext {
    fn parse_file(&self, filename: &str) {
        unsafe {
            parse_file(CString::new(filename).unwrap().as_ptr(), self);
        }
    }

    fn parse_string(&self, tiger_code: &str) {
        unsafe {
            parse_string(CString::new(tiger_code).unwrap().as_ptr(), self);
        }
    }
}
