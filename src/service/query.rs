//! # query
//! This module includes necessary structs and functions to parse query patameters in a spesific way. 
//! For filtering data.
use std::cmp::PartialEq;

#[allow(dead_code)]
/// parse query params ad
pub fn parse_query(query: Option<&str>) -> Vec<Query> {
    match query {
        Some(params) => {
            let mut queries = Vec::<Query>::new();
            for param in params.split("&") {
                if param.is_empty() {
                    continue;
                }
                let parts = param.split("=").collect::<Vec<&str>>();
                if parts.get(0).is_none() || parts.get(1).is_none() {
                    continue;
                }
                let mut key = parts.get(0).unwrap().to_string();
                let value = parts.get(1).unwrap().to_string();
                let key_op = key.to_string();
                let key_op_vec = key_op.split("_").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
                let op = match key_op_vec.get(1) {
                    Some(v) => {
                        key =  key_op_vec.get(0).unwrap().to_string();
                        v.to_string()
                        }
                    None => "=".to_string(),
                };

                queries.push(Query {
                    key: key,
                    value: value,
                    op: op,
                });
            }
            queries
        }
        None => Vec::<Query>::new(),
    }
}

/// A simple struct to hold necessary information about a query parameter.
#[derive(Eq)]
#[derive(Debug)]
pub struct Query {
    /// key of the parameter. It holds pure key without any `_eq` etc. 
    pub key: String,
    /// value of the parameter.
    pub value: String,
    /// operation of the parameter. =, eq,neq,gtw,let,like
    pub op: String,
}
impl Query {
    /// 
    #[allow(dead_code)]
    pub fn new(key: String, op: String, value: String) -> Query {
        Query {
            key: key,
            op: op,
            value: value,
        }
    }
}
impl PartialEq for Query {
    #[inline]
    fn eq(&self, other: &Query) -> bool {
        self.key == other.key && self.value == other.value && self.op == other.op
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_query_test() {
        assert_eq!(parse_query(None), Vec::<Query>::new());
        assert_eq!(parse_query(Some("")), Vec::<Query>::new());
        assert_eq!(parse_query(Some("&&")), Vec::<Query>::new());
        assert_eq!(parse_query(Some("a=1&b=2&c=3")),
                   vec![Query::new("a".to_string(), "=".to_string(), "1".to_string()),
                        Query::new("b".to_string(), "=".to_string(), "2".to_string()),
                        Query::new("c".to_string(), "=".to_string(), "3".to_string())]);

        assert_eq!(parse_query(Some("_start=20&_end=30")),
                   vec![Query::new("_start".to_string(), "=".to_string(), "20".to_string()),
                        Query::new("_end".to_string(), "=".to_string(), "30".to_string())]);
        assert_eq!(parse_query(Some("views_gte=10&views_lte=20")),
                   vec![Query::new("views".to_string(), "gte".to_string(), "10".to_string()),
                        Query::new("views".to_string(), "lte".to_string(), "20".to_string())]);
        assert_eq!(parse_query(Some("id_ne=1")),
                   vec![Query::new("id".to_string(), "ne".to_string(), "1".to_string())]);
        assert_eq!(parse_query(Some("title_like=server")),
                   vec![Query::new("title".to_string(), "like".to_string(), "server".to_string())]);
        assert_eq!(parse_query(Some("q=internet")),
                   vec![Query::new("q".to_string(), "=".to_string(), "internet".to_string())]);

                        
    }
}
