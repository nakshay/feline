mod internal;
mod cli;

use std::process;

use clap::App;

use internal::server;
use cli::app;


pub fn start() {

    let app = app::get_app();
    let config =  get_config(app);
    
    if let Err(e) = server::start_server(&config) {
        eprintln!("Error occured while starting server {}", e);
        process::exit(1);
    }
}

fn get_config(app: App) -> String { 
    let matches = app.get_matches();

    match matches.value_of("config") {
        Some(config_file) => {
            String::from(config_file)
        },
        None => {
            println!("No config file given using default config file");
            String::from("feline.toml")
        },
    }
}
