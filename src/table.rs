use crate::args::*;
use crate::expressions::*;
use crate::statements::{Dir, Order};
use crate::traits::*;

pub struct Table {
    name: String,
}

pub fn tb(table_name: &'static str) -> Table {
    Table::new(table_name.to_string())
}

impl Table {
    pub fn new(name: String) -> Self {
        return Table { name: name };
    }
    fn col(&self, name: String) -> Col {
        Col {
            table_name: self.name.clone(),
            column: name,
            alias: None,
            wrapper: None,
        }
    }
}

impl ToSQL for Table {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        (format!("`{}`", self.name), None)
    }
}

#[derive(Clone)]
enum Wrapper {
    Distinct(Option<Box<Wrapper>>),
    Count(Option<Box<Wrapper>>),
    Sum(Option<Box<Wrapper>>),
    Max(Option<Box<Wrapper>>),
    Min(Option<Box<Wrapper>>),
    Avg(Option<Box<Wrapper>>),
    Concat(Option<Box<Wrapper>>),
    Instr((Option<Box<Wrapper>>), String),
    Coalesce(Option<Box<Wrapper>>),
}

impl Wrapper {
    fn to_sql(&self, parent_sql: String) -> String {
        match self {
            Wrapper::Count(Some(sub)) => format!("COUNT({})", sub.to_sql(parent_sql)),
            Wrapper::Count(_) => format!("COUNT({parent_sql})"),
            Wrapper::Sum(Some(sub)) => format!("SUM({})", sub.to_sql(parent_sql)),
            Wrapper::Sum(_) => format!("SUM({parent_sql})"),
            Wrapper::Max(Some(sub)) => format!("MAX({})", sub.to_sql(parent_sql)),
            Wrapper::Max(_) => format!("MAX({parent_sql})"),
            Wrapper::Min(Some(sub)) => format!("MIN({})", sub.to_sql(parent_sql)),
            Wrapper::Min(_) => format!("MIN({parent_sql})"),
            Wrapper::Avg(Some(sub)) => format!("AVG({})", sub.to_sql(parent_sql)),
            Wrapper::Avg(_) => format!("AVG({parent_sql})"),
            Wrapper::Concat(Some(sub)) => format!("GROUP_CONCAT({})", sub.to_sql(parent_sql)),
            Wrapper::Concat(_) => format!("GROUP_CONCAT({parent_sql})"),
            Wrapper::Instr(Some(sub), txt) => format!("INSTR({}, {txt})", sub.to_sql(parent_sql)),
            Wrapper::Instr(_, txt) => format!("INSTR({parent_sql}, {txt})"),
            Wrapper::Coalesce(Some(sub)) => format!("COALESCE({})", sub.to_sql(parent_sql)),
            Wrapper::Coalesce(_) => format!("COALESC({parent_sql})"),
            Wrapper::Distinct(Some(sub)) => format!("DISTINCT {}", sub.to_sql(parent_sql)),
            Wrapper::Distinct(_) => format!("DISTINCT {parent_sql}"),
        }
    }
    fn wrap(self, outer: Wrapper) -> Self {
        match outer {
            Wrapper::Count(_) => Wrapper::Count(Some(Box::new(self))),
            Wrapper::Sum(_) => Wrapper::Sum(Some(Box::new(self))),
            Wrapper::Max(_) => Wrapper::Max(Some(Box::new(self))),
            Wrapper::Min(_) => Wrapper::Min(Some(Box::new(self))),
            Wrapper::Avg(_) => Wrapper::Avg(Some(Box::new(self))),
            Wrapper::Concat(_) => Wrapper::Concat(Some(Box::new(self))),
            Wrapper::Instr(_, txt) => Wrapper::Instr(Some(Box::new(self)), txt),
            Wrapper::Coalesce(_) => Wrapper::Coalesce(Some(Box::new(self))),
            Wrapper::Distinct(_) => Wrapper::Distinct(Some(Box::new(self))),
        }
    }
}

