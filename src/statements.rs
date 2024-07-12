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
enum JoinType {
    Inner,
    Left,
    Right,
    Union,
}
impl From<JoinType> for String {
    fn from(join: JoinType) -> Self {
        match join {
            JoinType::Inner => String::from("JOIN"),
            JoinType::Left => String::from("LEFT JOIN"),
            JoinType::Right => String::from("RIGHT JOIN"),
            _ => String::from("JOIN"),
        }
    }
}
pub struct Join {
    from: Col,
    join: JoinType,
    on: Option<On>,
}
impl ToSQL for Join {
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

pub struct On {
    r#where: Where,
}
impl ToSQL for On {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let (exp_sql, exp_args) = self.r#where.exp.to_sql();
        (format!("ON ({exp_sql})"), exp_args)
    }
}
pub struct Where {
    pub exp: Box<Exp>,
}

impl ToSQL for Where {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let (exp_sql, exp_args) = self.exp.to_sql();
        (format!("WHERE {exp_sql}"), exp_args)
    }
}

// TODO
// struct GroupBy {}

pub enum Dir {
    Asc,
    Desc,
}
impl ToSQL for Dir {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        match self {
            Dir::Asc => (String::from("ASC"), None),
            Dir::Desc => (String::from("DESC"), None),
        }
    }
}
pub struct Order {
    by: Col,
    dir: Dir,
}
impl Order {
    pub fn new(by: Col, dir: Dir) -> Self {
        Order {
            by,
            dir,
        }
    }
}
impl ToSQL for Order {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let (col_sql, col_args) = self.by.to_sql();
        (format!("{} {}", col_sql, self.dir.to_sql().0), col_args)
    }
}
