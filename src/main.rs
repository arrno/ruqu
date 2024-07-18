mod args;
mod expressions;
mod mysql;
mod statements;
mod table;
mod traits;

use std::vec;

use args::*;
use expressions::*;
use mysql::*;
use statements::*;
use table::*;
use traits::*;

fn main() {
    let (query, args) = MYSQLBuilder::query()
        .from("user")
        .select(vec![
            cl("user", "id")
                .max()
                .min()
                .count()
                .instr("hello")
                .as_alias("my_val"),
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
        .to_sql();

    println!("\n{query}\n");
    for arg in &args {
        println!("{:?}", arg);
    }

    let (queryy, argss) = MYSQLBuilder::query()
        .insert(tb("user"))
        .rows(
            vec!["name", "age", "active"],
            vec![
                vec![arg("Jake"), arg(23), arg(false)],
                vec![arg("Sally"), arg(42), arg(true)],
                vec![arg("Jasper"), arg(18), arg(true)],
            ],
        )
        .to_sql();

    println!("\n{queryy}\n");
    for arg in &argss {
        println!("{:?}", arg);
    }

    let (queryyy, argsss) = MYSQLBuilder::query()
        .delete(tb("user"))
        .r#where(cl("user", "active").neq(true))
        .to_sql();

    println!("\n{queryyy}\n");
    for arg in argsss {
        println!("{:?}", arg);
    }
}
