use crate::args::*;
use crate::mysql::*;
use crate::table::*;
use crate::traits::*;

pub enum Op {
    Eq,
    Neq,
    Lt,
    Gt,
    In,
    Is,
    IsNot,
}
impl ToSQL for Op {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        match self {
            Op::Eq => (String::from("="), None),
            Op::Neq => (String::from("!="), None),
            Op::Lt => (String::from("<"), None),
            Op::Gt => (String::from(">"), None),
            Op::In => (String::from("IN"), None),
            Op::Is => (String::from("IS"), None),
            Op::IsNot => (String::from("IS NOT"), None),
        }
    }
}

pub enum Exp {
    Exp(ExpU),
    And(And),
    Set(Vec<Exp>),
    Or(Or),
}
impl ToSQL for Exp {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        match self {
            Exp::Exp(e) => e.to_sql(),
            Exp::And(a) => a.to_sql(),
            Exp::Or(o) => o.to_sql(),
            Exp::Set(v) => {
                let mut sql = vec![];
                let mut args = vec![];
                v.iter().for_each(|e| {
                    let (s, a) = e.to_sql();
                    sql.push(s);
                    if let Some(v) = a {
                        args.extend(v);
                    }
                });
                (format!("({})", sql.join(" AND ")), Some(args))
            }
        }
    }
}
impl Exp {
    pub fn exp_and(left: Exp, right: Exp) -> Self {
        Exp::And(And {
            left: Box::new(left),
            right: Box::new(right),
        })
    }
    pub fn exp_or(left: Exp, right: Exp) -> Self {
        Exp::Or(Or {
            left: Box::new(left),
            right: Box::new(right),
        })
    }
    pub fn and(self, exp: Exp) -> Self {
        Exp::And(And {
            left: Box::new(self),
            right: Box::new(exp),
        })
    }
    pub fn or(self, exp: Exp) -> Self {
        Exp::Or(Or {
            left: Box::new(self),
            right: Box::new(exp),
        })
    }
}

pub struct And {
    left: Box<Exp>,
    right: Box<Exp>,
}
impl ToSQL for And {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let mut args = vec![];
        let (left_exp, left_args) = self.left.to_sql();
        let (right_exp, right_args) = self.right.to_sql();
        if let Some(a) = left_args {
            args.extend(a);
        }
        if let Some(a) = right_args {
            args.extend(a);
        }
        (format!("({left_exp} AND {right_exp})"), Some(args))
    }
}
pub struct Or {
    left: Box<Exp>,
    right: Box<Exp>,
}
impl ToSQL for Or {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let mut args = vec![];
        let (left_exp, left_args) = self.left.to_sql();
        let (right_exp, right_args) = self.right.to_sql();
        if let Some(a) = left_args {
            args.extend(a);
        }
        if let Some(a) = right_args {
            args.extend(a);
        }
        (format!("({left_exp} OR {right_exp})"), Some(args))
    }
}

pub enum ExpTar {
    A(Arg),
    C(Col),
    Null,
    T(MYSQLBuilder),
}
pub trait ToExpTar {
    fn to_exp_tar(self) -> ExpTar;
}
impl<T: ToArg> From<T> for ExpTar {
    fn from(val: T) -> Self {
        ExpTar::A(val.to_arg())
    }
}
impl From<Col> for ExpTar {
    fn from(col: Col) -> Self {
        ExpTar::C(col)
    }
}
impl<T: ToArg> ToExpTar for T {
    fn to_exp_tar(self) -> ExpTar {
        ExpTar::A(self.to_arg())
    }
}
impl ToExpTar for Col {
    fn to_exp_tar(self) -> ExpTar {
        ExpTar::C(self)
    }
}

impl ToSQL for ExpTar {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        match self {
            ExpTar::Null => (String::from("NULL"), None),
            ExpTar::A(Arg::Set(arg_set)) => {
                let arg_string: Vec<String> = (0..arg_set.len())
                    .into_iter()
                    .map(|_| String::from("?"))
                    .collect();
                (
                    format!("({})", arg_string.join(", ")),
                    Some(arg_set.iter().map(|arg| arg.clone()).collect()),
                )
            }
            ExpTar::A(arg) => (String::from("?"), Some(vec![arg.clone()])),
            ExpTar::C(col) => (col.to_sql().0, None),
            ExpTar::T(sub_query_builder) => {
                let (sub_query, sub_args) = sub_query_builder.try_to_sql().unwrap();
                (format!("({sub_query})"), Some(sub_args))
            }
        }
    }
}
pub struct ExpU {
    op: Op,
    left: ExpTar,
    right: ExpTar,
}

impl ExpU {
    pub fn new(op: Op, left: ExpTar, right: ExpTar) -> Self {
        ExpU { op, left, right }
    }
}

impl ToSQL for ExpU {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let mut args = vec![];
        let (left, arg) = self.left.to_sql();
        if let Some(v) = arg {
            args.push(v[0].clone())
        }
        let (right, arg) = self.right.to_sql();
        if let Some(v) = arg {
            args.push(v[0].clone())
        }
        let (op_sql, _) = self.op.to_sql();
        (format!("({left} {op_sql} {right})"), Some(args))
    }
}
