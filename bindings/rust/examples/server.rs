use custom_framework_rust::server::{self, routes::RequestHandler};

#[cfg_attr(test, test)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::add_route(RequestHandler {
        path: "/".to_string(),
        handler: |request_body| {
            println!("Request body: {}", request_body);
            "Hello from Rust!".to_string()
        },
    });

    server::start();

    Ok(())
}
