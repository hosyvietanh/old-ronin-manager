mod cmd;
mod util;

fn main() {
    cmd::commander().run().unwrap_or_else(|e| e.exit());
}
