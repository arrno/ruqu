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

pub enum ExpU {
    Exp(Exp),
    And(And),
    Or(Or),
}
impl ToSQL for ExpU {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        match self {
            ExpU::Exp(e) => e.to_sql(),
            ExpU::And(a) => a.to_sql(),
            ExpU::Or(o) => o.to_sql(),
        }
    }
}
pub struct And {
    left: Box<ExpU>,
    right: Box<ExpU>,
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
struct Or {
    left: Box<ExpU>,
    right: Box<ExpU>,
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
    T(MYSQLBuilder),
}

impl<T: ToArg> From<T> for ExpTar {
    fn from(val: T) -> Self {
        ExpTar::A(val.to_arg())
    }
}

impl ToSQL for ExpTar {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        match self {
            ExpTar::A(Arg::Null) => (String::from("NULL"), None),
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
pub struct Exp {
    pub op: Op,
    pub left: ExpTar,
    pub right: ExpTar,
}

impl ToSQL for Exp {
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
