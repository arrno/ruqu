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
        .from("user")
        .select(vec![
            cl("user", "id").distinct().count().as_alias("user_count"), // I think it would be better if we just always wrap whats on the left with what's on the right.
            cl("user", "name"),
        ])
        // .distinct()
        .join(
            tb("comment"),
            On::new(Exp::exp_and(
                cl("comment", "user_id").eq(cl("user", "id")),
                cl("comment", "deleted").is_null(),
            )),
        )
        .r#where(Exp::Set(vec![
            Exp::exp_or(cl("user", "active").eq(true), cl("user", "score").gt(9)),
            cl("comment", "likes").eq(cl("comment", "dislikes")),
        ]))
        .order(cl("user", "join_date"), Dir::Asc)
        .limit(5)
        .to_sql()
        .unwrap();

    println!("\n{query}\n");
    for arg in args {
        println!("{:?}", arg);
    }
}
