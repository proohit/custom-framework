use custom_framework_rust::server::{self, routes::RequestHandler};

fn main() {
    server::add_route(RequestHandler {
        path: "/".to_string(),
        handler: |request| {
            println!("Request: {:?}", request);
            "Hello, World!".to_string()
        },
    });

    server::start();
}
