// This SQL Parser only works for this little project since I don't need a comprehensive parser.
// SQLite's SQL language syntax -> https://www.sqlite.org/lang.html
// SQLite's SQL language parser (Lemon) -> https://sqlite.org/lemon.html

extern crate peg;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    CreateTable(CreateTableStmt),
    Select(SelectStmt),
}

#[derive(Debug, PartialEq)]
pub struct CreateTableStmt {
    pub table_name: String,
    pub column_def: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct SelectStmt {
    pub from: String,
    pub result_column: Vec<Expr>,
    pub where_clause: Option<WhereClause>,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Function(AggregateFunction),
    Column(String),
}

// https://www.sqlite.org/lang_aggfunc.html
#[derive(Debug, PartialEq)]
pub enum AggregateFunction {
    // count(X): The count(X) function returns a count of the number of times that X is not NULL in a group.
    // count(*): The count(*) function (with no arguments) returns the total number of rows in the group.
    Count(Option<usize>),
}

#[derive(Debug, PartialEq)]
pub struct WhereClause {
    pub column: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct SQLParser {}

impl SQLParser {
    pub fn parse_stmt(sql: &str) -> anyhow::Result<Stmt> {
        let stmt = sql::sql_stmt(sql)?;
        Ok(stmt)
    }

