use crate::args::*;
use crate::expressions::*;
use crate::statements::*;
use crate::table::*;
use std::collections::HashMap;

pub trait QueryBuilder {
    fn query() -> Self;
    fn to_sql(&self) -> (String, Vec<Arg>);
}

pub trait FetchQBuilder {
    fn from(self, table_name: &'static str) -> Self;
    fn select(self, cols: Vec<Col>) -> Self;
    fn distinct(self) -> Self;
    fn join(self, table: Table, on: On) -> Self;
    fn left_join(self, table: Table, on: Exp) -> Self;
    fn right_join(self, table: Table, on: Exp) -> Self;
    fn union(self, query: Self) -> Self;
    fn r#where(self, exp: Exp) -> Self;
    fn order(self, by: Col, dir: Dir) -> Self;
    fn group_by(self, by: Col) -> Self;
    fn having(self, exp: ExpU) -> Self;
    fn limit(self, by: i32) -> Self;
}

pub trait UpdateQBuilder {
    fn update(table: Table, set: Vec<Exp>) -> Self;
    fn r#where(self, exp: Exp) -> Self;
}

pub trait InsertQBuilder {
    fn insert(table: Table, data: Vec<HashMap<String, impl ToArg>>) -> Self;
}

pub trait DeleteQBuilder {
    fn delete(table: Table) -> Self;
    fn r#where(self, exp: Exp) -> Self;
}

pub trait ToSQL {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>);
}
