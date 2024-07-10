use crate::args::*;
use crate::clauses::*;
use crate::expressions::*;
use crate::table::*;
use crate::traits::*;

pub struct MYSQLBuilder {
    from: Option<Table>,
    select: Option<Select>,
    joins: Vec<JoinClause>,
    r#where: Vec<Where>,
    order: Option<OrderClause>,
}

// TODO impl QueryBuilder
impl QueryBuilder for MYSQLBuilder {
    fn query() -> Self {
        MYSQLBuilder {
            from: None,
            select: None,
            joins: vec![],
            r#where: vec![],
            order: None,
        }
    }
    fn from(mut self, table_name: String) -> Self {
        self.from = Some(Table { name: table_name });
        self
    }
    fn select(mut self, cols: Vec<Col>) -> Self {
        if let Some(mut select) = self.select {
            select.cols.extend(cols);
            self.select = Some(select)
        } else {
            self.select = Some(Select { cols: cols })
        }
        self
    }

    fn r#where(mut self, r#where: Where) -> Self {
        self.r#where.push(r#where);
        self
    }
    // fn r#where<T>(mut self, col: Col, op: Op, val: Val<T>) -> Self
    // where
    //     T: ToArg,
    // {
    //     self.r#where.push(Where{
    //         target: col,
    //         exp: ExpU::Exp(Exp{
    //             op: op,
    //             left: ExpTar::Col(col),
    //             right: match val {

    //             }
    //         }),
    //     });
    //     self
    // }
    fn to_sql(&self) -> Result<(String, Vec<Arg>), Box<dyn std::error::Error>> {
        self.try_to_sql()
    }
}

impl MYSQLBuilder {
    pub fn new() -> Self {
        MYSQLBuilder {
            select: None,
            from: None,
            joins: vec![],
            r#where: vec![],
            order: None,
        }
    }

    pub fn try_to_sql(&self) -> Result<(String, Vec<Arg>), Box<dyn std::error::Error>> {
        let (mut query, mut args) = self.unpack_element(&self.select);
        let (from_query, from_args) = self.unpack_element(&self.from);
        query.push_str(format!("\n{from_query}").as_str());
        args.extend(from_args);
        for join in &self.joins {
            let (join_query, join_args) = self.unpack_element_ref(&Some(join));
            query.push_str(format!("\n{join_query}").as_str());
            args.extend(join_args);
        }
        for r#where in &self.r#where {
            let (where_query, where_args) = self.unpack_element_ref(&Some(r#where));
            query.push_str(format!("\n{where_query}").as_str());
            args.extend(where_args);
        }
        let (order_query, order_args) = self.unpack_element(&self.order);
        query.push_str(format!("\n{order_query}").as_str());
        args.extend(order_args);
        Ok((query, args))
    }

    fn unpack_element<T>(&self, element: &Option<T>) -> (String, Vec<Arg>)
    where
        T: ToSQL,
    {
        match &element {
            Some(value) => {
                let (q, a) = value.to_sql();
                match a {
                    Some(v) => (q, v),
                    None => (q, vec![]),
                }
            }
            None => (String::from(""), vec![]),
        }
    }
    fn unpack_element_ref<T>(&self, element: &Option<&T>) -> (String, Vec<Arg>)
    where
        T: ToSQL,
    {
        match &element {
            Some(value) => {
                let (q, a) = value.to_sql();
                match a {
                    Some(v) => (q, v),
                    None => (q, vec![]),
                }
            }
            None => (String::from(""), vec![]),
        }
    }
}
