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

use std::process;

use clap::App;

mod internal;
mod cli;

use internal::server;
use cli::app;


fn main() {

    let app = app::get_app();
    let config =  get_config(app);
    
    if let Err(e) = server::start_server(&config) {
        eprintln!("Error occured while starting server {}", e);
        process::exit(1);
    }
}

fn get_config(app: App) -> String { 
    let matches = app.get_matches();

    match(matches.value_of("config")) {

        Some(config_file) => {
            String::from(config_file)
        },
        None => {
            println!("No config file given using default config file");
            String::from("feline.toml")
        },
    }
}
