use http::{http_request::HttpRequest, http_response::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

/// Serves 404 page
pub struct PageNotFound;
/// Serves static resources
pub struct StaticHandler;
/// Serves JSON data
pub struct WebServiceHandler;

pub trait Handler{
    /// Process the incoming request
    fn handle(req: &HttpRequest) -> HttpResponse; 
    /// Load a resource from PUBLIC_DIR or {root}/public folder and return it
    fn load(file: &str) -> Option<String> {
        let default = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public = env::var("PUBLIC_DIR").unwrap_or(default);
        let full_path = format!("{}/{}", public,file);

        let file_contents = fs::read_to_string(full_path);
        file_contents.ok()
    }
}