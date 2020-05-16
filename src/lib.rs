/*MIT License

Copyright (c) 2020 Akshay Naik

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

mod config;
mod internal;
mod cli;

use std::process;
use std::path::Path;

use clap::App;
use log::{error};

use internal::server;
use cli::app;
use config::Config;
use config::logger;

pub fn start() {

    let app = app::get_app();
    let config_file =  get_config_file(app);
    // if configuration file exist then pass configuration file else send None
    let config:Config = if Path::new(&config_file).exists() {
        config::parse_config(Some(&config_file))
    } else {
        println!("configuration file {} not found on disk ",&config_file);
        config::parse_config(None)
    };

    if let Some(log_file) =  config.log_file {
        logger::create_logger(Some(log_file));

    } else {
        logger::create_logger(None); 
    }

    if let Err(e) = server::start_server(&config) {
        error!("Error occured while starting server {}", e);
        process::exit(1);
    }
}

// Read configuration flle from cli, if not present then use default configuratoin file i.e. feline.toml
fn get_config_file(app: App) -> String { 
    let matches = app.get_matches();

    match matches.value_of("config") {
        Some(config_file) => {
            String::from(config_file)
        },
        None => {
            println!("No config file given using default config file, feline.toml");
            String::from("feline.toml")
        },
    }
}
