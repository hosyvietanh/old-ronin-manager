mod cmd;
mod util;

use std::env;

fn main() {
    let mut path = std::env::current_exe().unwrap();
    // Get the directory
    path.pop();
    env::set_current_dir(&path)
        .expect(&format!("Unable to change working directory into {}", path.display()));
    cmd::commander().run().unwrap_or_else(|e| e.exit());
}
