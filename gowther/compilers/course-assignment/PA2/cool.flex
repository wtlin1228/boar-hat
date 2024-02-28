/*
 *  The scanner definition for COOL.
 */

/*
 *  Stuff enclosed in %{ %} in the first section is copied verbatim to the
 *  output, so headers and global definitions are placed here to be visible
 * to the code in the file.  Don't remove anything that was here initially
 */

%option noyywrap

%{
#include <cool-parse.h>
#include <stringtab.h>
#include <utilities.h>

/* The compiler assumes these identifiers. */
#define yylval cool_yylval
#define yylex  cool_yylex

/* Max size of string constants */
#define MAX_STR_CONST 1025
#define YY_NO_UNPUT   /* keep g++ happy */

extern FILE *fin; /* we read from this file */

/* define YY_INPUT so we read from the FILE fin:
 * This change makes it possible to use this scanner in
 * the Cool compiler.
 */
#undef YY_INPUT
#define YY_INPUT(buf,result,max_size) \
	if ( (result = fread( (char*)buf, sizeof(char), max_size, fin)) < 0) \
		YY_FATAL_ERROR( "read() in flex scanner failed");

char string_buf[MAX_STR_CONST]; /* to assemble string constants */
char *string_buf_ptr;

extern int curr_lineno;
extern int verbose_flag;

extern YYSTYPE cool_yylval;

/*
 *  Add Your own definitions here
 */

int nested_comment_depth = 0;
int string_const_length = 0;
char get_string_special_char(char c) {
  if (c == 'n') return '\n';
  else if (c == 't') return '\t';
  else if (c == 'b') return '\b';
  else if (c == 'f') return '\f';
  else return c;
}

%}

%x SINGLE_LINE_COMMENT
%x NESTED_COMMENT
%x STRING
%x ESCAPE
%x TERMINATE

/*
 * Define names for regular expressions here.
 */

/*
 * 10.1 Integers, Identifiers, and Special Notation
 */

DIGIT                   [0-9]
LOWERCASE_LETTER        [a-z]
UPPERCASE_LETTER        [A-Z]
LETTER                  ({LOWERCASE_LETTER}|{UPPERCASE_LETTER})
INTEGER                 {DIGIT}+
TYPE_IDENTIFIER         ("SELF_TYPE"|{UPPERCASE_LETTER}({LETTER}|{DIGIT}|"_")*)
OBJECT_IDENTIFIER       ("self"|{LOWERCASE_LETTER}({LETTER}|{DIGIT}|"_")*)
SINGLE_CHAR_OPERATOR    ("+"|"/"|"-"|"*"|"="|"<"|"."|"~"|","|";"|":"|"("|")"|"@"|"{"|"}")
ASSIGN                  "<-"
DARROW                  "=>"
LE                      "<="

/*
 * 10.2 Strings
 */

STRING_START    "\""
STRING_END      "\""

/*
 * 10.3 Comments
 */

ONE_LINE_COMMENT_START         "--"
MULTIPLE_LINE_COMMENT_START    "\(\*"
MULTIPLE_LINE_COMMENT_END      "\*\)"

/*
 * 10.4 Keywords are case insensitive, expect for the constants true and false
 */

CLASS       (?i:class)
ELSE        (?i:else)
FI          (?i:fi)
IF          (?i:if)
IN          (?i:in)
INHERITS    (?i:inherits)
ISVOID      (?i:isvoid)
LET         (?i:let)
LOOP        (?i:loop)
POOL        (?i:pool)
THEN        (?i:then)
WHILE       (?i:while)
CASE        (?i:case)
ESAC        (?i:esac)
NEW         (?i:new)
OF          (?i:of)
NOT         (?i:not)

BOOL_CONST_TRUE     (t)(?i:rue)
BOOL_CONST_FALSE    (f)(?i:alse)

/*
 * 10.5 White Space
 */

WHITE_SPACE    (" "|\f|\r|\t|\v)

%%

 /*
  *  The keywords.
  */

{CLASS} { return CLASS; }
{ELSE} { return ELSE; }
{FI} { return FI; }
{IF} { return IF; }
{IN} { return IN; }
{INHERITS} { return INHERITS; }
{ISVOID} { return ISVOID; }
{LET} { return LET; }
{LOOP} { return LOOP; }
{POOL} { return POOL; }
{THEN} { return THEN; }
{WHILE} { return WHILE; }
{CASE} { return CASE; }
{ESAC} { return ESAC; }
{NEW} { return NEW; }
{OF} { return OF; }
{NOT} { return NOT; }

 /*
  *  The multiple-character operators.
  */

