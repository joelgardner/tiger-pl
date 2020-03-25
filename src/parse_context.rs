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

#[cfg(test)]
mod tests {
    use nodes::Expr::{Assignment, ExprList, IntegerLiteral, Let, Symbol, VarDeclaration};
    use nodes::ExprCons::{Cons, Empty};
    use nodes::Type::Int;
    use parse_context::get_ast_string;

    #[test]
    fn test_string_is_parsed() {
        let tiger_code = "
        let
            var a : int := 12345
        in
            a := 54321;
        end";
        let ctx = get_ast_string(tiger_code);
        let result = ctx.ast.expect("Parsing failed.");

        let expected = "Let { decs: ExprList(Cons(VarDeclaration { symbol: Symbol { name: \"a\", type: Some(Int) }, expr: IntegerLiteral(12345) }, Empty)), exprs: ExprList(Cons(Assignment { symbol: Symbol { name: \"a\", type: None }, expr: IntegerLiteral(54321) }, Empty)) }";
        let expected2 = Let {
            decs: ExprList(Cons(
                VarDeclaration {
                    symbol: Symbol {
                        name: "a".to_string(),
                        r#type: Some(Int),
                    }
                    .into_boxed(),
                    expr: IntegerLiteral(12345).into_boxed(),
                }
                .into_boxed(),
                Box::new(Empty),
            ))
            .into_boxed(),
            exprs: Box::new(ExprList(Cons(
                Box::new(Assignment {
                    symbol: Box::new(Symbol {
                        name: "a".to_string(),
                        r#type: None,
                    }),
                    expr: Box::new(IntegerLiteral(54321)),
                }),
                Box::new(Empty),
            ))),
        };
        assert_eq!(expected2, *result);
    }
}
