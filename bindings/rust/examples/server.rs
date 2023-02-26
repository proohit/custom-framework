use custom_framework_rust::server::Server;

#[cfg_attr(test, test)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new();
    server.start();
    Ok(())
}
