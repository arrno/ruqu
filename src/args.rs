use crate::table::*;
struct ArgIn(Box<dyn erased_serde::Serialize>);

pub enum Val {
    Col(Col),
    Arg(Arg),
}

pub trait ToArg {
    fn to_arg(self) -> Arg;
}

impl ToArg for usize {
    fn to_arg(self) -> Arg {
        Arg::Uint(self)
    }
}
impl ToArg for isize {
    fn to_arg(self) -> Arg {
        Arg::Int(self)
    }
}
impl ToArg for bool {
    fn to_arg(self) -> Arg {
        Arg::Bool(self)
    }
}
impl ToArg for String {
    fn to_arg(self) -> Arg {
        Arg::Str(self)
    }
}
impl ToArg for f64 {
    fn to_arg(self) -> Arg {
        Arg::Float(self)
    }
}

impl ToArg for Vec<usize> {
    fn to_arg(self) -> Arg {
        Arg::Set(self.into_iter().map(|x| x.to_arg()).collect())
    }
}
impl ToArg for Vec<isize> {
    fn to_arg(self) -> Arg {
        Arg::Set(self.into_iter().map(|x| x.to_arg()).collect())
    }
}
impl ToArg for Vec<bool> {
    fn to_arg(self) -> Arg {
        Arg::Set(self.into_iter().map(|x| x.to_arg()).collect())
    }
}
impl ToArg for Vec<String> {
    fn to_arg(self) -> Arg {
        Arg::Set(self.into_iter().map(|x| x.to_arg()).collect())
    }
}
impl ToArg for Vec<f64> {
    fn to_arg(self) -> Arg {
        Arg::Set(self.into_iter().map(|x| x.to_arg()).collect())
    }
}
pub enum Arg {
    Uint(usize),
    Int(isize),
    Bool(bool),
    Str(String),
    Float(f64),
    Set(Vec<Arg>),
    Null,
}
impl Clone for Arg {
    fn clone(&self) -> Self {
        match self {
            Arg::Uint(v) => Arg::Uint(*v),
            Arg::Int(v) => Arg::Int(*v),
            Arg::Bool(v) => Arg::Bool(*v),
            Arg::Str(v) => Arg::Str(v.clone()),
            Arg::Float(v) => Arg::Float(*v),
            _ => Arg::Null,
        }
    }
}