    pub fn parse_create_table_stmt(sql: &str) -> anyhow::Result<CreateTableStmt> {
        let stmt = sql::create_table_stmt(sql)?;
        Ok(stmt)
    }
}

peg::parser! {
    grammar sql() for str {
        pub rule sql_stmt() -> Stmt
            = stmt:create_table_stmt()
                {
                    Stmt::CreateTable(stmt)
                }
            / stmt:select_stmt()
                {
                    Stmt::Select(stmt)
                }

        // https://www.sqlite.org/lang_createtable.html
        pub rule create_table_stmt() -> CreateTableStmt
            = "CREATE" _ "TABLE" _ "\""? table_name:ident() "\""? _ "(" _ column_def:column_def_list() _ ")"
                {
                    CreateTableStmt { table_name, column_def }
                }

        rule column_def_list() -> Vec<String>
            = list:(column_def() ++ (_ "," _))
                {
                    list
                }

        rule column_def() -> String
            = id:ident() (_ ident())*
                {
                    id.to_string()
                }

        // https://www.sqlite.org/lang_select.html
        pub rule select_stmt() -> SelectStmt
            = ("SELECT" / "select") _ result_column:result_column_list() _ ("FROM" / "from") _ from:ident() _ where_clause:where_clause()?
                {
                    SelectStmt { from, result_column, where_clause }
                }

        rule result_column_list() -> Vec<Expr>
            = list:(result_column() ++ (_ "," _))
                {
                    list
                }

        rule result_column() -> Expr
            = ("COUNT(*)" / "count(*)")
                {
                    Expr::Function(AggregateFunction::Count(None))
                }
            / ("COUNT" / "count") "(" n:number() ")"
                {
                    Expr::Function(AggregateFunction::Count(Some(n)))
                }
            / column:ident()
                {
                    Expr::Column(column.to_string())
                }

        rule where_clause() -> WhereClause
            = ("WHERE" / "where") _ column:ident() _ "=" _ "'" value:$([^'\'']*) "'"
                {
                    WhereClause { column, value: value.to_string() }
                }

        rule ident() -> String
            = s:$(['a'..='z' | 'A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*)
                {
                    s.to_string()
                }

        rule number() -> usize
            = n:$(['0'..='9']+)
                {?
                    n.parse().or(Err("usize"))
                }

        rule _
            = [' ' | '\t' | '\n' ]*


    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_statement() {
        assert_eq!(
            SQLParser::parse_stmt(
                r#"
                CREATE TABLE apples 
                ( 
                    id integer primary key autoincrement,
                    name text,
                    color text
                )
                "#
                .trim()
            )
            .unwrap(),
            Stmt::CreateTable(CreateTableStmt {
                table_name: String::from("apples"),
                column_def: vec![
                    String::from("id"),
                    String::from("name"),
                    String::from("color"),
                ],
            })
        );

        assert_eq!(
            SQLParser::parse_stmt(
                r#"
                SELECT name, color FROM apples
                "#
                .trim()
            )
            .unwrap(),
            Stmt::Select(SelectStmt {
                from: String::from("apples"),
                result_column: vec![
                    Expr::Column(String::from("name")),
                    Expr::Column(String::from("color"))
                ],
                where_clause: None
            })
        );
    }

    #[test]
    fn test_parse_create_table_stmt() {
        assert_eq!(
            sql::create_table_stmt(
                r#"
                CREATE TABLE apples 
                ( 
                    id integer primary key autoincrement,
                    name text,
                    color text
                )
                "#
                .trim()
            ),
            Ok(CreateTableStmt {
                table_name: String::from("apples"),
                column_def: vec![
                    String::from("id"),
                    String::from("name"),
                    String::from("color"),
                ],
            })
        );
    }

    #[test]
    fn test_parse_select_stmt() {
        assert_eq!(
            sql::select_stmt(
                r#"
                SELECT count(*) FROM apples
                "#
                .trim()
            ),
            Ok(SelectStmt {
                from: String::from("apples"),
                result_column: vec![Expr::Function(AggregateFunction::Count(None))],
                where_clause: None
            })
        );

        assert_eq!(
            sql::select_stmt(
                r#"
                SELECT count(32) FROM apples
                "#
                .trim()
            ),
            Ok(SelectStmt {
                from: String::from("apples"),
                result_column: vec![Expr::Function(AggregateFunction::Count(Some(32)))],
                where_clause: None
            })
        );

        assert_eq!(
            sql::select_stmt(
                r#"
                SELECT name FROM apples
                "#
                .trim()
            ),
            Ok(SelectStmt {
                from: String::from("apples"),
                result_column: vec![Expr::Column(String::from("name"))],
                where_clause: None
            })
        );

        assert_eq!(
            sql::select_stmt(
                r#"
                SELECT name, color FROM apples
                "#
                .trim()
            ),
            Ok(SelectStmt {
                from: String::from("apples"),
                result_column: vec![
                    Expr::Column(String::from("name")),
                    Expr::Column(String::from("color"))
                ],
                where_clause: None
            })
        );

        assert_eq!(
            sql::select_stmt(
                r#"
                SELECT name, color FROM apples WHERE color = 'Yellow'
                "#
                .trim()
            ),
            Ok(SelectStmt {
                from: String::from("apples"),
                result_column: vec![
                    Expr::Column(String::from("name")),
                    Expr::Column(String::from("color"))
                ],
                where_clause: Some(WhereClause {
                    column: String::from("color"),
                    value: String::from("Yellow")
                })
            })
        );
    }

    #[test]
    fn test_parse_stage8_sql() {
        assert_eq!(
            sql::create_table_stmt(
                r#"
                CREATE TABLE "superheroes" 
                (
                    id integer primary key autoincrement, 
                    name text not null, 
                    eye_color text, 
                    hair_color text, 
                    appearance_count integer, 
                    first_appearance text, 
                    first_appearance_year text
                )
                "#
                .trim()
            ),
            Ok(CreateTableStmt {
                table_name: String::from("superheroes"),
                column_def: vec![
                    String::from("id"),
                    String::from("name"),
                    String::from("eye_color"),
                    String::from("hair_color"),
                    String::from("appearance_count"),
                    String::from("first_appearance"),
                    String::from("first_appearance_year"),
                ],
            })
        );

        assert_eq!(
            sql::select_stmt(
                r#"
                SELECT id, name FROM superheroes WHERE eye_color = 'Pink Eyes'
                "#
                .trim()
            ),
            Ok(SelectStmt {
                from: String::from("superheroes"),
                result_column: vec![
                    Expr::Column(String::from("id")),
                    Expr::Column(String::from("name"))
                ],
                where_clause: Some(WhereClause {
                    column: String::from("eye_color"),
                    value: String::from("Pink Eyes")
                })
            })
        );
    }
}
