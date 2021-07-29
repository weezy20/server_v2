//! Create HTTP Responses that can be understood by a browser

use std::collections::HashMap;
use std::fmt::Write;

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
    fn from(hrp: HttpResponse<'a>) -> Self {
        let mut res = String::new();
        res.push_str(&format!(
            "{} {} {}",
            hrp.version, hrp.status_code, hrp.status_text
        ));
        match hrp.headers {
            None => (),
            Some(hashmap) => {
                for (k, v) in hashmap {
                    let line = k.to_string() + ": " + v;
                    res.push_str(r#"\r\n"#);
                    res.push_str(&line);
                }
            }
        }
        match hrp.body {
            None => (),
            Some(body) => res.push_str(&body),
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
        response.headers = match &headers {
            Some(_) => headers,
            None => {
                let mut h: HashMap<&'a str, &'a str> = HashMap::new();
                h.insert("Content-type", "text/html");
                Some(h)
            }
        };
        response.body = match &body {
            Some(_) => body,
            None => None,
        };
        response
    }

    /// Write the current HttpResponse object to a `Write` data type
    pub fn send_response(&self, writer: impl)
    {
        let res = self.clone();
        let res = String::from(res);
        let bytes_written = writer.write(res);
    }
}
