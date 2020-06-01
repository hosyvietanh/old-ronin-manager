use std::ffi::OsStr;
use std::fmt;
use colored::Colorize;
use which::which;

pub fn command_exists<C: AsRef<OsStr>>(cmd: C) -> bool {
    which(cmd).is_ok()
}

pub fn check_install<C: AsRef<OsStr> + fmt::Display>(cmd: C) -> bool {
    print!(
        "{} {} {}",
        "Checking if".yellow(),
        cmd,
        "is installed..".yellow()
    );

    let installed = command_exists(cmd);

    println!(
        " {}",
        if installed {
            "✓".green()
        } else {
            "✘".red()
        }
    );

    installed
}

pub fn install_docker() {
    if !check_install("docker") {
        println!("Installing docker...");

        duct::cmd(
            "curl",
            vec![
                "-fsSL",
                "https://get.docker.com",
                "-o",
                "get-docker.sh",
            ],
        )
            .then(duct::cmd(
                "sh",
                vec!["get-docker.sh"],
            ))
            .run()
            .unwrap();
    }
}

pub fn install_docker_compose() {
    if !check_install("docker-compose") {
        println!("Installing docker-compose...");

        duct::cmd(
            "sudo",
            vec![
                "curl",
                "-L",
                "https://github.com/docker/compose/releases/download/1.25.5/run.sh",
                "-o",
                "/usr/bin/docker-compose",
            ],
        )
            .then(duct::cmd(
                "sudo",
                vec![
                    "chmod",
                    "+x",
                    "/usr/bin/docker-compose"
                ],
            ))
            .run().unwrap();
    }
}
