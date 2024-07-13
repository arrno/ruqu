# goqu clone in rust

This project is intended to replicate the features of [this project](https://doug-martin.github.io/goqu/docs/database.html) but in RUST 🦀! ATM the project is in progress. Here is a working example:

```rust
    let (query, args) = MYSQLBuilder::query()
        .from("Table")
        .select(vec![Cl("Table", "Column"), Cl("Table", "Column")])
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
```

produces:

```sql
SELECT `Table`.`Column`, `Table`.`Column`
FROM `Table`
JOIN `Second`.`Blue`
        ON ((`Table`.`Column` = `Second`.`Blue`) AND (`Second`.`Deleted` IS NULL))
WHERE (((`Table`.`Column` = ?) OR (`More Table`.`SHWEET` > ?)) AND (`Table`.`Column` = `Other`.`Val`))
ORDER BY `Second`.`Blue` ASC

-- Bool(true)
-- Int(7)
```