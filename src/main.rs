mod args;
mod clauses;
mod expressions;
mod mysql;
mod table;
mod traits;

use expressions::*;
use mysql::*;
use table::*;
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
        .from("Table")
        .select(vec![
            Col::new("Table", "Column"),
            Col::new("Table", "Column"),
        ])
        .r#where(Col::new("Table", "Column").eq(Col::new("Other", "Val")))
        .r#where(Exp::exp_or(
            Col::new("Table", "Column").eq(true),
            Col::new("More Table", "SHWEET").gt(7),
        ))
        .r#where(Exp::Set(vec![
            Col::new("Table", "Column").eq(true),
            Col::new("More Table", "SHWEET").gt(7),
            Col::new("Table", "Column").eq(Col::new("Other", "Val")),
        ]))
        .to_sql()
        .unwrap();

    println!("{query}");
    for arg in args {
        println!("{:?}", arg);
    }
}
