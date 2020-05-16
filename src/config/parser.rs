/*MIT License

Copyright (c) 2020 Akshay Naik

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the folowing conditions:

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

pub fn parse_config<'a>(config_file: Option<&'a str>) -> Config<'a> {
   
    let mut log_file: Option<&str> = None;
    let default_ip = "127.0.0.1";
    let default_port = 6370;
    // check if configration file present on disk
    if let Some(config_file) = config_file {
        // check if log file present in parsed configuration and add the log file in Config struct
        //log_file = None; // change this when we will read configuration file
    } else {
        log_file = None;
    }

    // check if log file present in the config file
    Config {
        server_ip: default_ip,
        server_port: default_port,
        log_file ,
    }
}

pub struct Config<'a> {
    pub server_ip: &'a str,
    pub server_port: u16,
    pub log_file: Option<&'a str>,
}
