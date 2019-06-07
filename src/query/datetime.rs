use crate::{expr::Expr, query::Query};

query![Date, Epoch, Time];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Date<'a> {
    date: Expr<'a>,
}

impl<'a> Date<'a> {
    pub fn new(dateish: impl Into<Expr<'a>>) -> Self {
        Self {
            date: dateish.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum EpochUnit {
    #[serde(rename = "second")]
    Second,
    #[serde(rename = "millisecond")]
    Millisecond,
    #[serde(rename = "microsecond")]
    Microsecond,
    #[serde(rename = "nanosecond")]
    Nanosecond,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Epoch<'a> {
    epoch: Expr<'a>,
    unit: EpochUnit,
}

impl<'a> Epoch<'a> {
    pub fn new(num: impl Into<Expr<'a>>, unit: EpochUnit) -> Self {
        Self {
            epoch: num.into(),
            unit,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Time<'a> {
    time: Expr<'a>,
}

impl<'a> Time<'a> {
    pub fn new(timeish: impl Into<Expr<'a>>) -> Self {
        Self {
            time: timeish.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde_json::{self, json};

    #[test]
    fn test_date() {
        let fun = Date::new("1970-01-01");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "date": "1970-01-01",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_epoch() {
        let fun = Epoch::new(5, EpochUnit::Second);

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "epoch": 5,
            "unit": "second"
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_time() {
        let fun = Time::new("1970-01-01T00:00:00+00:00");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "time": "1970-01-01T00:00:00+00:00",
        });

        assert_eq!(expected, serialized);
    }
}
