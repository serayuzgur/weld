//! # q
//! All necessery functions for appliying full-text search to json results.
use serde_json::Value;
use service::query_api::Queries;

/// full-text search on all String fields according to the query api
pub fn apply(obj: &mut Value, queries: &Queries) {
    if let Some(ref q) = queries.q {
        if let &mut Value::Array(ref mut arr) = obj {
            let mut size = arr.len();
            let mut i = 0;
            while i < size {
                let mut valid = false;
                if let Some(item) = arr.get(i) {
                    /// get item field list
                    match item {
                        &Value::Object(ref map) => {
                            for key in map.keys() {
                                if let Some(field) = item.get(key) {
                                    if let &Value::String(ref f_str) = field {
                                        valid = f_str.contains(q);
                                        if valid {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        &Value::String(ref f_val) => {
                            valid = f_val == q;
                            if !valid {
                                break;
                            }
                        }
                        _ => {}
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
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
                "password":"3212"
            }
        ]"#;
        serde_json::from_str(&json_string).unwrap()
    }

    #[test]
    fn apply_q_test() {
        let mut queries = Queries::new();
        {
            queries.q = Some("12".to_string());
        }
        let expected: Value = serde_json::from_str(&r#"[
            {
                "name":"seray",
                "age":31,
                "active":true,
                "password":"123"
            },
            {
                "name":"hasan",
                "age":25,
                "active":true,
                "password":"3212"
            }]"#)
            .unwrap();
        let json = &mut get_json();
        apply(json, &queries);
        assert_eq!(json.clone(), expected);
    }
}