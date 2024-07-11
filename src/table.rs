use crate::args::*;
use crate::clauses::Where;
use crate::expressions::*;
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
}

impl Col {
    pub fn new(table: String, col: String) -> Self {
        Col {
            table_name: table,
            column: col,
        }
    }
    pub fn name(&self) -> &str {
        &self.column
    }
    pub fn eq(self, comp: ExpTar) -> ExpU {
        self.make_exp(comp, Op::Eq)
    }
    pub fn neq(&self, comp: ExpTar) -> ExpU {
        self.make_exp(comp, Op::Neq)
    }
    pub fn lt(&self, comp: ExpTar) -> ExpU {
        self.make_exp(comp, Op::Lt)
    }
    pub fn gt(&self, comp: ExpTar) -> ExpU {
        self.make_exp(comp, Op::Gt)
    }
    fn make_exp(&self, comp: ExpTar, op: Op) -> ExpU {
        ExpU::Exp(Exp {
            op: op,
            left: ExpTar::C(self.clone()),
            right: comp,
        })
    }
}

impl ToSQL for Col {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        (format!("`{}`.`{}`", self.table_name, self.column), None)
    }
}
