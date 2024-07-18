# goqu clone in rust

This project is intended to replicate the features of [this project](https://doug-martin.github.io/goqu/docs/database.html) but in RUST 🦀! ATM the project is in progress. Here are a few working examples:

### Query

```rust
let (query, args) = MYSQLBuilder::query()
    .from("user")
    .select(vec![cl("user", "id"), cl("user", "name")])
    .distinct()
    .join(
        tb("comment"),
        On::new(Exp::exp_and(
            cl("comment", "user_id").eq(cl("user", "id")),
            cl("comment", "deleted").is_null(),
        )),
    )
    .r#where(Exp::Set(vec![
        Exp::exp_or(
            cl("user", "active").eq(true),
            cl("user", "score").gt(9),
        ),
        cl("comment", "likes").eq(cl("comment", "dislikes")),
    ]))
    .order(cl("user", "join_date"), Dir::Asc)
    .limit(5)
    .to_sql();
```

produces:

```sql
SELECT DISTINCT `user`.`id`, `user`.`name`
FROM `user`
JOIN `comment` ON ((`comment`.`user_id` = `user`.`id`) AND (`comment`.`deleted` IS NULL))
WHERE (((`user`.`active` = ?) OR (`user`.`score` > ?)) AND (`comment`.`likes` = `comment`.`dislikes`))
ORDER BY `user`.`join_date` ASC
LIMIT 5

-- Bool(true)
-- Int(7)
```

### Insert

```rust
let (query, args) = MYSQLBuilder::query()
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
```

produces:

```sql
INSERT INTO `user` (`name`, `age`, `active`) 
VALUES
        (?, ?, ?),
        (?, ?, ?),
        (?, ?, ?)

-- Str("Jake")
-- Int(23)
-- Bool(false)
-- Str("Sally")
-- Int(42)
-- Bool(true)
-- ...
```

### Delete

```rust
let (query, args) = MYSQLBuilder::query()
    .delete(tb("user"))
    .r#where(cl("user", "active").neq(true))
    .to_sql();
```

produces:

```sql
DELETE FROM `user` WHERE (`user`.`active` != ?)

-- Bool(true)
```

## Todo

- ~~Limit, GroupBy, Having~~
- ~~Select distinct~~
- ~~Union Join~~
- ~~ColumnFunctions~~: 
    - ~~Count(Distinct)~~
    - ~~Sum, Max, Min, Avg~~
    - ~~GroupConcat(Distinct, Order)~~
    - ~~Instr~~
    - ~~Coalesce~~
- Update, Insert, Delete