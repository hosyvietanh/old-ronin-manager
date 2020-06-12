use clap_nested::Command;
use duct;

pub fn cmd<'a>() -> Command<'a, ()> {
    Command::new("stop")
        .description("Stop all services")
        .runner(|_env, _matches| {
            duct::cmd(
                "docker-compose",
                vec![
                    "down",
                ],
            )
                .run()
                .unwrap();
            Ok(())
        })
}
