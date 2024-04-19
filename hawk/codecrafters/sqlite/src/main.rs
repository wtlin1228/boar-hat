use anyhow::{bail, Context, Result};
use sqlite_starter_rust::{
    sql_parser::{AggregateFunction, Expr, SQLParser, SelectStmt, Stmt, WhereClause},
    SQLiteDB,
};

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    // Parse command and act accordingly
    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            let db = SQLiteDB::new(&args[1])?;
            println!("database page size: {}", db.get_page_size());
            println!("number of tables: {}", db.get_tables()?.len());
        }
        ".tables" => {
            let db = SQLiteDB::new(&args[1])?;
            for name in db.get_tables()?.iter().map(|table| &table.tbl_name) {
                // ref: https://www.sqlite.org/fileformat2.html#internal_schema_objects
                if !name.starts_with("sqlite_") {
                    print!("{} ", name);
                }
            }
        }
        _ => {
            let db = SQLiteDB::new(&args[1])?;
            match SQLParser::parse_stmt(command)? {
                Stmt::CreateTable(_) => unimplemented!(),
                Stmt::Select(SelectStmt {
                    from,
                    result_column,
                    where_clause,
                }) => {
                    let tables = db.get_tables()?;
                    let schema_table = tables
                        .iter()
                        .find(|&x| &x.tbl_name == &from)
                        .context(format!("Invalid table name {}", from))?;
                    let root_page =
                        db.get_page(schema_table.rootpage.context("Table has no root page")?)?;

                    let column_def = &schema_table.column_def;
                    let mut selected_columns: Vec<usize> = vec![];
                    for expr in result_column.iter() {
                        match expr {
                            Expr::Function(AggregateFunction::Count(count)) => match count {
                                Some(_) => unimplemented!(),
                                None => {
                                    println!("{}", root_page.cell_count);
                                    return Ok(());
                                }
                            },
                            Expr::Column(column) => {
                                match column_def.iter().position(|x| x == column) {
                                    Some(column_idx) => selected_columns.push(column_idx),
                                    None => bail!("Selected column {} doesn't exist", column),
                                }
                            }
                        }
                    }

                    let mut rows = root_page.get_rows()?;
                    if let Some(WhereClause { column, value }) = where_clause {
                        let column_idx = match column_def.iter().position(|x| x == &column) {
                            Some(column_idx) => column_idx,
                            None => bail!("Where condition column {} doesn't exist", column),
                        };
                        rows = rows
                            .into_iter()
                            .filter(|row| format!("{}", row.columns[column_idx]) == value)
                            .collect();
                    }

                    for row in rows.iter() {
                        let mut output = vec![];
                        for column_idx in selected_columns.iter() {
                            output.push(format!("{}", row.columns[*column_idx]));
                        }
                        println!("{}", output.join("|"));
                    }
                }
            }
        }
    }

    Ok(())
}
