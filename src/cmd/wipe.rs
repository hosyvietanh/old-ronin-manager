#![allow(dead_code)]
use clap_nested::Command;
use colored::Colorize;
use duct;

pub fn cmd<'a>() -> Command<'a, ()> {
    Command::new("wipe")
        .description("Stop all services and clean all block chain and bridge data")
        .runner(|_env, _matches| {
            let res = duct::cmd("docker-compose", vec!["down", "-v"])
                .then(
                    duct::cmd("rm", vec![
                        "-r",
                        &format!("{}/.skymavis", std::env::var("HOME").unwrap()),
                    ])
                )
                .run();
            match res {
                Err(e) => {
                    println!("Error happened: {}", e);
                    println!(
                        "Please run this command to wipe data: {}",
                        &format!("sudo rm -r {}/.skymavis", std::env::var("HOME").unwrap()).yellow(),
                    );
                }
                _ => {
                    println!("Data wiped successfully!");
                }
            }
            Ok(())
        })
}
