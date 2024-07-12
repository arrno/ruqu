use crate::args::*;
use crate::statements::*;
use crate::expressions::*;
use crate::table::*;

pub trait QueryBuilder {
    fn query() -> Self;
    fn from(self, table_name: &'static str) -> Self;
    fn select(self, cols: Vec<Col>) -> Self;
    // fn join(self) -> Self;
    // fn left_join(self) -> Self;
    // fn right_join(self) -> Self;
    fn r#where(self, exp: Exp) -> Self;
    fn order(self, order: Order) -> Self;
    fn to_sql(&self) -> Result<(String, Vec<Arg>), Box<dyn std::error::Error>>;
}

pub trait ToSQL {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>);
}
