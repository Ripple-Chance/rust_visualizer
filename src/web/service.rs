use std::net::SocketAddr;

pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    
    println!("Web server feature is temporarily disabled.");
    println!("To enable web server, add hyper dependency to Cargo.toml.");
    println!("Server would run on http://{}", addr);
    
    Ok(())
}
