extern crate hyper;
extern crate regex;
extern crate clap;

use std::io::Read;
use hyper::Client;
use hyper::header::Connection;
use regex::Regex;
use clap::{Arg,App};

fn main() {

    // Setup CLI
    let matches = App::new("Accuweather")
        .version("1.0")
        .author("James B. <j-bartholomew@live.com>")
        .about("Retrieves the current weather information from Accuweather")
        .arg(Arg::with_name("METRIC")
             .short("m")
             .long("metric")
             .help("Sets which metric should be used to display the temperature (Fahrenheit(0), Celsius(1))")
             .takes_value(true))
        .arg(Arg::with_name("CODE")
             .short("c")
             .long("code")
             .help("The country code for the location. For example: EUR|NL|NL008|AMSTERDAM'")
             .takes_value(true))
        .get_matches();
    
    //Parse args or use default
    let metric = matches.value_of("METRIC").unwrap_or("1");
    let country_code = matches.value_of("CODE").unwrap_or("EUR|NL|NL008|AMSTERDAM");

    //Connect to accuweather
    let url = format!("http://rss.accuweather.com/rss/liveweather_rss.asp?metric={}&locCode={}", metric, country_code);
    let client = Client::new();
    let mut res = client.get(&url)
        .header(Connection::close())
        .send().unwrap();

    //Parse the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let re = Regex::new(r"<title>Currently: (.*)?</title>").unwrap();
    let text = &body;
    let cap = re.captures(text).unwrap();

    //Print weather info to STDOUT
    println!("{}", cap.at(1).unwrap_or(""));
}
