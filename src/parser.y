%{
#include <stdio.h>
#include <string.h>
#include "tiger-pl.h"
int yylex();
int yyerror();
FILE *yyin;

void set_input_string(char* input);
void end_lexical_scan();
void reset_line_number_info();

%}

%glr-parser
%expect-rr 1
%locations

%define parse.error verbose
%parse-param { void *astx }

%union {
  int operator;
  int inumber;
  float fnumber;
  char* string;
  struct Expr *ast;
  struct Type *type;
}

%token <inumber>  INT_LITERAL
%token <fnumber>  FLOAT_LITERAL
%token <string>   STRING_LITERAL
%token            NIL
%token            TYPE
%token            INT
%token            FLOAT
%token            STRING
%token            ARRAY
%token            FUNCTION
%right            OF
%token            END
%token            EQUALS
%token            DOT
%token            LET
%token            VAR
%token            IN
%token            SEMICOLON
%token            COLON
%token            COMMA
%right            WHILE DO FOR TO
%right            IF THEN ELSE
%token            BREAK
%right            ASSIGN
%left             BIT_OR
%left             BIT_AND
%left             EQUALS NOT_EQUALS LT GT LTE GTE
%left             ADD SUB
%left             MUL DIV
%right            NEG
%token            LBRACKET RBRACKET
%token            LCURLY RCURLY
%token            LPAREN RPAREN
%token <string>   IDENTIFIER
%token <string>   TYPE_IDENTIFIER

%type  <ast>      prog var_dec literal exp exps seq_exp lvalue assignment let_exp decs dec
%type  <type>     type_identifier

%%

prog:
    exp                 { ((struct ParseContext *)astx)->ast = $1; }
  ;

exp:
    let_exp
  | seq_exp
  | assignment
  | infix_exp
  | literal
  | arr_create
  | record_create
  | lvalue
  | fn_call
  | control_flow
  | BREAK
  | NIL                 { $$ = make_nil(); }
  | SUB exp %prec NEG
  ;

exps:
    /* empty */         { $$ = NULL; }
  | exp                 { $$ = append_to_exp_list(make_empty_exp_list(), $1); }
  | exp SEMICOLON exps  { $$ = append_to_exp_list($3 == NULL ? make_empty_exp_list() : $3, $1); }
  ;

seq_exp:
    LPAREN exps RPAREN  { $$ = $2; }
  ;

let_exp:
    LET decs IN exps END    { $$ = make_let_exp($2, $4); }
  ;

decs:
    /* empty */         { $$ = NULL; }
  | dec decs            { $$ = append_to_exp_list($2 == NULL ? make_empty_exp_list() : $2, $1); }
  ;

dec:
    var_dec
  | type_dec
  | function_dec
  ;

assignment:
    lvalue ASSIGN exp   { $$ = make_assignment($1, $3); }
  ;

control_flow:
    IF exp THEN exp ELSE exp
  | IF exp THEN exp
  | WHILE exp DO exp
  | FOR IDENTIFIER ASSIGN exp TO exp DO exp
  ;

lvalue:
    IDENTIFIER %expect-rr 1         { $$ = make_symbol($1); }
  | lvalue LBRACKET exp RBRACKET
  | lvalue DOT IDENTIFIER
  ;

var_dec:
    VAR IDENTIFIER ASSIGN exp                         { $$ = make_var_dec(make_symbol($2), $4); }
  | VAR IDENTIFIER COLON type_identifier ASSIGN exp   { $$ = make_var_dec(make_typed_symbol($2, $4), $6); }
  ;

type_dec:
    TYPE IDENTIFIER EQUALS type_definition
  ;

fn_call:
    IDENTIFIER LPAREN fn_params RPAREN
  ;

fn_params:
    /* empty */
  | exp
  | exp COMMA exps
  ;

type_definition:
    type_identifier
  | ARRAY OF type_identifier
  | record_type
  ;

record_type:
    LCURLY field_decs RCURLY
  ;

field_decs:
    /* empty */
  | field_dec
  | field_decs COMMA field_dec
  ;

field_dec:
    IDENTIFIER COLON type_identifier
  ;

function_dec:
    FUNCTION IDENTIFIER LPAREN field_decs RPAREN COLON type_identifier EQUALS exp
  | FUNCTION IDENTIFIER LPAREN field_decs RPAREN EQUALS exp
  ;

type_identifier:
    INT                         { $$ = make_int_type(); }
  | FLOAT                       { $$ = make_float_type(); }
  | STRING                      { $$ = make_str_type(); }
  | IDENTIFIER %expect-rr 1     { $$ = make_defined_type($1); }
  ;

infix_exp:
    exp ADD exp
  | exp SUB exp
  | exp MUL exp
  | exp DIV exp
  | exp EQUALS exp
  | exp NOT_EQUALS exp
  | exp LT exp
  | exp GT exp
  | exp LTE exp
  | exp GTE exp
  | exp BIT_AND exp
  | exp BIT_OR exp
  ;

arr_create:
    type_identifier LBRACKET exp RBRACKET OF exp
  ;

record_create:
    type_identifier LCURLY field_creates RCURLY
  ;

field_creates:
    /* empty */
  | field_create
  | field_creates COMMA field_create
  ;

field_create:
    IDENTIFIER EQUALS exp
  ;

literal:
    FLOAT_LITERAL     { $$ = make_float_literal($1); }
  | INT_LITERAL       { printf("location of int: %d: %d\n", @1.first_line, @1.first_column); $$ = make_int_literal($1); }
  | STRING_LITERAL    { $$ = make_string_literal($1); }
  ;

%%

void parse_file(char *filename, struct ParseContext* astx) {
  FILE *f = fopen(filename, "r");
  if (!f) {
    printf("Error opening file %s", filename);
    return;
  }
  yyin = f;
  reset_line_number_info();
  yyparse(astx);
}

void parse_string(char *input, struct ParseContext* astx) {
  reset_line_number_info();
  set_input_string(input);
  yyparse(astx);
  end_lexical_scan();
}

int yyerror(char *s) {
	printf("Syntax Error on line %s\n", s);
	return 0;
}
