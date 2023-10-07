use server::Server;

fn main() {
    let mut server = Server::new();
    println!("Server running.");
    loop {
        server.tick(std::time::Duration::ZERO);
    }
}
