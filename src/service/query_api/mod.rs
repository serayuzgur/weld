//! # query
//! This module includes necessary structs and functions to parse query patameters in a spesific way.
mod query;
mod sort;
mod page;

pub use self::query::Query;
pub use self::sort::Sort;
pub use self::page::Page;

/// parse query params. For duplicate parameters only last one will be used.
pub fn parse(query: Option<&str>) -> Option<Queries> {
    match query {
        Some(params) => {
            let mut queries = Queries::new();
            for param in params.split("&") {
                if param.is_empty() {
                    continue;
                }
                // wellcome, now we can start the real parsing
                let parts = param.splitn(2, "=").collect::<Vec<&str>>();
                if parts.get(0).is_none() || parts.get(1).is_none() {
                    continue;
                }
                // so we got a real parameter
                let key = parts.get(0).unwrap().to_string();
                let value = parts.get(1).unwrap().to_string();
                if key.starts_with("_") {
                    // fields,offset,limit,sort,filter,q
                    set_where_it_belongs(
                        &mut queries,
                        Query {
                            key: key,
                            value: value,
                            op: "=".to_string(),
                        },
                    );
                }
            }
            Some(queries)
        }
        None => None,
    }
}
fn set_where_it_belongs(queries: &mut Queries, q: Query) {

    match q.key.as_str() {
        "_fields" => {
            let fields_vec = &mut queries.fields;
            fields_vec.extend(
                q.value
                    .split(",")
                    .map(String::from)
                    .collect::<Vec<String>>(),
            );
        }
        "_offset" | "_limit" => {
            if let Some(page) = page::parse(q) {
                let mut paging = &mut queries.paginate;
                println!("paging {:?}", page);
                match page {
                    page::Page::OFFSET(_) => paging.0 = page,
                    page::Page::LIMIT(_) => paging.1 = page,
                }
            }
        }
        "_sort" => {
            let sort_vet = &mut queries.sort;
            sort_vet.extend(q.value.split(",").map(Sort::from).collect::<Vec<Sort>>());
        }
        "_filter" => {
            let filter_vet = &mut queries.filter;
            println!("parsing {}", q.value);
            filter_vet.extend(q.value.split(",").map(Query::from).collect::<Vec<Query>>());
        }
        "_q" => {
            queries.q = Some(q.value);
        }
        _ => {
            // do nothing}
        }
    }
}

/// A simple struct to hold query parameters well structured.
#[derive(Debug, Clone)]
pub struct Queries {
    /// field names to return
    pub fields: Vec<String>,
    /// filter and operation parameters
    pub filter: Vec<Query>,
    /// Full text search
    pub q: Option<String>,
    /// Pagination parameters
    pub paginate: (page::Page, page::Page),
    /// Slice parameters
    pub slice: Vec<Query>,
    /// Sorting parameters
    pub sort: Vec<Sort>,
}

impl Queries {
    /// Creates a new instance with the empty values.
    pub fn new() -> Queries {
        Queries {
            fields: Vec::<String>::new(),
            filter: Vec::<Query>::new(),
            q: None,
            paginate: (page::Page::OFFSET(0), page::Page::LIMIT(10)),
            slice: Vec::<Query>::new(),
            sort: Vec::<Sort>::new(),
        }
    }
}

impl PartialEq for Queries {
    #[inline]
    fn eq(&self, other: &Queries) -> bool {
        self.fields == other.fields && self.filter == other.filter && self.q == other.q &&
            self.paginate == other.paginate && self.slice == other.slice &&
            self.sort == other.sort
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_none_test() {
        assert_eq!(parse(None), None);
    }
    #[test]
    fn parse_fields_test() {
        let mut queries = Queries::new();
        {
            let fields = &mut queries.fields;
            fields.push("a".to_string());
            fields.push("b".to_string());
        }
        assert_eq!(parse(Some("_fields=a,b")), Some(queries));
    }

    #[test]
    fn parse_paginate_test() {
        let mut queries = Queries::new();
        {
            let paginate = &mut queries.paginate;
            paginate.0 = page::Page::OFFSET(10);
            paginate.1 = page::Page::LIMIT(5);
        }
        assert_eq!(parse(Some("_offset=10&_limit=5")), Some(queries));
    }

    #[test]
    fn parse_sort_test() {
        let mut queries = Queries::new();
        {
            let sort = &mut queries.sort;
            sort.push(Sort::ASC("a".to_string()));
            sort.push(Sort::DSC("b".to_string()));
            sort.push(Sort::ASC("c".to_string()));
        }
        assert_eq!(parse(Some("_sort=a+,b-,c")), Some(queries));
    }

    #[test]
    fn parse_filter_test() {
        let mut queries = Queries::new();
        {
            let filter = &mut queries.filter;
            filter.push(Query::new("name", "=", "seray"));
            filter.push(Query::new("active", "=", "true"));
        }
        assert_eq!(parse(Some("_filter=name=seray,active=true")), Some(queries));
    }

    #[test]
    fn parse_q_test() {
        let mut queries = Queries::new();
        {
            queries.q = Some("seray".to_string());
        }
        assert_eq!(parse(Some("_q=seray")), Some(queries));
    }
}
