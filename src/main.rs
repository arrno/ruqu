mod args;
mod clauses;
mod expressions;
mod mysql;
mod table;
mod traits;

use args::Arg::*;
use expressions::ExpTar::A;
use mysql::*;
use table::Col;
use traits::*;

fn main() {
    let (query, args) = MYSQLBuilder::query()
        .from(String::from("Table"))
        .select(vec![
            Col::new(String::from("Table"), String::from("Column")),
            Col::new(String::from("Table"), String::from("Column")),
        ])
        .r#where(Col::new(String::from("Table"), String::from("Column")).eq(true.into()))
        .to_sql()
        .unwrap();
}
