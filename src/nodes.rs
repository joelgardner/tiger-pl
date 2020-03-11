use std::ffi::CStr;
use std::os::raw::{c_char, c_float, c_int};

#[derive(Debug)]
#[repr(C)]
pub enum Expr {
    IntegerLiteral(i32),
    FloatLiteral(f32),
    StringLiteral(String),
    NilLiteral,
    Symbol { name: String, r#type: Option<Type> },
    VarDeclaration { symbol: Box<Expr>, expr: Box<Expr> },
    Assignment { symbol: Box<Expr>, expr: Box<Expr> },
    Let { decs: Box<Expr>, exprs: Box<Expr> },
    ExprList(ExprCons),
}

#[derive(Debug)]
#[repr(C)]
pub enum Type {
    Int,
    Float,
    Str,
    Defined(String),
}

#[derive(Debug)]
#[repr(C)]
pub enum ExprCons {
    Cons(Box<Expr>, Box<ExprCons>),
    Nil,
}

#[no_mangle]
pub extern "C" fn make_symbol(symbol_name: *const c_char) -> Box<Expr> {
    let name = unsafe { CStr::from_ptr(symbol_name).to_str().unwrap().to_owned() };
    println!("in make_symbol {}", name);
    Box::new(Expr::Symbol { name, r#type: None })
}

#[no_mangle]
pub extern "C" fn make_typed_symbol(
    symbol_name: *const c_char,
    r#type: Box<Option<Type>>,
) -> Box<Expr> {
    let name = unsafe { CStr::from_ptr(symbol_name).to_str().unwrap().to_owned() };
    println!("in make_typed_symbol {} {:?}", name, r#type);
    Box::new(Expr::Symbol {
        name,
        r#type: *r#type,
    })
}

#[no_mangle]
pub extern "C" fn make_int_type() -> Box<Type> {
    println!("in make_int_type");
    Box::new(Type::Int)
}

#[no_mangle]
pub extern "C" fn make_float_type() -> Box<Type> {
    println!("in make_float_type");
    Box::new(Type::Float)
}

#[no_mangle]
pub extern "C" fn make_str_type() -> Box<Type> {
    println!("in make_str_type");
    Box::new(Type::Str)
}

#[no_mangle]
pub extern "C" fn make_defined_type(type_name: *const c_char) -> Box<Type> {
    let name = unsafe { CStr::from_ptr(type_name).to_str().unwrap().to_owned() };
    println!("in make_defined_type");
    Box::new(Type::Defined(name))
}

#[no_mangle]
pub extern "C" fn make_let_exp(decs: Box<Expr>, exprs: Box<Expr>) -> Box<Expr> {
    println!("in make_let_exp");
    Box::new(Expr::Let { decs, exprs })
}

#[no_mangle]
pub extern "C" fn make_int_literal(value: c_int) -> Box<Expr> {
    println!("in make_int_literal {}", value);
    Box::new(Expr::IntegerLiteral(value))
}

#[no_mangle]
pub extern "C" fn make_float_literal(value: c_float) -> Box<Expr> {
    println!("in make_float_literal {}", value);
    Box::new(Expr::FloatLiteral(value))
}

#[no_mangle]
pub extern "C" fn make_string_literal(s: *const c_char) -> Box<Expr> {
    let string = unsafe {
        CStr::from_ptr(s)
            .to_str()
            .expect("Could not get string passed from Bison.")
            .to_owned()
    };
    println!("in make_string_literal {}", string);
    Box::new(Expr::StringLiteral(string))
}

#[no_mangle]
pub extern "C" fn make_var_dec(symbol: Box<Expr>, expr: Box<Expr>) -> Box<Expr> {
    println!("in make_var_dec {:?} := {:?}", symbol, expr);
    Box::new(Expr::VarDeclaration { symbol, expr })
}

#[no_mangle]
pub extern "C" fn make_assignment(symbol: Box<Expr>, expr: Box<Expr>) -> Box<Expr> {
    println!("in make_assignment {:?} := {:?}", *symbol, *expr);
    Box::new(Expr::Assignment { symbol, expr })
}

#[no_mangle]
pub extern "C" fn make_empty_exp_list() -> Box<Expr> {
    println!("in make_empty_exp_list");
    Box::new(Expr::ExprList(ExprCons::Nil))
}

#[no_mangle]
pub extern "C" fn append_to_exp_list(exp_list: Box<Expr>, exp: Box<Expr>) -> Box<Expr> {
    println!("in append_to_exp_list {:?} {:?} ", exp_list, exp);
    if let Expr::ExprList(cons) = *exp_list {
        Box::new(Expr::ExprList(ExprCons::Cons(exp, Box::new(cons))))
    } else {
        panic!("Trying to append to a non ExprList. {:?}", exp_list)
    }
}

#[no_mangle]
pub extern "C" fn make_nil() -> Box<Expr> {
    println!("in make_empty_exp_list");
    Box::new(Expr::NilLiteral)
}
