%{
#include <stdio.h>
#include "parser.tab.h"
#define MAX_STRING_SIZE 2048


int yyerror();
int line_num = 1;
int nested_comments = 0;
int next_identifier_is_type = 0;

struct node  {
  char* type_name;
  struct node* next;
};

char string_buf[MAX_STRING_SIZE];
char *string_buf_ptr;

%}

%option noyywrap
%option nounput
%option noinput

%x comment
%x str

float       ([0-9]+\.[0-9]+)
integer     ([0-9])+
identifier  ([a-zA-Z][a-zA-Z0-9_]*)

%%


\"                      { string_buf_ptr = string_buf; BEGIN(str); }
<str>{
  \n                    { yyerror("Unterminated string contains unescaped newline."); }
  [^\\\n\"]+            { /* Eat up anything inside the string that isn't an escape/newline/" */
		                      char *yptr = yytext;
                          while (*yptr) {
                            *string_buf_ptr++ = *yptr++;
                          }
                        }
  \\n                   { *string_buf_ptr++ = '\n'; }
  \\(.|\n)              { *string_buf_ptr++ = yytext[1]; }
  "\""                  { /* Closing quote - all done */
                          BEGIN(INITIAL);
                          *string_buf_ptr = '\0';
                          yylval.string = strdup(string_buf);
                          return STRING_LITERAL;
                        }
}
{float}                 { yylval.fnumber = atof(yytext); return FLOAT_LITERAL; }
{integer}               { yylval.inumber = atoi(yytext); return INT_LITERAL; }
":="                    { return ASSIGN; }
"+"                     { return ADD; }
"-"                     { return SUB; }
"*"                     { return MUL; }
"/"                     { return DIV; }
"="                     { return EQUALS; }
"<>"                    { return NOT_EQUALS; }
"<"                     { return LT; }
">"                     { return GT; }
">="                    { return GTE; }
"<="                    { return LTE; }
"&"                     { return BIT_AND; }
"|"                     { return BIT_OR; }
";"                     { return SEMICOLON; }
":"                     { return COLON; }
"{"                     { return LCURLY; }
"}"                     { return RCURLY; }
"("                     { return LPAREN; }
")"                     { return RPAREN; }
"["                     { return LBRACKET; }
"]"                     { return RBRACKET; }
","                     { return COMMA; }
"."                     { return DOT; }
"nil"                   { return NIL; }
"let"                   { return LET; }
"if"                    { return IF; }
"then"                  { return THEN; }
"else"                  { return ELSE; }
"while"                 { return WHILE; }
"do"                    { return DO; }
"for"                   { return FOR; }
"break"                 { return BREAK; }
"to"                    { return TO; }
"in"                    { return IN; }
"end"                   { return END; }
"var"                   { return VAR; }
"array"                 { return ARRAY; }
"function"              { return FUNCTION; }
"of"                    { return OF; }
"type"                  { return TYPE; }
"int"                   { return INT; }
"float"                 { return FLOAT; }
"string"                { return STRING; }
\n                      { ++line_num; }
"/*"                    { /* Encountered beginning of comment */ BEGIN(comment); }
<comment>{
  "/*"                  { ++nested_comments; }
  "*"+"/"               { if (!nested_comments--) { BEGIN(INITIAL); } }
  "*"+                  /* Eat *s that don't come before /s. */
  [^*/\n]+              /* Eat anything that isn't a *, /, or new-line. */
  \/                    /* Eat /s one at a time so we can stop before consuming a *. */
  \n                    { ++line_num; }
}
{identifier}            { yylval.string = strdup(yytext); return IDENTIFIER; }
[ \t\r]+                ; // Ignore whitespace
.                       { printf("Unknown token: %s\n", yytext); exit(1); }

%%

void set_input_string(char* input) {
  yy_scan_string(input);
}

void end_lexical_scan() {
  yy_delete_buffer(YY_CURRENT_BUFFER);
}
