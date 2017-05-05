//! # query_api
//! All necessery functions for appliying query api to json results.
use serde_json::{Value, Error};
use serde_json;
use serde_json::error::ErrorCode::Message;
use service::query_api::Queries;

/// filter array according to the query api
pub fn apply(obj: &mut Value, queries: &Queries) {
    let ref filters = queries.filter;
    if let &mut Value::Array(ref mut arr) = obj {
        let mut size = arr.len();
        let mut i = 0;
        while i < size {
            let mut valid = true;
            if let Some(item) = arr.get(i) {
                for q in filters {
                    if let Some(field) = item.get(&q.key) {
                        valid = is_valid(&q.op, field, &q.value);
                        if !valid {
                            break;
                        }
                    }
                }
            } else {
                break;
            }
            if !valid {
                arr.remove(i);
                size -= 1;
            } else {
                i += 1;
            }
        }
    } else {}
}

fn convert_2_same_type(field_value: &Value, query_value: &str) -> Result<Value, Error> {
    match field_value {
        &Value::Bool(_) => return serde_json::to_value(query_value == "true"),
        &Value::Number(_) => return serde_json::to_value(i64::from_str_radix(query_value, 10).ok()),
        &Value::String(_) => return serde_json::to_value(query_value),
        _ => {
            let error = Message("Filter is not applicable for this column".to_string());
            Err(Error::syntax(error, 1, 1))
        }
    }
}

fn is_valid(op: &str, field_value: &Value, query_value: &str) -> bool {
    let mut valid = true;
    let parsed_q = convert_2_same_type(field_value, query_value);
    if let Ok(qval) = parsed_q {
        match op {
            "=" => valid = field_value == &qval,
            "!=" => valid = field_value != &qval,
            ">" => {
                if let Some(num_f) = field_value.as_i64() {
                    if let Some(num_q) = qval.as_i64() {
                        valid = num_f > num_q;
                    }
                }
            }
            ">=" => {
                if let Some(num_f) = field_value.as_i64() {
                    if let Some(num_q) = qval.as_i64() {
                        valid = num_f >= num_q;
                    }
                }
            }
            "<" => {
                if let Some(num_f) = field_value.as_i64() {
                    if let Some(num_q) = qval.as_i64() {
                        valid = num_f < num_q;
                    }
                }
            }
            "<=" => {
                if let Some(num_f) = field_value.as_i64() {
                    if let Some(num_q) = qval.as_i64() {
                        valid = num_f <= num_q;
                    }
                }
            }
            "~=" => {
                println!("checking {:?}~={:?}", field_value, &qval);
                if let Some(str_f) = field_value.as_str() {
                    if let Some(str_q) = qval.as_str() {
                        valid = str_f.starts_with(str_q);
                    }
                }
            }
            "|=" => {
                let parts = query_value.split("|");
                for part in parts {
                    valid = false;
                    if let Ok(qval) = convert_2_same_type(field_value, part) {
                        println!("checking {:?}|={:?}", field_value, &qval);
                        if field_value == &qval {
                            valid = true;
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    valid
}




#[cfg(test)]
mod tests {
    use service::query_api::Query;
    use super::*;

    fn get_json() -> Value {
        let json_string = r#"[
            {
                "name":"seray",
                "age":31,
                "active":true,
                "password":"123"
            },
            {
                "name":"kamil",
                "age":900,
                "active":false,
                "password":"333"
            },
            {
                "name":"hasan",
                "age":25,
                "active":true,
                "password":"321"
            }
        ]"#;
        serde_json::from_str(&json_string).unwrap()
    }

    #[test]
    fn apply_eq_test() {
        let mut queries = Queries::new();
        {
            let filter = &mut queries.filter;
            filter.push(Query::new("name", "=", "seray"));
            filter.push(Query::new("active", "=", "true"));
        }
        let expected: Value = serde_json::from_str(&r#"[
            {
                "name":"seray",
                "age":31,
                "active":true,
                "password":"123"
            }]"#)
            .unwrap();
        let json = &mut get_json();
        apply(json, &queries);
        assert_eq!(json.clone(), expected);
    }
    #[test]
    fn apply_ne_test() {
        let mut queries = Queries::new();
        {
            let filter = &mut queries.filter;
            filter.push(Query::new("name", "!=", "seray"));
            filter.push(Query::new("active", "!=", "true"));
        }
        let expected: Value = serde_json::from_str(&r#"[
            {
                "name":"kamil",
                "age":900,
                "active":false,
                "password":"333"
            }]"#)
            .unwrap();
        let json = &mut get_json();
        apply(json, &queries);
        assert_eq!(json.clone(), expected);
    }

    #[test]
    fn apply_gt_lt_test() {
        let mut queries = Queries::new();
        {
            let filter = &mut queries.filter;
            filter.push(Query::new("age", "<", "500"));
            filter.push(Query::new("age", ">", "26"));
        }
        let expected: Value = serde_json::from_str(&r#"[
            {
                "name":"seray",
                "age":31,
                "active":true,
                "password":"123"
            }]"#)
            .unwrap();
        let json = &mut get_json();
        apply(json, &queries);
        assert_eq!(json.clone(), expected);
    }
    #[test]
    fn apply_gte_lte_test() {
        let mut queries = Queries::new();
        {
            let filter = &mut queries.filter;
            filter.push(Query::new("age", "<=", "31"));
            filter.push(Query::new("age", ">=", "31"));
        }
        let expected: Value = serde_json::from_str(&r#"[
            {
                "name":"seray",
                "age":31,
                "active":true,
                "password":"123"
            }]"#)
            .unwrap();
        let json = &mut get_json();
        apply(json, &queries);
        assert_eq!(json.clone(), expected);
    }
    #[test]
    fn apply_like_test() {
        let mut queries = Queries::new();
        {
            let filter = &mut queries.filter;
            filter.push(Query::new("password", "~=", "3"));
        }
        let expected: Value = serde_json::from_str(&r#"[
            {
                "name":"kamil",
                "age":900,
                "active":false,
                "password":"333"
            },
            {
                "name":"hasan",
                "age":25,
                "active":true,
                "password":"321"
            }
            ]"#)
            .unwrap();
        let json = &mut get_json();
        apply(json, &queries);
        assert_eq!(json.clone(), expected);
    }

    #[test]
    fn apply_in_test() {
        let mut queries = Queries::new();
        {
            let filter = &mut queries.filter;
            filter.push(Query::new("name", "|=", "kamil|hasan"));
        }
        let expected: Value = serde_json::from_str(&r#"[
            {
                "name":"kamil",
                "age":900,
                "active":false,
                "password":"333"
            },
            {
                "name":"hasan",
                "age":25,
                "active":true,
                "password":"321"
            }
            ]"#)
            .unwrap();
        let json = &mut get_json();
        apply(json, &queries);
        assert_eq!(json.clone(), expected);
    }
}