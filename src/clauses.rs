use crate::args::*;
use crate::expressions::*;
use crate::table::*;
use crate::traits::*;

pub struct Select {
    pub cols: Vec<Col>,
}
impl ToSQL for Select {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let mut query = String::from("SELECT ");
        let mut selects = Vec::new();
        let mut args = Vec::new();
        self.cols.iter().for_each(|col| {
            let (col_query, col_args_op) = col.to_sql();
            selects.push(col_query);
            if let Some(col_args) = col_args_op {
                args.extend(col_args);
            }
        });
        query.push_str(selects.join(", ").as_str());
        (query, Some(args))
    }
}

#[derive(Clone)]
enum Join {
    Inner,
    Left,
    Right,
    Union,
}
impl From<Join> for String {
    fn from(join: Join) -> Self {
        match join {
            Join::Inner => String::from("JOIN"),
            Join::Left => String::from("LEFT JOIN"),
            Join::Right => String::from("RIGHT JOIN"),
            _ => String::from("JOIN"),
        }
    }
}
pub struct JoinClause<'a> {
    from: &'a Col,
    join: Join,
    on: Option<On<'a>>,
}
impl ToSQL for JoinClause<'_> {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let join_str: String = self.join.clone().into();
        let mut args = vec![];
        let (from_sql, from_args) = self.from.to_sql();
        let mut sql = format!("{join_str} {from_sql}");
        if let Some(on) = &self.on {
            let (exp_sql, exp_args_op) = on.r#where.to_sql();
            if let Some(exp_args) = exp_args_op {
                args.extend(exp_args)
            }
            sql.push_str(format!(" ON ({exp_sql})").as_str());
        };
        (sql, Some(args))
    }
}

pub struct On<'a> {
    r#where: Where<'a>,
}
impl ToSQL for On<'_> {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let (exp_sql, exp_args) = self.r#where.exp.to_sql();
        (format!("ON ({exp_sql})"), exp_args)
    }
}
pub struct Where<'a> {
    target: &'a Col,
    exp: ExpU<'a>,
}

impl ToSQL for Where<'_> {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let (exp_sql, exp_args) = self.exp.to_sql();
        (format!("WHERE ({exp_sql})"), exp_args)
    }
}

// TODO
// struct GroupBy {}

enum Order {
    Asc,
    Desc,
}
impl ToSQL for Order {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        match self {
            Order::Asc => (String::from("ASC"), None),
            Order::Desc => (String::from("DESC"), None),
        }
    }
}
pub struct OrderClause<'a> {
    by: Vec<&'a Col>,
    dir: Order,
}
impl ToSQL for OrderClause<'_> {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let mut col_sql = Vec::new();
        let mut args = Vec::new();
        self.by.iter().for_each(|col| {
            let (csql, cargs) = col.to_sql();
            col_sql.push(csql);
            if let Some(v) = cargs {
                args.extend(v);
            }
        });
        (
            format!(
                "ORDER BY {} {}",
                col_sql.join(" ").as_str(),
                self.dir.to_sql().0
            ),
            Some(args),
        )
    }
}
