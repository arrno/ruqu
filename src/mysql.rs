use crate::args::*;
use crate::expressions::*;
use crate::statements::*;
use crate::table::*;
use crate::traits::*;

pub struct MYSQLBuilder {
    from: Option<Table>,
    select: Option<Select>,
    joins: Vec<Join>,
    unions: Vec<MYSQLBuilder>,
    r#where: Option<Where>,
    order: Vec<Order>,
    limit: Option<Limit>,
    group_by: Option<GroupBy>,
}

impl QueryBuilder for MYSQLBuilder {
    fn query() -> Self {
        MYSQLBuilder {
            from: None,
            select: None,
            joins: vec![],
            unions: vec![],
            r#where: None,
            order: vec![],
            limit: None,
            group_by: None,
        }
    }
    fn to_sql(&self) -> (String, Vec<Arg>) {
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
        let (group_query, group_args) = self.unpack_element(&self.group_by);
        if group_query.len() > 0 {
            query.push_str(format!("\n{group_query}").as_str());
            args.extend(group_args);
        }
        let (limit_query, limit_args) = self.unpack_element(&self.limit);
        if limit_query.len() > 0 {
            query.push_str(format!("\n{limit_query}").as_str());
            args.extend(limit_args);
        }
        for qb in &self.unions {
            let (union_query, union_args) = qb.to_sql();
            query.push_str(format!("\nUNION\n{union_query}").as_str());
            args.extend(union_args);
        }
        (query, args)
    }
}

impl FetchQBuilder for MYSQLBuilder {
    fn from(mut self, table_name: &'static str) -> Self {
        self.from = Some(Table::new(table_name.to_string()));
        self
    }
    fn select(mut self, cols: Vec<Col>) -> Self {
        if let Some(mut select) = self.select {
            select.extend(cols);
            self.select = Some(select)
        } else {
            self.select = Some(Select::new(cols))
        }
        self
    }
    fn distinct(mut self) -> Self {
        if let Some(mut select) = self.select {
            select.distinct();
            self.select = Some(select);
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
    fn join(self, table: Table, on: On) -> Self {
        self.do_join(table, *on.exp, JoinType::Inner)
    }
    fn left_join(self, table: Table, on: Exp) -> Self {
        self.do_join(table, on, JoinType::Inner)
    }
    fn right_join(self, table: Table, on: Exp) -> Self {
        self.do_join(table, on, JoinType::Inner)
    }
    fn union(mut self, query: Self) -> Self {
        self.unions.push(query);
        self
    }
    fn group_by(mut self, by: Col) -> Self {
        match self.group_by {
            Some(mut group_by) => {
                group_by.extend(vec![by]);
                self.group_by = Some(group_by);
            }
            None => self.group_by = Some(GroupBy::new(vec![by])),
        };
        self
    }
    fn having(mut self, exp: ExpU) -> Self {
        if let Some(mut group_by) = self.group_by {
            group_by.having(exp);
            self.group_by = Some(group_by);
        }
        self
    }
    fn order(mut self, by: Col, dir: Dir) -> Self {
        self.order.push(Order::new(by, dir));
        self
    }
    fn limit(mut self, by: i32) -> Self {
        self.limit = Some(Limit::new(by));
        self
    }
}

impl MYSQLBuilder {
    pub fn new() -> Self {
        MYSQLBuilder {
            select: None,
            from: None,
            joins: vec![],
            unions: vec![],
            r#where: None,
            order: vec![],
            limit: None,
            group_by: None,
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
        let (group_query, group_args) = self.unpack_element(&self.group_by);
        if group_query.len() > 0 {
            query.push_str(format!("\n{group_query}").as_str());
            args.extend(group_args);
        }
        let (limit_query, limit_args) = self.unpack_element(&self.limit);
        if limit_query.len() > 0 {
            query.push_str(format!("\n{limit_query}").as_str());
            args.extend(limit_args);
        }
        for qb in &self.unions {
            let (union_query, union_args) = qb.to_sql();
            query.push_str(format!("\nUNION\n{union_query}").as_str());
            args.extend(union_args);
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

    fn do_join(mut self, table: Table, on: Exp, join: JoinType) -> Self {
        self.joins.push(Join::new(table, join, Some(On::new(on))));
        self
    }
}
