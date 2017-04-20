use std::cmp::{PartialEq, Eq};

/// Splits query params
pub fn split_query(query: Option<&str>) -> Vec<Query> {
    match query {
        Some(params) => {
            // params.split("&").filter(|x| !x.is_empty()).map(String::from).collect::<Vec<String>>()
            let mut queries = Vec::<Query>::new();
            for param in params.split("&") {
                if param.is_empty() {
                    continue;
                }
                let parts = param.split("=").collect::<Vec<&str>>();
                if parts.get(0).is_none() || parts.get(1).is_none() {
                    continue;
                }
                let key = parts.get(0).unwrap().to_string();
                let value = parts.get(1).unwrap().to_string();
                let op = match key.split("_").collect::<Vec<&str>>().get(1) {
                    Some(v) => v.to_string(),
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

#[derive(Eq)]
#[derive(Debug)]
pub struct Query {
    pub key: String,
    pub value: String,
    pub op: String,
}
impl Query {
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
    fn split_query_test() {
        assert_eq!(split_query(None), Vec::<Query>::new());
        assert_eq!(split_query(Some("")), Vec::<Query>::new());
        assert_eq!(split_query(Some("&&")), Vec::<Query>::new());
        assert_eq!(split_query(Some("a=1&b=2&c=3")),
                   vec![Query::new("a".to_string(), "=".to_string(), "1".to_string()),
                        Query::new("b".to_string(), "=".to_string(), "2".to_string()),
                        Query::new("c".to_string(), "=".to_string(), "3".to_string())]);
    }
}