#![allow(dead_code)]
use clap_nested::Command;
use duct;

pub fn cmd<'a>() -> Command<'a, ()> {
    Command::new("wipe")
        .description("Stop all services and clean all block chain and bridge data")
        .runner(|_env, _matches| {
            duct::cmd("docker-compose", vec!["down", "-v"])
                .then(
                    duct::cmd("rm", vec![
                        "-r",
                        &format!("{}/.skymavis", std::env::var("HOME").unwrap()),
                    ])
                )
                .run()
                .unwrap();
            Ok(())
        })
}
