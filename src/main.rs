use clap::{arg, Command};

fn cli() -> Command {
    Command::new("cypher")
        .about("A bunch of cypher tools.")
        .subcommand_required(true)
        .subcommand(
            Command::new("rot")
                .arg(arg!(<STRING> "String to rotate").required(true))
                .arg(arg!(<ROT> "Rotation amount").required(false).default_value("13").value_parser(clap::value_parser!(usize)))
        )
}

fn main() {
    let commands = cli().get_matches();

    match commands.subcommand() {
        Some(("rot", sub)) => {
            let string = sub.get_one::<String>("STRING").unwrap();
            let rot = sub.get_one::<usize>("ROT").unwrap();
            println!("{:?}", cypher::rot(string.to_string(), *rot));
        }
        _ => println!("Invalid command. Please use <help> to see full list of commands.")
    }
}
