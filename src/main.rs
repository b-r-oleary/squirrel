use std::fs;

use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

use squirrel::{Options, Format};

fn main() {
    let dialect = PostgreSqlDialect {};
    let sql = fs::read_to_string("./sql/example_1.sql").expect("Unable to read file");

    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    let options = Options::default();
    let formatted = ast.format(&options, 0);
    println!("{}", formatted);
}
