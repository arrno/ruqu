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
    Distinct,
    Count(Option<Box<Wrapper>>),
    Sum,
    Max,
    Min,
    Avg,
    Concat(Option<Box<Wrapper>>),
    Instr(String),
    Coalesce,
}

impl Wrapper {
    fn to_sql(&self, parent_sql: String) -> String {
        match self {
            Wrapper::Count(Some(sub)) => format!("COUNT({})", sub.to_sql(parent_sql)),
            Wrapper::Count(_) => format!("COUNT({parent_sql})"),
            Wrapper::Sum => format!("SUM({parent_sql})"),
            Wrapper::Max => format!("MAX({parent_sql})"),
            Wrapper::Min => format!("MIN({parent_sql})"),
            Wrapper::Avg => format!("AVG({parent_sql})"),
            Wrapper::Concat(Some(sub)) => format!("GROUP_CONCAT({})", sub.to_sql(parent_sql)),
            Wrapper::Concat(_) => format!("GROUP_CONCAT({parent_sql})"),
            Wrapper::Instr(sub) => format!("INSTR({parent_sql}, {sub})"),
            Wrapper::Coalesce => format!("COALESCE({parent_sql})"),
            Wrapper::Distinct => format!("DISTINCT {parent_sql}"),
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
        self.do_wrapper(Wrapper::Distinct)
    }
    pub fn count(self) -> Self {
        self.do_wrapper(Wrapper::Count(None))
    }
    pub fn sum(self) -> Self {
        self.do_wrapper(Wrapper::Sum)
    }
    pub fn max(self) -> Self {
        self.do_wrapper(Wrapper::Max)
    }
    pub fn min(self) -> Self {
        self.do_wrapper(Wrapper::Min)
    }
    pub fn avg(self) -> Self {
        self.do_wrapper(Wrapper::Avg)
    }
    pub fn concat(self) -> Self {
        self.do_wrapper(Wrapper::Concat(None))
    }
    pub fn instr(self, search: String) -> Self {
        self.do_wrapper(Wrapper::Instr(search))
    }
    pub fn coalesce(self) -> Self {
        self.do_wrapper(Wrapper::Coalesce)
    }
    fn do_wrapper(mut self, wrapper: Wrapper) -> Self {
        match self.wrapper {
            Some(Wrapper::Count(_)) => {
                if let Wrapper::Distinct = wrapper {
                    self.wrapper = Some(Wrapper::Count(Some(Box::new(Wrapper::Distinct))));
                } else {
                    self.wrapper = Some(wrapper);
                }
                self
            }
            Some(Wrapper::Concat(_)) => {
                if let Wrapper::Distinct = wrapper {
                    self.wrapper = Some(Wrapper::Concat(Some(Box::new(Wrapper::Distinct))));
                } else {
                    self.wrapper = Some(wrapper);
                }
                self
            }
            Some(Wrapper::Distinct) => {
                match wrapper {
                    Wrapper::Concat(_) => {
                        self.wrapper = Some(Wrapper::Concat(Some(Box::new(Wrapper::Distinct))))
                    }
                    Wrapper::Count(_) => {
                        self.wrapper = Some(Wrapper::Count(Some(Box::new(Wrapper::Distinct))))
                    }
                    _ => self.wrapper = Some(wrapper),
                };
                self
            }
            _ => {
                self.wrapper = Some(wrapper);
                self
            }
        }
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
