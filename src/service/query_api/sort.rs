//! # sort
//! All sorting related codes contained under this module.
use std::cmp::PartialEq;

/// An enum to hold sort parameters with the direction.
#[derive(Debug)]
pub enum Sort {
    /// Ascending sorting.
    ASC(String),
    /// Descendinf sorting.
    DSC(String),
}

impl PartialEq for Sort {
    #[inline]
    fn eq(&self, other: &Sort) -> bool {
        match self {
            &Sort::ASC(ref s_name) => {
                match other {
                    &Sort::ASC(ref o_name) => s_name == o_name,
                    &Sort::DSC(_) => false,
                }
            }
            &Sort::DSC(ref s_name) => {
                match other {
                    &Sort::ASC(_) => false,
                    &Sort::DSC(ref o_name) => s_name == o_name,
                }
            }
        }
    }
}

impl<'a> From<&'a str> for Sort {
    fn from(s: &'a str) -> Sort {
        if let Some(sort) = parse(s) {
            return sort;
        } else {
            return Sort::ASC(s.to_string());
        }
    }
}

/// Parses parameter to a Sort enum
pub fn parse<S: Into<String>>(param: S) -> Option<Sort> {
    let mut param_mut = param.into().to_string();
    match param_mut.pop() {
        Some(t) => {
            match t {
                '+' => Some(Sort::ASC(param_mut)),
                '-' => Some(Sort::DSC(param_mut)),
                _ => None,
            }
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_test() {
        assert_eq!(parse(""), None);
        assert_eq!(parse("name"), None);
        assert_eq!(parse("name+"), Some(Sort::ASC("name".to_string())));
        assert_eq!(parse("name-"), Some(Sort::DSC("name".to_string())));
    }
}