#[derive(Clone)]
pub struct Col {
    table_name: String,
    column: String,
    alias: Option<String>,
    wrapper: Option<Wrapper>,
}

pub fn cl(table: &'static str, col: &'static str) -> Col {
    Col::new(table, col)
}

impl Col {
    pub fn new(table: &'static str, col: &'static str) -> Self {
        Col {
            table_name: table.to_string(),
            column: col.to_string(),
            alias: None,
            wrapper: None,
        }
    }
    pub fn name(&self) -> &str {
        &self.column
    }
    pub fn as_alias(mut self, val: &'static str) -> Self {
        self.alias = Some(val.to_string());
        self
    }
    pub fn eq<T: ToExpTar>(self, exp: T) -> Exp {
        self.make_exp(exp.to_exp_tar(), Op::Eq)
    }
    pub fn neq<T: ToExpTar>(&self, exp: T) -> Exp {
        self.make_exp(exp.to_exp_tar(), Op::Neq)
    }
    pub fn lt<T: ToExpTar>(&self, exp: T) -> Exp {
        self.make_exp(exp.to_exp_tar(), Op::Lt)
    }
    pub fn gt<T: ToExpTar>(&self, exp: T) -> Exp {
        self.make_exp(exp.to_exp_tar(), Op::Gt)
    }
    pub fn like(&self, search: String) -> Exp {
        self.make_exp(search.to_exp_tar(), Op::Like)
    }
    pub fn r#in<T: ToExpTar>(&self, exp: T) -> Exp {
        self.make_exp(exp.to_exp_tar(), Op::In)
    }
    pub fn is_null(&self) -> Exp {
        self.make_exp(ExpTar::Null, Op::Is)
    }
    pub fn is_not_null(&self) -> Exp {
        self.make_exp(ExpTar::Null, Op::IsNot)
    }
    fn make_exp(&self, comp: ExpTar, op: Op) -> Exp {
        Exp::Exp(ExpU::new(op, ExpTar::C(self.clone()), comp))
    }
    pub fn asc(&self) -> Order {
        Order::new(self.clone(), Dir::Asc)
    }
    pub fn desc(&self) -> Order {
        Order::new(self.clone(), Dir::Desc)
    }
    pub fn distinct(self) -> Self {
        self.do_wrapper(Wrapper::Distinct(None))
    }
    pub fn count(self) -> Self {
        self.do_wrapper(Wrapper::Count(None))
    }
    pub fn sum(self) -> Self {
        self.do_wrapper(Wrapper::Sum(None))
    }
    pub fn max(self) -> Self {
        self.do_wrapper(Wrapper::Max(None))
    }
    pub fn min(self) -> Self {
        self.do_wrapper(Wrapper::Min(None))
    }
    pub fn avg(self) -> Self {
        self.do_wrapper(Wrapper::Avg(None))
    }
    pub fn concat(self) -> Self {
        self.do_wrapper(Wrapper::Concat(None))
    }
    pub fn instr(self, search: &'static str) -> Self {
        self.do_wrapper(Wrapper::Instr(None, search.to_string()))
    }
    pub fn coalesce(self) -> Self {
        self.do_wrapper(Wrapper::Coalesce(None))
    }
    fn do_wrapper(mut self, wrapper: Wrapper) -> Self {
        match self.wrapper {
            Some(inner_wrapper) => self.wrapper = Some(inner_wrapper.wrap(wrapper)),
            None => self.wrapper = Some(wrapper),
        }
        self
    }
}

impl ToSQL for Col {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let sql = format!("`{}`.`{}`", self.table_name, self.column);
        let mut sql = match &self.wrapper {
            Some(wrapper) => wrapper.to_sql(sql),
            None => sql,
        };
        if let Some(val) = &self.alias {
            sql.push_str(format!(" AS {val}").as_str())
        }
        (sql, None)
    }
}
