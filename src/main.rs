mod server;
mod handler;
mod command;
mod parser;
mod store;
mod aof;

#[tokio::main]
async fn main() {

    print_banner();

    let addr = "127.0.0.1:6379";
    println!("MiniRedis is Listening on {}",addr);

    if let Err(e) = server::run(addr).await{
        eprint!("Server error: {}",e);
    }
}
fn print_banner() {
    println!();
    println!("🧠  MemIgnite v0.1.0");
    println!("    In-Memory Key-Value Engine");
    println!("------------------------------------------------");
    println!();
}