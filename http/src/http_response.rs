//! Create HTTP Responses that can be understood by a browser
#![allow(non_snake_case)]
use std::collections::HashMap;
use std::io::Write;

/// Type representing an HTTP response

// The status line can be broken down into 3 fields:
// HTTP version, Status Code and Status text
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    // Serialize HttpResponse into a String for transmission
    fn from(hrp: HttpResponse<'a>) -> String {
        let mut res = String::new();
        res.push_str(&format!(
            "{} {} {}",
            hrp.version, hrp.status_code, hrp.status_text
        ));
        let mut content_length_defined: bool = false;
        match hrp.headers {
            None => (),
            Some(hashmap) => {
                for (k, v) in hashmap {
                    // Insert one ugly check for Content-length so that it may not
                    // be repeate in the match hrp.body code block:
                    if let Some(_) = k.find("Content-length") {
                        content_length_defined = true;
                    }
                    let line = k.to_string() + ": " + v;
                    // for some reason r#"\r\n" pushes double slashes like : \\r\\n
                    res.push_str("\r\n");
                    res.push_str(&line);
                }
            }
        }
        match hrp.body {
            None => (),
            Some(body) => {
                // There's a risk of double inserting Content-length here:
                // hrp.headers may already contain this line
                if !content_length_defined {
                    res.push_str(&format!("\r\nContent-length: {}", body.len()));
                }
                res.push_str("\r\n\r\n");
                res.push_str(&body);
            }
        }
        res
    }
}

impl<'a> HttpResponse<'a> {
    /// Accepts custom values for a HTTP response and edits them into the default value
    /// as returned by `HttpResponse::default()`
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        // todo!("Add a new field to set HTTP version");
        let mut response = Self::default();
        if status_code != response.status_code {
            response.status_code = status_code;
        }
        response.status_text = match status_code {
            // you can add more status codes here
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Lol",
        };
        response.headers = match headers {
            Some(mut headers) => {
                // Check if headers contains Content-type
                // if not, insert it
                headers.entry("Content-type").or_insert("text/html");
                Some(headers)
            }
            None => {
                let mut h: HashMap<&'a str, &'a str> = HashMap::new();
                h.insert("Content-type", "text/html");
                Some(h)
            }
        };
        response.body = match body {
            Some(_) => body,
            None => None,
        };
        response
    }

    /// Write the current HttpResponse object to a `Write` data type
    pub fn send_response(&self, writer: &mut impl Write) -> std::io::Result<()> {
        let res_clone = self.clone();
        let res = String::from(res_clone);
        // let bytes_written = writer.write(res.as_bytes())?;
        write!(writer, "{}", res)
    }
}

mod tests {
    use super::*;
    #[test]
    fn check_status_200_OK() {
        let response = HttpResponse::new("200", None, Some("lorem ipsum".into()));
        let expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h: HashMap<&str, &str> = HashMap::new();
                h.insert("Content-type", "text/html");
                Some(h)
            },
            body: Some(String::from("lorem ipsum")),
        };
        assert_eq!(response, expected);
    }
    #[test]
    fn check_double_content_length_defined() {
        let body = String::from("lorem ipsum");
        let body_len = &body.len().to_string()[..];
        let mut headers: HashMap<&str, &str> = HashMap::new();

        headers.entry("Content-length").or_insert(body_len);
        headers.insert("Content-type", "text/html");

        let response =
            HttpResponse::new("200", Some(headers), Some("lorem ipsum".into()));
        let expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h: HashMap<&str, &str> = HashMap::new();
                h.insert("Content-type", "text/html");
                // headers has a Content-length defined so the body block in
                // `From<HttpResponse> for String` should not trigger
                h.insert("Content-length", body_len);
                Some(h)
            },
            body: Some(body),
        };
        assert_eq!(response, expected);
    }
    #[test]
    fn check_serialization_http_response() {
        let body = "It's a nice day today";
        let response = HttpResponse::new(
            "500",
            {
                let mut h = HashMap::new();
                h.insert("Content-type", "text/html");
                Some(h)
            },
            Some(body.to_string()),
        );
        let response_str = String::from(response);

        let expected = format!("HTTP/1.1 500 Internal Server Error\r\nContent-type: text/html\r\nContent-length: {}\r\n\r\n{}", body.len(), body);
        assert_eq!(response_str, expected);
    }
}
