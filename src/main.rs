use std::fs;

use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use structopt::StructOpt;

use squirrel::{Options, Format};

#[derive(Debug, StructOpt)]
#[structopt(name = "squirrel", about = "A ~~squirrel~~ sql formatter.")]
struct Opt {
    #[structopt(name = "FILE")]
    file_name: String,
}

fn main() {
    let opt = Opt::from_args();

    let dialect = PostgreSqlDialect {};
    let sql = fs::read_to_string(&opt.file_name).expect("Unable to read file");

    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    let options = Options::default();
    let formatted = ast.format(&options, 0);
    println!("{}", formatted);
}
