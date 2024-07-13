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
        .select(vec![cl("Table", "Column"), cl("Table", "Column")])
        .join(
            cl("Second", "Blue"),
            On::new(Exp::exp_and(
                cl("Table", "Column").eq(cl("Second", "Blue")),
                cl("Second", "Deleted").is_null(),
            )),
        )
        .r#where(Exp::Set(vec![
            Exp::exp_or(
                cl("Table", "Column").eq(true),
                cl("More Table", "SHWEET").gt(7),
            ),
            cl("Table", "Column").eq(cl("Other", "Val")),
        ]))
        .order(cl("Second", "Blue"), Dir::Asc)
        .to_sql()
        .unwrap();

    println!("\n{query}\n");
    for arg in args {
        println!("{:?}", arg);
    }
}
