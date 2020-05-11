use clap::{crate_authors, crate_version, App, Arg};

pub fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("feline")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Redis compatible in-memory datastore")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true))
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Starts feline in debug mode")
        )
}
