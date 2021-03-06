//! Logical functions
use crate::{
    expr::{Expr, Ref},
    query::Query,
};
use chrono::{DateTime, Utc};

query![And, Or, Not, Contains, Exists, Equals, Lt, Lte, Gt, Gte];

/// The `And` function computes the conjunction of a list of boolean values,
/// returning `true` if all elements are "true", and `false` otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/and)
#[derive(Serialize, Debug, Clone, Default)]
pub struct And<'a> {
    and: Vec<Expr<'a>>,
}

impl<'a> And<'a> {
    /// A simple and with two expressions. For a vector comparison, use the
    /// `From` trait.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self {
            and: vec![left.into(), right.into()],
        }
    }
}

impl<'a, I, E> From<I> for And<'a>
where
    I: IntoIterator<Item = E>,
    E: Into<Expr<'a>>,
{
    fn from(exprs: I) -> Self {
        Self {
            and: exprs.into_iter().map(Into::into).collect(),
        }
    }
}

/// The `Or` function operates on one or more values and returns true if at least
/// one of the values is `true`.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/or)
#[derive(Serialize, Debug, Clone, Default)]
pub struct Or<'a> {
    or: Vec<Expr<'a>>,
}

impl<'a> Or<'a> {
    /// A simple and with two expressions. For a vector comparison, use the
    /// `From` trait.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self {
            or: vec![left.into(), right.into()],
        }
    }
}

impl<'a, I, E> From<I> for Or<'a>
where
    I: IntoIterator<Item = E>,
    E: Into<Expr<'a>>,
{
    fn from(exprs: I) -> Self {
        Self {
            or: exprs.into_iter().map(Into::into).collect(),
        }
    }
}

/// The `Not` function computes the negation of a boolean value, returning true if
/// its argument is `false`, or `false` if its argument is `true`.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/not)
#[derive(Serialize, Debug, Clone)]
pub struct Not<'a> {
    not: Expr<'a>,
}

impl<'a> Not<'a> {
    pub fn new(expr: impl Into<Expr<'a>>) -> Self {
        Self { not: expr.into() }
    }
}

/// The `Equals` function tests equivalence between a list of values.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/equals)
#[derive(Serialize, Debug, Clone)]
pub struct Equals<'a> {
    equals: Vec<Expr<'a>>,
}

impl<'a> Equals<'a> {
    /// A simple and with two expressions. For a vector comparison, use the
    /// `From` trait.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self {
            equals: vec![left.into(), right.into()],
        }
    }
}

impl<'a, I, E> From<I> for Equals<'a>
where
    I: IntoIterator<Item = E>,
    E: Into<Expr<'a>>,
{
    fn from(exprs: I) -> Self {
        Self {
            equals: exprs.into_iter().map(Into::into).collect(),
        }
    }
}

/// The `Lt` function returns `true` if each specified value is less than the ones
/// following it, and `false` otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/lt)
#[derive(Serialize, Debug, Clone, Default)]
pub struct Lt<'a> {
    lt: Vec<Expr<'a>>,
}

impl<'a> Lt<'a> {
    /// A simple and with two expressions. For a vector comparison, use the
    /// `From` trait.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self {
            lt: vec![left.into(), right.into()],
        }
    }
}

impl<'a, I, E> From<I> for Lt<'a>
where
    I: IntoIterator<Item = E>,
    E: Into<Expr<'a>>,
{
    fn from(exprs: I) -> Self {
        Self {
            lt: exprs.into_iter().map(Into::into).collect(),
        }
    }
}

/// The `Lte` function returns `true` if each specified value is less than or
/// equal to the ones following it, and `false` otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/lte)
#[derive(Serialize, Debug, Clone, Default)]
pub struct Lte<'a> {
    lte: Vec<Expr<'a>>,
}

impl<'a> Lte<'a> {
    /// A simple and with two expressions. For a vector comparison, use the
    /// `From` trait.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self {
            lte: vec![left.into(), right.into()],
        }
    }
}

impl<'a, I, E> From<I> for Lte<'a>
where
    I: IntoIterator<Item = E>,
    E: Into<Expr<'a>>,
{
    fn from(exprs: I) -> Self {
        Self {
            lte: exprs.into_iter().map(Into::into).collect(),
        }
    }
}

/// The `Gt` function returns `true` if each specified value is greater than the
/// ones following it, and `false` otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/gt)
#[derive(Serialize, Debug, Clone, Default)]
pub struct Gt<'a> {
    gt: Vec<Expr<'a>>,
}

