use crate::args::*;
use crate::expressions::*;
use crate::statements::*;
use crate::table::*;
use crate::traits::*;

pub struct MYSQLBuilder {
    from: Option<Table>,
    select: Option<Select>,
    joins: Vec<Join>,
    r#where: Option<Where>,
    order: Vec<Order>,
}

// TODO impl QueryBuilder
impl QueryBuilder for MYSQLBuilder {
    fn query() -> Self {
        MYSQLBuilder {
            from: None,
            select: None,
            joins: vec![],
            r#where: None,
            order: vec![],
        }
    }
    fn from(mut self, table_name: &'static str) -> Self {
        self.from = Some(Table {
            name: table_name.to_string(),
        });
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

    fn r#where(mut self, exp: Exp) -> Self {
        match self.r#where {
            Some(where_clause) => {
                self.r#where = Some(Where {
                    exp: Box::new(where_clause.exp.and(exp)),
                })
            }
            None => self.r#where = Some(Where { exp: Box::new(exp) }),
        };
        self
    }

    fn join(self, col: Col, on: On) -> Self {
        self.do_join(col, *on.exp, JoinType::Inner)
    }

    fn left_join(self, col: Col, on: Exp) -> Self {
        self.do_join(col, on, JoinType::Inner)
    }

    fn right_join(self, col: Col, on: Exp) -> Self {
        self.do_join(col, on, JoinType::Inner)
    }

    fn order(mut self, order: Order) -> Self {
        self.order.push(order);
        self
    }

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
            r#where: None,
            order: vec![],
        }
    }

    pub fn try_to_sql(&self) -> Result<(String, Vec<Arg>), Box<dyn std::error::Error>> {
        let (mut query, mut args) = self.unpack_element(&self.select);
        let (from_query, from_args) = self.unpack_element(&self.from);
        query.push_str(format!("\nFROM {from_query}").as_str());
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
        let mut order_query_strings = vec![];
        for order in &self.order {
            let (or_query, order_args) = self.unpack_element_ref(&Some(order));
            order_query_strings.push(or_query);
            args.extend(order_args);
        }
        if order_query_strings.len() > 0 {
            let order_query = format!("\nORDER BY {}", order_query_strings.join(", "));
            query.push_str(order_query.as_str());
        }
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

    fn do_join(mut self, col: Col, on: Exp, join: JoinType) -> Self {
        self.joins.push(Join {
            from: col,
            join: join,
            on: Some(On::new(on)),
        });
        self
    }
}
