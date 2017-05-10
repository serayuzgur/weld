//! # fields
//! All necessery functions for appliying fields to json results.
use serde_json::Value;
use serde_json;
use service::query_api::Queries;

/// let only named fields  array according to the query api
pub fn apply(obj: &mut Value, queries: &Queries) {
    let ref fields = queries.fields;
    if fields.len() == 0 {
        return;
    }
    match obj {
        &mut Value::Array(ref mut arr) => {
            let mut i = 0usize;
            let size = (&arr.len()).clone();
            let remove_keys = match arr.get(0) {
                Some(item) => {
                    if let &Value::Object(ref map) = item {
                        diff_left(map.keys(), fields)
                    } else {
                        Vec::<String>::new()
                    }
                }
                None => Vec::<String>::new(),
            };
            while i < size {
                let map_val = arr.get_mut(i).unwrap();
                if let &mut Value::Object(ref mut obj) = map_val {
                    for key in &remove_keys {
                        obj.remove(key);
                    }
                }
                i += 1;
            }

        }
        &mut Value::Object(ref mut map) => {
            let remove_keys = diff_left(map.keys(), fields);
            for key in &remove_keys {
                map.remove(key);
            }
        }
        _ => {
            //No need to handle other types
        }
    }
}

fn diff_left(a: serde_json::map::Keys, b: &Vec<String>) -> Vec<String> {
    let mut diff = Vec::<String>::new();
    'outer: for a_item in a {
        for b_item in b {
            if a_item == b_item {
                continue 'outer;
            }
        }
        diff.push(a_item.clone());
    }
    diff
}

#[cfg(test)]
mod tests {
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
    fn apply_test() {
        let mut queries = Queries::new();
        {
            let fields = &mut queries.fields;
            fields.push("name".to_string());
            fields.push("active".to_string());
        }
        let expected: Value = serde_json::from_str(&r#"[
            {
                "name":"seray",
                "active":true
            },
            {
                "name":"kamil",
                "active":false
            },
            {
                "name":"hasan",
                "active":true
            }
        ]"#)
            .unwrap();
        let json = &mut get_json();
        apply(json, &queries);
        assert_eq!(json.clone(), expected);
    }

    #[test]
    fn diff_left_test() {
        let mut a = serde_json::Map::<String, Value>::new();
        a.insert("a".to_string(), Value::Null);
        a.insert("b".to_string(), Value::Null);
        a.insert("c".to_string(), Value::Null);
        let b = vec!["b".to_string(), "c".to_string()];
        let r = diff_left(a.keys(), &b);
        assert_eq!(r, vec!["a".to_string()]);
    }
}