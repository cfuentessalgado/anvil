use clap::{arg, Command};

fn main() {
    let matches = app().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => add_site_config(sub_matches.get_one::<String>("SITE_URL").expect("required")),
        _ => unreachable!(),
    }
}

fn app() -> Command<'static> {
    return Command::new("Anvil").subcommand_required(true).subcommand(
        Command::new("add")
            .about("Adds new site config")
            .arg(arg!(<SITE_URL> "The site url"))
            .arg_required_else_help(true),
    );
}

fn add_site_config(site_url: &String) {
    println!(
        "Adding site \"{}\"",
        site_url
    );
}
