use std::collections::HashMap;

use once_cell::sync::Lazy;

pub struct RequestHandler {
    pub path: String,
    pub handler: fn(String) -> String,
}

pub(crate) type RequestHandlerMap = HashMap<i32, RequestHandler>;

pub(crate) static mut ROUTES: Lazy<RequestHandlerMap> = Lazy::new(|| RequestHandlerMap::new());
