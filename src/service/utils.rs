use hyper::StatusCode;
use hyper::server::Response;
use hyper;
use futures::{Future, BoxFuture};
use futures::future::ok;
use hyper::header::ContentType;
use serde_json;

/// Prepares an error response , logs it, wraps to BoxFuture.
pub fn error(response: Response,
             code: StatusCode,
             message: &str)
             -> BoxFuture<Response, hyper::Error> {
    ok(response.with_header(ContentType::plaintext())
            .with_status(code)
            .with_body(message.to_string()))
        .boxed()
}

/// Prepares an success response, wraps to BoxFuture.
pub fn success(response: Response,
               code: StatusCode,
               value: &serde_json::Value)
               -> BoxFuture<Response, hyper::Error> {
    ok(response.with_header(ContentType::json())
            .with_status(code)
            .with_body(serde_json::to_vec(&value).unwrap()))
        .boxed()
}

/// Splits '/'  and filters empty strings
pub fn split_path(path: String) -> Vec<String> {
    path.split("/").filter(|x| !x.is_empty()).map(String::from).collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_path_test() {
        // split_path("as/saß".to_string());
        assert_eq!(split_path("".to_string()),
                   Vec::<String>::new(),
                   "Empty string must return empty vector.");
        assert_eq!(split_path("/".to_string()),
                   Vec::<String>::new(),
                   "Empty string must return empty vector.");
        assert_eq!(split_path("//".to_string()),
                   Vec::<String>::new(),
                   "Empty string must return empty vector.");
        assert_eq!(split_path("/posts".to_string()), vec!["posts"]);
        assert_eq!(split_path("/posts/".to_string()), vec!["posts"]);
        assert_eq!(split_path("/posts/1".to_string()), vec!["posts", "1"]);
    }
}