{ASSIGN} { return ASSIGN; }
{DARROW} { return DARROW; }
{LE} { return LE; }

{BOOL_CONST_TRUE} {
  cool_yylval.boolean = true;
  return BOOL_CONST;
}
{BOOL_CONST_FALSE} {
  cool_yylval.boolean = false;
  return BOOL_CONST;
}

 /*
  *  The single-character operators.
  */

{SINGLE_CHAR_OPERATOR} { return yytext[0]; }

{INTEGER} {
  cool_yylval.symbol = inttable.add_string(yytext);
  return INT_CONST;
}

{TYPE_IDENTIFIER} {
  cool_yylval.symbol = idtable.add_string(yytext);
  return TYPEID;
}

{OBJECT_IDENTIFIER} {
  cool_yylval.symbol = idtable.add_string(yytext);
  return OBJECTID;
}

 /*
  *  Single line comments
  */

{ONE_LINE_COMMENT_START} { BEGIN(SINGLE_LINE_COMMENT); };
<SINGLE_LINE_COMMENT>\n {
  curr_lineno++;
  BEGIN(INITIAL);
}
<SINGLE_LINE_COMMENT>. {}

 /*
  *  Nested comments
  */

{MULTIPLE_LINE_COMMENT_START} { 
  nested_comment_depth++;
  BEGIN(NESTED_COMMENT); 
}
<NESTED_COMMENT>{MULTIPLE_LINE_COMMENT_START} {
  nested_comment_depth++;
}
<NESTED_COMMENT>{MULTIPLE_LINE_COMMENT_END} {
  nested_comment_depth--;
  if (nested_comment_depth < 0) {
    cool_yylval.error_msg = "Unmatched *)";
    return ERROR;
  } else if (nested_comment_depth == 0) {
    BEGIN(INITIAL);
  }
}
<NESTED_COMMENT><<EOF>> {
  cool_yylval.error_msg = "EOF in comment";
  BEGIN(TERMINATE);
  return ERROR;
}
<NESTED_COMMENT>\n { curr_lineno++; }
<NESTED_COMMENT>. {}

{MULTIPLE_LINE_COMMENT_END} {
  cool_yylval.error_msg = "Unmatched *)";
  return ERROR;
}

 /*
  *  String constants (C syntax)
  *  Escape sequence \c is accepted for all characters c. Except for 
  *  \n \t \b \f, the result is c.
  *
  */

{STRING_START} { BEGIN(STRING); }
<STRING>{STRING_END} {
  string_buf_ptr = (char*) &string_buf;
  cool_yylval.symbol = stringtable.add_string(string_buf_ptr, string_const_length);
  string_const_length = 0;
  BEGIN(INITIAL);
  return STR_CONST;
} 
<STRING>\n {
  curr_lineno++;
  cool_yylval.error_msg = "Unterminated string constant";
  string_const_length = 0;
  BEGIN(INITIAL);
  return ERROR;
}
<STRING><<EOF>> {
  cool_yylval.error_msg = "EOF in string constant";
  BEGIN(TERMINATE);
  return ERROR;
}
<STRING>"\\"[^\0] {
  if (string_const_length + 1 < MAX_STR_CONST ) {
    string_buf[string_const_length++] = get_string_special_char(yytext[1]);
  } else {
    cool_yylval.error_msg = "String constant too long";
    string_const_length = 0;
    BEGIN(ESCAPE);
    return ERROR; 
  }
}
<STRING>\0 {
  cool_yylval.error_msg = "String contains escaped null character.";
  BEGIN(ESCAPE);
  return ERROR;
}
<STRING>. {
  if (string_const_length + 1 < MAX_STR_CONST ) {
    string_buf[string_const_length++] = yytext[0];
  } else {
    cool_yylval.error_msg = "String constant too long";
    string_const_length = 0;
    BEGIN(ESCAPE);
    return ERROR; 
  }
}

<ESCAPE>(\n|{STRING_END}) { BEGIN(INITIAL); }
<ESCAPE>[^\n] {}

<TERMINATE>. { yyterminate(); }

\n { curr_lineno++; }
{WHITE_SPACE} {}

. {
  cool_yylval.error_msg = &yytext[0];
  return ERROR;
}

%%

