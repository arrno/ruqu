fn main() {
    let qb = MYSQLBuilder::new();
}

trait QueryBuilder {
    fn from(self) -> Self;
    fn select(self) -> Self;
    fn join(self) -> Self;
    fn left_join(self) -> Self;
    fn right_join(self) -> Self;
    fn r#where(self) -> Self;
    fn order(self) -> Self;
    fn to_sql(&self) -> (String, Vec<Arg>);
}

enum Arg {
    Usize(usize),
    Isize(isize),
    Bool(bool),
    String(String),
    Float(f64),
}
impl Clone for Arg {
    fn clone(&self) -> Self {
        match self {
            Arg::Usize(v) => Arg::Usize(*v),
            Arg::Isize(v) => Arg::Isize(*v),
            Arg::Bool(v) => Arg::Bool(*v),
            Arg::String(v) => Arg::String(v.clone()),
            Arg::Float(v) => Arg::Float(*v),
        }
    }
}
struct MYSQLBuilder<'a> {
    from: Option<Col>,
    select: Option<Select>,
    joins: Vec<JoinClause<'a>>,
    r#where: Option<Where<'a>>,
    order: Option<Order>,
}

// TODO impl QueryBuilder

impl MYSQLBuilder<'_> {
    fn new() -> Self {
        MYSQLBuilder {
            from: None,
            select: None,
            joins: vec![],
            r#where: None,
            order: None,
        }
    }
    // TODO
    fn try_to_sql(&self) -> Result<(String, Vec<Arg>), Box<dyn std::error::Error>> {
        Ok(("".to_string(), vec![]))
    }
}

trait ToSQL {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>);
}

// TODO
struct Select {}

#[derive(Clone)]
enum Join {
    Inner,
    Left,
    Right,
    Union,
}
impl From<Join> for String {
    fn from(join: Join) -> Self {
        match join {
            Join::Inner => String::from("JOIN"),
            Join::Left => String::from("LEFT JOIN"),
            Join::Right => String::from("RIGHT JOIN"),
            _ => String::from("JOIN"),
        }
    }
}
struct JoinClause<'a> {
    from: &'a Col,
    join: Join,
    on: Option<On<'a>>,
}
impl ToSQL for JoinClause<'_> {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let join_str: String = self.join.clone().into();
        let mut args = vec![];
        let (from_sql, from_args) = self.from.to_sql();
        let mut sql = format!("{join_str} {from_sql}");
        if let Some(on) = &self.on {
            let (exp_sql, exp_args_op) = on.r#where.to_sql();
            if let Some(exp_args) = exp_args_op {
                args.extend(exp_args)
            }
            sql.push_str(format!(" ON ({exp_sql})").as_str());
        };
        (sql, Some(args))
    }
}

struct On<'a> {
    r#where: Where<'a>,
}
impl ToSQL for On<'_> {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let (exp_sql, exp_args) = self.r#where.exp.to_sql();
        (format!("ON ({exp_sql})"), exp_args)
    }
}
struct Where<'a> {
    target: &'a Col,
    exp: ExpU<'a>,
}

impl ToSQL for Where<'_> {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let (exp_sql, exp_args) = self.exp.to_sql();
        (format!("WHERE ({exp_sql})"), exp_args)
    }
}

// TODO
// struct GroupBy {}

enum Order {
    Asc,
    Desc,
}
impl ToSQL for Order {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        match self {
            Order::Asc => (String::from("ASC"), None),
            Order::Desc => (String::from("DESC"), None),
        }
    }
}
struct OrderClause<'a> {
    by: Vec<&'a Col>,
    dir: Order,
}
impl ToSQL for OrderClause<'_> {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        let mut col_sql = Vec::new();
        let mut args = Vec::new();
        self.by.iter().for_each(|col| {
            let (csql, cargs) = col.to_sql();
            col_sql.push(csql);
            if let Some(v) = cargs {
                args.extend(v);
            }
        });
        (
            format!(
                "ORDER BY {} {}",
                col_sql.join(" ").as_str(),
                self.dir.to_sql().0
            ),
            Some(args),
        )
    }
}

struct Table {
    name: String,
}

impl Table {
    fn new(name: String) -> Self {
        return Table { name: name };
    }
    fn col(&self, name: String) -> Col {
        Col {
            table_name: self.name.clone(),
            name: name,
        }
    }
}

impl ToSQL for Table {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        (format!("`{}`", self.name), None)
    }
}

struct Col {
    table_name: String,
    name: String,
}

impl<'a> Col {
    fn name(&self) -> &str {
        &self.name
    }
    fn eq(&'a self, comp: ExpTar<'a>) -> ExpU {
        self.make_exp(comp, Op::Eq)
    }
    fn neq(&'a self, comp: ExpTar<'a>) -> ExpU {
        self.make_exp(comp, Op::Neq)
    }
    fn lt(&'a self, comp: ExpTar<'a>) -> ExpU {
        self.make_exp(comp, Op::Lt)
    }
    fn gt(&'a self, comp: ExpTar<'a>) -> ExpU {
        self.make_exp(comp, Op::Gt)
    }
    fn make_exp(&'a self, comp: ExpTar<'a>, op: Op) -> ExpU {
        ExpU::Exp(Exp {
            op: op,
            left: ExpTar::Col(self),
            right: comp,
        })
    }
}

impl ToSQL for Col {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        (format!("`{}`.`{}`", self.table_name, self.name), None)
    }
}

// ---------------- EXPRESSIONS -------------------------------
enum Op {
    Eq,
    Neq,
    Lt,
    Gt,
}
impl ToSQL for Op {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        match self {
            Op::Eq => (String::from("="), None),
            Op::Neq => (String::from("!="), None),
            Op::Lt => (String::from("<"), None),
            Op::Gt => (String::from(">"), None),
        }
    }
}

enum ExpU<'a> {
    Exp(Exp<'a>),
    And(And<'a>),
    Or(Or<'a>),
}
impl ToSQL for ExpU<'_> {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        match self {
            ExpU::Exp(e) => e.to_sql(),
            ExpU::And(a) => a.to_sql(),
            ExpU::Or(o) => o.to_sql(),
        }
    }
}
struct And<'a> {
    left: Box<ExpU<'a>>,
    right: Box<ExpU<'a>>,
}
impl ToSQL for And<'_> {
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
struct Or<'a> {
    left: Box<ExpU<'a>>,
    right: Box<ExpU<'a>>,
}
impl ToSQL for Or<'_> {
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

enum ExpTar<'a> {
    Arg(Arg),
    Col(&'a Col),
}

impl ToSQL for ExpTar<'_> {
    fn to_sql(&self) -> (String, Option<Vec<Arg>>) {
        match self {
            ExpTar::Arg(arg) => (String::from("?"), Some(vec![arg.clone()])),
            ExpTar::Col(col) => (col.to_sql().0, None),
        }
    }
}
struct Exp<'a> {
    op: Op,
    left: ExpTar<'a>,
    right: ExpTar<'a>,
}

impl ToSQL for Exp<'_> {
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