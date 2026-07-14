mod user;
mod root;

use user::run_user_service;
use root::run_root_service;

use std::env;


//Rust attribute, runs a macro around this function
#[tokio::main]
async fn main() -> zbus::Result<()> {
    if env::args().any(|arg| arg == "--root") {
        run_root_service().await
    } else {
        run_user_service().await
    }
}
