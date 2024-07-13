# goqu clone in rust

This project is intended to replicate the features of [this project](https://doug-martin.github.io/goqu/docs/database.html) but in RUST 🦀! ATM the project is in progress. Here is a working example:

```rust
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

TODO
- Limit, GroupBy, Having,
- Select distinct
- ColumnFunctions: 
    - Count(Distinct)
    - Sum, Max, Min, Avg
    - Group_concat(Distinct, Order)
    - Like
    - Instr
    - coalesce
- Union Join
- Update, Insert, Delete