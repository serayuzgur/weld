//! # Page
//! All paging related codes contained under this module.
use service::query_api::query::Query;
/// An enum to hold sort parameters with the direction.
#[derive(Debug)]
pub enum Page {
    /// Ascending sorting.
    OFFSET(u8),
    /// Descendinf sorting.
    LIMIT(u8),
}

impl PartialEq for Page {
    #[inline]
    fn eq(&self, other: &Page) -> bool {
        match self {
            &Page::OFFSET(s_size) => {
                match other {
                    &Page::OFFSET(o_size) => s_size == o_size,
                    &Page::LIMIT(_) => false,
                }
            }
            &Page::LIMIT(s_size) => {
                match other {
                    &Page::OFFSET(_) => false,
                    &Page::LIMIT(o_size) => s_size == o_size,
                }
            }
        }
    }
}

/// Parses parameter to a Sort enum
pub fn parse(param: Query) -> Option<Page> {
    if let Ok(size) = u8::from_str_radix(&param.value, 10) {
        match param.key.as_str() {
            "_offset" => Some(Page::OFFSET(size)),
            "_limit" => Some(Page::LIMIT(size)),
            _ => None,
        }
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_test() {
        assert_eq!(parse(Query::new("", "", "")), None);
        assert_eq!(parse(Query::new("", "_", "")), None);
        assert_eq!(parse(Query::new("", "=", "")), None);
        assert_eq!(parse(Query::new("", "=", "Abc")), None);
        assert_eq!(parse(Query::new("", "=", "123")), None);
        assert_eq!(parse(Query::new("Abc", "=", "123")), None);
        assert_eq!(parse(Query::new("_offset", "=", "")), None);
        assert_eq!(parse(Query::new("_limit", "=", "")), None);
        assert_eq!(parse(Query::new("_offset", "=", "123")),
                   Some(Page::OFFSET(123)));
        assert_eq!(parse(Query::new("_limit", "=", "123")),
                   Some(Page::LIMIT(123)));
    }
}
