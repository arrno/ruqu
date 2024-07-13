mod args;
mod expressions;
mod mysql;
mod statements;
mod table;
mod traits;

use expressions::*;
use mysql::*;
use statements::*;
use table::*;
use traits::*;

fn main() {
    let (query, args) = MYSQLBuilder::query()
        .from("Table")
        .select(vec![
            Col::new("Table", "Column"),
            Col::new("Table", "Column"),
        ])
        .join(
            Col::new("Second", "Blue"),
            On::new(Exp::exp_and(
                Col::new("Table", "Column").eq(Col::new("Second", "Blue")),
                Col::new("Second", "Deleted").is_null(),
            )),
        )
        .r#where(Exp::Set(vec![
            Exp::exp_or(
                Col::new("Table", "Column").eq(true),
                Col::new("More Table", "SHWEET").gt(7),
            ),
            Col::new("Table", "Column").eq(Col::new("Other", "Val")),
        ]))
        .order(Col::new("Second", "Blue"), Dir::Asc)
        .to_sql()
        .unwrap();

    println!("\n{query}\n");
    for arg in args {
        println!("{:?}", arg);
    }
}
