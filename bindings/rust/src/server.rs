mod constants;
pub(crate) mod handle_request_external;
pub mod routes;

use self::routes::{RequestHandler, ROUTES};
use crate::module_wrapper;

pub fn start() {
    module_wrapper::init();
    let stringified_routes_string = transform_request_handlers_to_string();
    module_wrapper::start(&stringified_routes_string);
}

pub fn add_route(handler: RequestHandler) {
    let last_index = unsafe { ROUTES.len() } as i32;
    unsafe { ROUTES.insert(last_index, handler) };
}

fn transform_request_handlers_to_string() -> String {
    let mut stringified_routes: Vec<String> = Vec::new();
    for (key, value) in unsafe { ROUTES.iter() } {
        let stringified_route = format!("{}:{}", key, value.path);
        stringified_routes.push(stringified_route);
    }

    let stringified_routes_string = stringified_routes.join(",");
    stringified_routes_string
}
