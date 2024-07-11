mod args;
mod clauses;
mod expressions;
mod mysql;
mod table;
mod traits;

use args::Arg::*;
use expressions::{ExpTar::A, ExpU, Or};
use mysql::*;
use table::Col;
use traits::*;

fn main() {
    // let (query, args) = MYSQLBuilder::query()
    //     .from(String::from("Table"))
    //     .select(vec![
    //         Col::new(String::from("Table"), String::from("Column")),
    //         Col::new(String::from("Table"), String::from("Column")),
    //     ])
    //     .r#where(Col::new(String::from("Table"), String::from("Column")).eq(true.into()))
    //     .r#where(Col::new(String::from("MoreTable"), String::from("SHWEET")).gt((7 as isize).into()))
    //     .to_sql()
    //     .unwrap();

    let (query, args) = MYSQLBuilder::query()
        .from(String::from("Table"))
        .select(vec![
            Col::new(String::from("Table"), String::from("Column")),
            Col::new(String::from("Table"), String::from("Column")),
        ])
        .r#where(
            Col::new(String::from("Table"), String::from("Column"))
                .eq(Col::new(String::from("Other"), String::from("Val"))),
        )
        .r#where(ExpU::exp_or(
            Col::new(String::from("Table"), String::from("Column")).eq(true),
            Col::new(String::from("MoreTable"), String::from("SHWEET")).gt((7 as isize)),
        ))
        .r#where(ExpU::Vec(vec![
            Col::new(String::from("Table"), String::from("Column")).eq(true),
            Col::new(String::from("MoreTable"), String::from("SHWEET")).gt((7 as isize)),
            Col::new(String::from("Table"), String::from("Column"))
                .eq(Col::new(String::from("Other"), String::from("Val"))),
        ]))
        .to_sql()
        .unwrap();

    println!("{query}");
    for arg in args {
        println!("{:?}", arg);
    }
}
