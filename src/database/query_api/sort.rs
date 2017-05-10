//! # sort
//! All necessery functions for appliying sorting to json results.
use serde_json::Value;
use service::query_api::Queries;
use service::query_api::Sort;
use serde_json::Map;
use std::cmp::Ordering;

fn compare(a: &Map<String, Value>, b: &Map<String, Value>, sort_key: &String) -> Ordering {
    if let Some(a_val) = a.get(sort_key) {
        if let Some(b_val) = b.get(sort_key) {
            match a_val {
                &Value::Array(ref a_arr) => {
                    if let &Value::Array(ref b_arr) = b_val {
                        return a_arr.len().cmp(&b_arr.len());
                    } else {
                        return Ordering::Greater;
                    }
                }
                &Value::Object(ref a_obj) => {
                    if let &Value::Object(ref b_obj) = b_val {
                        return a_obj.len().cmp(&b_obj.len());
                    } else {
                        return Ordering::Greater;
                    }
                }
                &Value::Number(ref a_num) => {
                    if let &Value::Number(ref b_num) = b_val {
                        // TODO: clear unwraps
                        return a_num.as_f64()
                            .unwrap()
                            .partial_cmp(&b_num.as_f64().unwrap())
                            .unwrap();
                    } else {
                        return Ordering::Greater;
                    }
                }
                &Value::String(ref a_str) => {
                    if let &Value::String(ref b_str) = b_val {
                        return a_str.cmp(&b_str);
                    } else {
                        return Ordering::Greater;
                    }
                }
                &Value::Bool(a_bool) => {
                    if let &Value::Bool(b_bool) = b_val {
                        return a_bool.cmp(&b_bool);
                    } else {
                        return Ordering::Greater;
                    }
                }
                &Value::Null => {
                    if let &Value::Null = b_val {
                        return Ordering::Equal;
                    } else {
                        return Ordering::Less;
                    }
                }
            }
        } else {
            Ordering::Greater
        }
    } else {
        if let Some(_) = b.get(sort_key) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

/// sort on all desired fields fields according to the query api
pub fn apply(obj: &mut Value, queries: &Queries) {
    if queries.sort.len() == 0 {
        return;
    }
    if let &mut Value::Array(ref mut arr) = obj {
        let ref sorts = queries.sort;
        arr.sort_by(|a: &Value, b: &Value| {
            match a {
                &Value::Object(ref map_a) => {
                    if let &Value::Object(ref map_b) = b {
                        let mut result = Ordering::Equal;
                        for sort in sorts {
                            result = match sort {
                                &Sort::ASC(ref sort_key) => compare(&map_a, &map_b, &sort_key),
                                &Sort::DSC(ref sort_key) => compare(&map_b, &map_a, &sort_key),
                            };
                            if result != Ordering::Equal {
                                return result;
                            }
                        }
                        return result;
                    } else {
                        Ordering::Greater
                    }
                }
                _ => Ordering::Less,
            }
        });
    };
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use service::query_api::Sort;

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
                "name":"recep",
                "age":25,
                "active":true,
                "password":"3212"
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
    fn apply_sort_test() {
        let mut queries = Queries::new();
        {
            queries.sort.push(Sort::ASC("age".to_string()));
            queries.sort.push(Sort::ASC("name".to_string()));
        }
        let expected: Value = serde_json::from_str(&r#"[
            {
                "name":"hasan",
                "age":25,
                "active":true,
                "password":"3212"
            },
            {
                "name":"recep",
                "age":25,
                "active":true,
                "password":"3212"
            },
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
            }
        ]"#)
            .unwrap();
        let json = &mut get_json();
        apply(json, &queries);
        assert_eq!(json.clone(), expected);
    }
}