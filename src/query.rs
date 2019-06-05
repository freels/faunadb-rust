mod cond;
mod create;
mod create_class;
mod create_database;
mod create_index;
mod delete;
mod do_many;
mod get;
mod lambda;
mod let_var;
mod logical;
mod map;
mod var;

pub use cond::*;
pub use create::*;
pub use create_class::*;
pub use create_database::*;
pub use create_index::*;
pub use delete::*;
pub use do_many::*;
pub use get::*;
pub use lambda::*;
pub use let_var::*;
pub use logical::*;
pub use map::*;
pub use var::*;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Query<'a> {
    Create(Create<'a>),
    CreateClass(Box<CreateClass<'a>>),
    CreateDatabase(CreateDatabase<'a>),
    CreateIndex(Box<CreateIndex<'a>>),
    Delete(Delete<'a>),
    Get(Get<'a>),
    Do(Do<'a>),
    If(If<'a>),
    Let(Let<'a>),
    Var(Var<'a>),
    Lambda(Lambda<'a>),
    Map(Map<'a>),
    And(And<'a>),
    Or(Or<'a>),
    Not(Not<'a>),
    Lt(Lt<'a>),
    Lte(Lte<'a>),
    Gt(Gt<'a>),
    Gte(Gte<'a>),
    Contains(Contains<'a>),
    Equals(Equals<'a>),
    Exists(Exists<'a>),
}

query!(
    Create,
    CreateDatabase,
    Get,
    If,
    Delete,
    Do,
    Let,
    Var,
    Lambda,
    Map,
    And,
    Or,
    Not,
    Contains,
    Exists,
    Equals,
    Lt,
    Lte,
    Gt,
    Gte
);

boxed_query!(CreateClass, CreateIndex);

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use chrono::{offset::TimeZone, Utc};
    use serde_json::{self, json};

    #[test]
    fn test_create() {
        let mut obj = Object::default();
        obj.insert("test_field", "test_value");

        let params = InstanceParams::new(obj);

        let query = Query::from(Create::new(Ref::class("test"), params));
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "params": {
                "object": {
                    "data": {
                        "object": {
                            "test_field": "test_value"
                        }
                    }
                }
            },
            "create": {
                "@ref": {
                    "class": {
                        "@ref": {
                            "id": "classes"
                        }
                    },
                    "id": "test",
                }
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_create_class() {
        let mut permission = ClassPermission::default();
        permission.read(Level::public());

        let mut params = ClassParams::new("test");
        params.history_days(10);
        params.permissions(permission);

        let query = Query::from(CreateClass::new(params));
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "create_class": {
                "object": {
                    "history_days": 10,
                    "name": "test",
                    "permissions": { "object": { "read": "public" } },
                    "ttl_days": null,
                }
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_create_index() {
        let mut permission = IndexPermission::default();
        permission.read(Level::public());

        let mut params = IndexParams::new("meows", Ref::class("cats"));
        params.permissions(permission);

        let age_term = Term::field(vec!["data", "age"]);
        let name_term = Term::binding("cats_name");

        params.terms(vec![age_term, name_term]);

        let name_value = Value::field(vec!["data", "name"]);

        let mut age_value = Value::binding("cats_age");
        age_value.reverse();

        params.values(vec![age_value, name_value]);

        let query = Query::from(CreateIndex::new(params));
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "create_index": {
                "object": {
                    "active": false,
                    "name": "meows",
                    "permissions": {
                        "object": {
                            "read": "public",
                        }
                    },
                    "serialized": false,
                    "source": {
                        "@ref": {
                            "class": {
                                "@ref": {
                                    "id": "classes",
                                },
                            },
                            "id": "cats",
                        },
                    },
                    "terms": [
                        {
                            "object": {
                                "field": ["data", "age"],
                            }
                        },
                        {
                            "object": {
                                "binding": "cats_name",
                            }
                        },
                    ],
                    "unique": false,
                    "values": [
                        {
                            "object": {
                                "binding": "cats_age",
                                "reverse": true,
                            }
                        },
                        {
                            "object": {
                                "field": ["data", "name"],
                                "reverse": false,
                            }
                        },
                    ]
                }
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_create_database() {
        let mut params = DatabaseParams::new("test");
        params.priority(10).unwrap();

        let query = Query::from(CreateDatabase::new(params));
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "create_database": {
                "object": {
                    "name": "test",
                    "api_version": 2.0,
                    "priority": 10,
                }
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_get() {
        let mut get = Get::instance(Ref::instance("musti"));
        get.timestamp(Utc.timestamp(60, 0));

        let query = Query::from(get);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "get": {
                "@ref": {
                    "id": "musti"
                }
            },
            "ts": {
                "@ts": Utc.timestamp(60, 0)
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_delete() {
        let delete = Delete::instance(Ref::instance("musti"));
        let query = Query::from(delete);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "delete": {
                "@ref": {
                    "id": "musti"
                }
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_do() {
        let mut do_many = Do::new(Get::instance(Ref::instance("musti")));
        do_many.push(Delete::instance(Ref::instance("musti")));

        let query = Query::from(do_many);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "do": [
                {"get": {"@ref": {"id": "musti"}}},
                {"delete": {"@ref": {"id": "musti"}}},
            ]
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_if() {
        let query = Query::from(If::cond(true, "is true", "is false"));
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "if": true,
            "then": "is true",
            "else": "is false",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_let_var() {
        let let_var = Let::bindings(
            vec![Binding::new("cat", If::cond(true, "Musti", "Naukio"))],
            Var::new("cat"),
        );

        let query = Query::from(let_var);

        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "let": {"cat": {"if": true, "then": "Musti", "else": "Naukio"}},
            "in": {"var": "cat"},
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_lambda() {
        let lambda = Lambda::new(vec!["cat"], Var::new("cat"));
        let query = Query::from(lambda);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "lambda": ["cat"],
            "expr": {"var": "cat"},
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_map() {
        let map = Map::new(
            Array::from(vec!["Musti", "Naukio"]),
            Lambda::new(vec!["cat"], Var::new("cat")),
        );

        let query = Query::from(map);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "collection": ["Musti", "Naukio"],
            "map": {
                "lambda": ["cat"],
                "expr": {"var": "cat"},
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_and() {
        let aaaand = And::new(vec![true, true, false]);
        let query = Query::from(aaaand);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"and": [true, true, false]}), serialized);
    }

    #[test]
    fn test_or() {
        let oooor = Or::new(vec![true, true, false]);
        let query = Query::from(oooor);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"or": [true, true, false]}), serialized);
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
        let equals = Equals::new(vec!["musti", "naukio"]);
        let query = Query::from(equals);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"equals": ["musti", "naukio"]}), serialized);
    }

    #[test]
    fn test_lt() {
        let lt = Lt::new(vec![1, 2, 3]);
        let query = Query::from(lt);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"lt": [1, 2, 3]}), serialized);
    }

    #[test]
    fn test_lte() {
        let lte = Lte::new(vec![1, 2, 3]);
        let query = Query::from(lte);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"lte": [1, 2, 3]}), serialized);
    }

    #[test]
    fn test_gt() {
        let gt = Gt::new(vec![1, 2, 3]);
        let query = Query::from(gt);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"gt": [1, 2, 3]}), serialized);
    }

    #[test]
    fn test_gte() {
        let gte = Gte::new(vec![1, 2, 3]);
        let query = Query::from(gte);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"gte": [1, 2, 3]}), serialized);
    }
}
