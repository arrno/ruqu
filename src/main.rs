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
            Col {
                table_name: String::from("Table"),
                column: String::from("Column"),
            },
            Col {
                table_name: String::from("Table"),
                column: String::from("Column"),
            },
        ])
        .r#where(
            Col {
                table_name: String::from("Table"),
                column: String::from("Column"),
            }
            .eq(A(Bool(true))),
        )
        .to_sql()
        .unwrap();
}
