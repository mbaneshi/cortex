use clap::{App, Arg};

pub fn parse_args() -> String {
    let matches = App::new("Cortex CLI")
        .version("0.1.0")
        .about("Command-line utility for Cortex")
        .arg(
            Arg::new("command")
                .about("The command to execute")
                .required(true)
                .index(1),
        )
        .get_matches();

    matches.value_of("command").unwrap().to_string()
}