impl<'a> Gt<'a> {
    /// A simple and with two expressions. For a vector comparison, use the
    /// `From` trait.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self {
            gt: vec![left.into(), right.into()],
        }
    }
}

impl<'a, I, E> From<I> for Gt<'a>
where
    I: IntoIterator<Item = E>,
    E: Into<Expr<'a>>,
{
    fn from(exprs: I) -> Self {
        Self {
            gt: exprs.into_iter().map(Into::into).collect(),
        }
    }
}

/// The `Gte` function returns `true` if each specified value is greater than or
/// equal to the ones following it, and `false` otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/gte)
#[derive(Serialize, Debug, Clone, Default)]
pub struct Gte<'a> {
    gte: Vec<Expr<'a>>,
}

impl<'a> Gte<'a> {
    /// A simple and with two expressions. For a vector comparison, use the
    /// `From` trait.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self {
            gte: vec![left.into(), right.into()],
        }
    }
}

impl<'a, I, E> From<I> for Gte<'a>
where
    I: IntoIterator<Item = E>,
    E: Into<Expr<'a>>,
{
    fn from(exprs: I) -> Self {
        Self {
            gte: exprs.into_iter().map(Into::into).collect(),
        }
    }
}

/// The `Contains` function returns `true` if the argument passed as in contains a
/// value at the specified path, and `false` otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/contains)
#[derive(Serialize, Debug, Clone)]
pub struct Contains<'a> {
    contains: Vec<Expr<'a>>,
    #[serde(rename = "in")]
    in_: Expr<'a>,
}

impl<'a> Contains<'a> {
    pub fn new<I, E, F>(path: I, in_: F) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Expr<'a>>,
        F: Into<Expr<'a>>,
    {
        Self {
            contains: path.into_iter().map(|e| e.into()).collect(),
            in_: in_.into(),
        }
    }
}

/// The `Exists` function returns boolean `true` if the provided ref exists at the
/// specified timestamp (in the case of an instance), or is non-empty (in the
/// case of a set), and `false` otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/exists)
#[derive(Serialize, Debug, Clone)]
pub struct Exists<'a> {
    exists: Expr<'a>,
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    timestamp: Option<Expr<'a>>,
}

impl<'a> Exists<'a> {
    pub fn new(reference: Ref<'a>) -> Self {
        Self {
            exists: Expr::from(reference),
            timestamp: None,
        }
    }

    pub fn timestamp(&mut self, ts: DateTime<Utc>) -> &mut Self {
        self.timestamp = Some(Expr::from(ts));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use chrono::{offset::TimeZone, Utc};
    use serde_json::{self, json};

    #[test]
    fn test_and() {
        let aaaand = And::from(vec![true, true, false]);
        let query = Query::from(aaaand);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"and": [true, true, false]}), serialized);
    }

    #[test]
    fn test_or() {
        let oooor = Or::new(Var::new("x"), false);
        let query = Query::from(oooor);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"or": [{"var": "x"}, false]}), serialized);
    }

    #[test]
    fn test_not() {
        let noooot = Not::new(false);
        let query = Query::from(noooot);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"not": false}), serialized);
    }

    #[test]
    fn test_equals() {
        let equals = Equals::new("musti", "naukio");
        let query = Query::from(equals);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"equals": ["musti", "naukio"]}), serialized);
    }

    #[test]
    fn test_exists() {
        let mut exists = Exists::new(Ref::instance("Musti"));
        exists.timestamp(Utc.timestamp(60, 0));

        let query = Query::from(exists);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "exists": {
                "@ref": {
                    "id": "Musti"
                }
            },
            "ts": {
                "@ts": Utc.timestamp(60, 0)
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_lt() {
        let lt = Lt::new(1, 2);
        let query = Query::from(lt);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"lt": [1, 2]}), serialized);
    }

    #[test]
    fn test_lte() {
        let lte = Lte::new(2, 3);
        let query = Query::from(lte);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"lte": [2, 3]}), serialized);
    }

    #[test]
    fn test_gt() {
        let gt = Gt::new(1, 2);
        let query = Query::from(gt);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"gt": [1, 2]}), serialized);
    }

    #[test]
    fn test_gte() {
        let gte = Gte::from(vec![1, 2, 3]);
        let query = Query::from(gte);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"gte": [1, 2, 3]}), serialized);
    }
}
