use crate::args::*;
use crate::expressions::*;
use crate::statements::{Dir, Order};
use crate::traits::*;

pub struct Table {
    pub name: String,
}

impl Table {
    fn new(name: String) -> Self {
        return Table { name: name };
    }
    fn col(&self, name: String) -> Col {
        Col {
            table_name: self.name.clone(),
            column: name,
            alias: None,
        }
    }
}

impl ToSQL for Table {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        (format!("`{}`", self.name), None)
    }
}

#[derive(Clone)]
pub struct Col {
    pub table_name: String,
    pub column: String,
    pub alias: Option<String>,
}

impl Col {
    pub fn new(table: &'static str, col: &'static str) -> Self {
        Col {
            table_name: table.to_string(),
            column: col.to_string(),
            alias: None,
        }
    }
    pub fn name(&self) -> &str {
        &self.column
    }
    pub fn as_alias(mut self, val: String) -> Self {
        self.alias = Some(val);
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
        Exp::Exp(ExpU {
            op: op,
            left: ExpTar::C(self.clone()),
            right: comp,
        })
    }
    pub fn asc(&self) -> Order {
        Order::new(self.clone(), Dir::Asc)
    }
    pub fn desc(&self) -> Order {
        Order::new(self.clone(), Dir::Desc)
    }
}

impl ToSQL for Col {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let mut sql = format!("`{}`.`{}`", self.table_name, self.column);
        if let Some(val) = &self.alias {
            sql.push_str(format!(" AS {val}").as_str())
        }
        (sql, None)
    }
}
