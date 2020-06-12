use clap_nested::Command;
use crate::util;

pub fn cmd<'a>() -> Command<'a, ()> {
    Command::new("install")
        .description("Installs all prerequisites")
        .runner(|_env, _matches| {
            util::install_docker();
            util::install_docker_compose();
            Ok(())
        })
}
