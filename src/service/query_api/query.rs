//! # query_param
//! This module includes necessary structs and functions to parse query patameters in a spesific way.
use std::cmp::PartialEq;

/// A simple struct to hold necessary information about a query parameter.
#[derive(Debug,Clone,Eq)]
pub struct Query {
    /// key of the parameter. It holds pure key without any `_eq` etc.
    pub key: String,
    /// value of the parameter.
    pub value: String,
    /// operation of the parameter. =, eq,neq,gtw,let,like
    pub op: String,
}
impl Query {
    /// Creates new instance for the query
    #[allow(dead_code)]
    pub fn new<S: Into<String>>(key: S, op: S, value: S) -> Query {
        Query {
            key: key.into(),
            op: op.into(),
            value: value.into(),
        }
    }
}
impl PartialEq for Query {
    #[inline]
    fn eq(&self, other: &Query) -> bool {
        self.key == other.key && self.value == other.value && self.op == other.op
    }
}

impl<'a> From<&'a str> for Query {
    fn from(s: &'a str) -> Query {
        if let Some(query) = parse(s) {
            return query;
        } else {
            return Query::new("", "", "");
        }
    }
}

/// Parses parameter to a Sort enum
pub fn parse<S: Into<String>>(param: S) -> Option<Query> {
    // add all ops =,!=,<,<=,>,>=,~=,|=
    let mut param_s = param.into();

    let indexes = find_indexes(param_s.clone());

    if indexes.0 != -1i8 && indexes.1 != -1i8 {
        let mut op = param_s.split_off(indexes.0 as usize);
        let val = op.split_off(indexes.1 as usize);
        Some(Query::new(param_s.to_string(), op.clone(), val.clone()))
    } else {
        None
    }
}

fn find_indexes(param: String) -> (i8, i8) {
    let mut key_end = -1i8;

    let chars = param.chars();
    let mut idx = -1i8;
    let mut pre_c = ' ';
    for c in chars {
        idx += 1;
        if key_end == -1i8 {
            match c {
                '=' | '!' | '<' | '>' | '~' | '|' => {
                    key_end = idx;
                    pre_c = c;
                }
                _ => {}
            }
        } else {
            match c {
                '=' => return (key_end, 2),
                _ => {
                    match pre_c {
                        '=' | '<' | '>' => return (key_end, 1),
                        _ => return (-1, -1),
                    }
                }
            }
        }
    }
    (key_end, -1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_test() {
        assert_eq!(parse(""), None);
        assert_eq!(parse("a|b"), None);
        assert_eq!(parse("aaa|*bbb"), None);
        assert_eq!(parse("aaa|09*bbb"), None);

        assert_eq!(parse("a=b"), Some(Query::new("a", "=", "b")));
        assert_eq!(parse("aaa=bbb"), Some(Query::new("aaa", "=", "bbb")));

        assert_eq!(parse("a!=b"), Some(Query::new("a", "!=", "b")));
        assert_eq!(parse("aaa!=bbb"), Some(Query::new("aaa", "!=", "bbb")));

        assert_eq!(parse("a<b"), Some(Query::new("a", "<", "b")));
        assert_eq!(parse("aaa<bbb"), Some(Query::new("aaa", "<", "bbb")));
        assert_eq!(parse("a<=b"), Some(Query::new("a", "<=", "b")));
        assert_eq!(parse("aaa<=bbb"), Some(Query::new("aaa", "<=", "bbb")));

        assert_eq!(parse("a>b"), Some(Query::new("a", ">", "b")));
        assert_eq!(parse("aaa>bbb"), Some(Query::new("aaa", ">", "bbb")));
        assert_eq!(parse("a>=b"), Some(Query::new("a", ">=", "b")));
        assert_eq!(parse("aaa>=bbb"), Some(Query::new("aaa", ">=", "bbb")));


        assert_eq!(parse("a~=b"), Some(Query::new("a", "~=", "b")));
        assert_eq!(parse("aaa~=bbb"), Some(Query::new("aaa", "~=", "bbb")));

        assert_eq!(parse("a|=b"), Some(Query::new("a", "|=", "b")));
        assert_eq!(parse("aaa|=bbb"), Some(Query::new("aaa", "|=", "bbb")));


    }
}
