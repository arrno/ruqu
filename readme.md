# goqu clone in rust

This project is intended to replicate the features of [this project](https://doug-martin.github.io/goqu/docs/database.html) but in RUST 🦀! ATM the project is in progress. Here are a few working examples:

### Query

```rust
let (query, args) = MYSQLBuilder::query()
    .from("user")
    .select(vec![cl("user", "name"), cl("user", "date")])
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
    .order(cl("user", "date"), Dir::Asc)
    .limit(5)
    .to_sql();
```

produces:

```sql
SELECT DISTINCT `user`.`name`, `user`.`date`
FROM `user`
JOIN `comment` ON ((`comment`.`user_id` = `user`.`id`) AND (`comment`.`deleted` IS NULL))
WHERE (((`user`.`active` = ?) OR (`user`.`score` > ?)) AND (`comment`.`likes` = `comment`.`dislikes`))
ORDER BY `user`.`date` ASC
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

### Update

```rust
let (query, args) = MYSQLBuilder::query()
    .update(tb("user"))
    .set(vec![
        cl("user", "active").eq(true),
        cl("user", "score").eq(100),
    ])
    .r#where(Exp::exp_and(
        cl("user", "name").like("%ally"),
        cl("user", "deleted").is_null(),
    ))
    .to_sql();
```

produces:

```sql
UPDATE `user` 
SET `user`.`active` = ?, `user`.`score` = ?
WHERE ((`user`.`name` LIKE ?) AND (`user`.`deleted` IS NULL))

-- Bool(true)
-- Int(100)
-- Str("%ally")
```

## Features
- Query
    - from
    - select
    - select.distinct
    - tb (table)
    - cl (column)
    - cl.as_alias
    - [RECURSIVE]
        - max
        - min
        - count
        - sum
        - avg
        - instr
        - concat (group_concat)
        - coalesce
        - cl.distinct
    - join
    - left_join
    - right_join
    - union
    - where
    - group_by
    - having
    - order
        - Dir::Asc
        - Dir::Desc
    - limit
- Expressions
    - where
    - On
    - [RECURSIVE]
        - ExpU (simple expression unit)
        - Exp::Set (MULTI AND)
        - Exp::exp_and
        - Exp::exp_or
    - [COLUMN EXP]
        - eq
        - neq
        - gt (greater than)
        - lt (less than)
        - like
        - in
        - is_null
        - is_not_null
- Update
    - update
    - set
    - where
- Insert
    - insert
    - rows
- Delete
    - delete
    - where
- Args
    - Uint
    - Int
    - Bool
    - Str
    - Float
    - Set<Arg>
    - Null