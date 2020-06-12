mod install;
mod start;
mod stop;
mod wipe;

use clap_nested::Commander;

pub fn commander<'a>() -> Commander<'a, (), ()> {
    Commander::new()
        .add_cmd(install::cmd())
        .add_cmd(start::cmd())
        .add_cmd(stop::cmd())
        .add_cmd(wipe::cmd())
        .no_cmd(|_args, _matches| {
            println!("No subcommand matched");
            Ok(())
        })
}
