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
            Cl("Table", "Column"),
            Cl("Table", "Column"),
        ])
        .join(
            Cl("Second", "Blue"),
            On::new(Exp::exp_and(
                Cl("Table", "Column").eq(Cl("Second", "Blue")),
                Cl("Second", "Deleted").is_null(),
            )),
        )
        .r#where(Exp::Set(vec![
            Exp::exp_or(
                Cl("Table", "Column").eq(true),
                Cl("More Table", "SHWEET").gt(7),
            ),
            Cl("Table", "Column").eq(Cl("Other", "Val")),
        ]))
        .order(Cl("Second", "Blue"), Dir::Asc)
        .to_sql()
        .unwrap();

    println!("\n{query}\n");
    for arg in args {
        println!("{:?}", arg);
    }
}
