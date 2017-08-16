//! # fields
//! All necessery functions for appliying fields to json results.
use serde_json::Value;
use service::query_api::Queries;
use service::query_api::Page;
use std::vec::Vec;

/// let only named fields  array according to the query api
pub fn apply(obj: &mut Value, queries: &Queries) {
    let ref _offset = queries.paginate.0;
    let ref _limit = queries.paginate.1;

    match obj {
        &mut Value::Array(ref mut arr) => {
            let offset = match _offset {
                &Page::OFFSET(ref index) => index.clone(),
                _ => 0u8,
            };
            let limit = match _limit {
                &Page::LIMIT(ref index) => index.clone(),
                _ => arr.len().clone() as u8,
            };
            let o: usize = offset.clone() as usize;
            let l: usize = limit.clone() as usize;

            let mut temp = clone_slice(arr, o, l);

            arr.clear();
            arr.append(&mut temp);
        }
        _ => {
            //No need to handle other types
        }
    }
}
/// copies array within the given indexes.
fn clone_slice(arr: &mut Vec<Value>, offset: usize, limit: usize) -> Vec<Value> {
    let mut temp = Vec::<Value>::new();
    let mut i = 0;
    let upperlimit = offset + limit;
    for val in arr {
        if i >= offset && i < upperlimit {
            temp.push(val.clone());
        }
        i += 1;
    }
    temp
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
                "password":"321"
            }
        ]"#;
        serde_json::from_str(&json_string).unwrap()
    }

    #[test]
    fn apply_test_middle() {
        let mut queries = Queries::new();
        {
            let paginate = &mut queries.paginate;
            paginate.0 = Page::OFFSET(1);
            paginate.1 = Page::LIMIT(1);
        }
        let expected: Value = serde_json::from_str(
            &r#"[
             {
                "name":"kamil",
                "age":900,
                "active":false,
                "password":"333"
            }
        ]"#,
        ).unwrap();
        let json = &mut get_json();
        apply(json, &queries);
        assert_eq!(json.clone(), expected);
    }
    #[test]
    fn apply_test_start() {
        let mut queries = Queries::new();
        {
            let paginate = &mut queries.paginate;
            paginate.0 = Page::OFFSET(0);
            paginate.1 = Page::LIMIT(1);
        }
        let expected: Value = serde_json::from_str(
            &r#"[
              {
                "name":"seray",
                "age":31,
                "active":true,
                "password":"123"
            }
        ]"#,
        ).unwrap();
        let json = &mut get_json();
        apply(json, &queries);
        assert_eq!(json.clone(), expected);
    }
    #[test]
    fn apply_test_end() {
        let mut queries = Queries::new();
        {
            let paginate = &mut queries.paginate;
            paginate.0 = Page::OFFSET(2);
            paginate.1 = Page::LIMIT(1);
        }
        let expected: Value = serde_json::from_str(
            &r#"[
             {
                "name":"hasan",
                "age":25,
                "active":true,
                "password":"321"
            }
        ]"#,
        ).unwrap();
        let json = &mut get_json();
        apply(json, &queries);
        assert_eq!(json.clone(), expected);
    }
}
