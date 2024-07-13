use crate::args::*;
use crate::expressions::*;
use crate::table::*;
use crate::traits::*;

pub struct Limit(i32);
impl Limit {
    pub fn new(by: i32) -> Self {
        Limit(by)
    }
}
impl ToSQL for Limit {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        (format!("LIMIT {}", self.0), None)
    }
}

pub struct GroupBy {
    cols: Vec<Col>,
    having: Option<Box<ExpU>>,
}

impl GroupBy {
    pub fn new(cols: Vec<Col>) -> Self {
        GroupBy {
            cols: cols,
            having: None,
        }
    }
    pub fn extend(&mut self, cols: Vec<Col>) {
        self.cols.extend(cols);
    }
    pub fn having(&mut self, exp: ExpU) {
        self.having = Some(Box::new(exp));
    }
}

impl ToSQL for GroupBy {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let mut col_sql = vec![];
        let mut col_args = vec![];
        self.cols.iter().for_each(|col| match col.to_sql() {
            (sql, Some(args)) => {
                col_sql.push(sql);
                col_args.extend(args);
            }
            (sql, None) => col_sql.push(sql),
        });
        let mut sql = format!("GROUP BY {}", col_sql.join(", "));
        if let Some(having) = &self.having {
            match having.to_sql() {
                (having_sql, Some(having_args)) => {
                    sql.push_str(format!("HAVING {having_sql}").as_str());
                    col_args.extend(having_args);
                }
                (having_sql, None) => sql.push_str(format!("HAVING {having_sql}").as_str()),
            };
        }
        (sql, Some(col_args))
    }
}

pub struct Select {
    cols: Vec<Col>,
    distinct: bool,
}

impl Select {
    pub fn new(cols: Vec<Col>) -> Self {
        Select {
            cols: cols,
            distinct: false,
        }
    }
    pub fn extend(&mut self, cols: Vec<Col>) {
        self.cols.extend(cols);
    }
    pub fn distinct(&mut self) {
        self.distinct = true;
    }
}

impl ToSQL for Select {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let mut query = String::from("SELECT ");
        if self.distinct {
            query.push_str("DISTINCT ");
        }
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
pub enum JoinType {
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
    from: Table,
    join: JoinType,
    on: Option<On>,
}

impl Join {
    pub fn new(from: Table, join: JoinType, on: Option<On>) -> Self {
        Join { from, join, on }
    }
}
impl ToSQL for Join {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let join_str: String = self.join.clone().into();
        let mut args = vec![];
        let (from_sql, _) = self.from.to_sql();
        let mut sql = format!("{join_str} {from_sql}");
        if let Some(on) = &self.on {
            let (exp_sql, exp_args_op) = on.to_sql();
            if let Some(exp_args) = exp_args_op {
                args.extend(exp_args)
            }
            sql.push_str(format!(" {exp_sql}").as_str());
        };
        (sql, Some(args))
    }
}

pub struct On {
    pub exp: Box<Exp>,
}

impl On {
    pub fn new(exp: Exp) -> Self {
        On { exp: Box::new(exp) }
    }
}
impl ToSQL for On {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let (exp_sql, exp_args) = self.exp.to_sql();
        (format!("ON {exp_sql}"), exp_args)
    }
}
pub struct Where {
    pub exp: Box<Exp>,
}

impl Where {
    pub fn new(exp: Exp) -> Self {
        Where { exp: Box::new(exp) }
    }
}
impl ToSQL for Where {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let (exp_sql, exp_args) = self.exp.to_sql();
        (format!("WHERE {exp_sql}"), exp_args)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Order {
    by: Col,
    dir: Dir,
}

impl Order {
    pub fn new(by: Col, dir: Dir) -> Self {
        Order { by, dir }
    }
}
impl ToSQL for Order {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let (col_sql, col_args) = self.by.to_sql();
        (format!("{} {}", col_sql, self.dir.to_sql().0), col_args)
    }
}
