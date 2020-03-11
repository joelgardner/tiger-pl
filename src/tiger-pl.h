#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum Expr_Tag {
  IntegerLiteral,
  StringLiteral,
  FloatLiteral,
  NilLiteral,
  Symbol,
  VarDeclaration,
  Let,
  ExprList,
};

enum Type_Tag {
  Int,
  Float,
  Str,
  Defined,
};

struct Int_Body {};
struct Float_Body {};
struct Str_Body {};
struct Defined_Body {
  char* _0;
};

struct Type {
  enum Type_Tag tag;
  union {
    struct Int_Body intType;
    struct Float_Body floatType;
    struct Str_Body strType;
    struct Defined_Body definedType;
  };
};

struct IntegerLiteral_Body {
  int _0;
};

struct FloatLiteral_Body {
  float _0;
};

struct StringLiteral_Body {
  char* _0;
};

struct NilLiteral_Body {};

struct Symbol_Body {
  char* name;
  struct Type type;
};

struct VarDeclaration_Body {
  struct Expr* symbol;
  struct Expr* expr;
};

struct Assignment_Body {
  struct Expr* symbol;
  struct Expr* expr;
};

struct Let_Body {
  struct Expr* decs;
  struct Expr* exprs;
};

struct Nil_Body {};

enum ExprCons_Tag {
  Cons,
  Nil,
};

struct Cons_Body {
  struct Expr* _0;
  struct ExprCons* _1;
};

struct ExprCons {
  enum ExprCons_Tag tag;
  union {
    struct Cons_Body cons;
    struct Nil_Body nil;
  };
};

struct ExprList_Body {
  struct ExprCons _0;
};

struct Expr {
  enum Expr_Tag tag;
  union {
    struct IntegerLiteral_Body integerLiteral;
    struct StringLiteral_Body stringLiteral;
    struct FloatLiteral_Body floatLiteral;
    struct NilLiteral_Body nilLiteral;
    struct Symbol_Body symbol;
    struct VarDeclaration_Body varDeclaration;
    struct Assignment_Body assignment;
    struct Let_Body let;
    struct ExprList_Body exprList;
  };
};

struct ParseContext {
  struct Expr *ast;
};

struct Expr *make_nil();
struct Expr *make_int_literal(int value);
struct Expr *make_float_literal(float value);
struct Expr *make_string_literal(char* s);
struct Expr *make_symbol(char* s);
struct Expr *make_typed_symbol(char* s, struct Type* _type);
struct Expr *make_var_dec(struct Expr* symbol, struct Expr* expr);
struct Expr *make_assignment(struct Expr* symbol, struct Expr* expr);
struct Expr *make_let_exp(struct Expr* decs, struct Expr* exprs);
struct Expr *append_to_exp_list(struct Expr* expr_seq, struct Expr* expr);
struct Expr *make_empty_exp_list();

struct Type *make_int_type();
struct Type *make_float_type();
struct Type *make_str_type();
struct Type *make_defined_type(char* s);
