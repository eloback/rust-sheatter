mod args;

use args::SheatterArgs;
use clap::Parser;
use reqwest::blocking::Client;
use std::io::stdin;

const BASE_URL: &str = "http://cheat.sh/";

fn make_request(client: &Client, url: &str) -> Result<String, reqwest::Error> {
    let req = client.get(url).header("User-Agent", "curl");
    req.send()?.text()
}

fn print_topics(topics: &str) {
    println!("select a topic");
    println!("Here are some sugestions:");
    topics.lines().for_each(|topic| println!("\t{topic}"));
}

fn get_topic() -> Result<String, std::io::Error> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = SheatterArgs::parse();
    let client = Client::new();

    let topic = args.topic;

    let url = BASE_URL.to_owned() + &topic + "/:list";
    let topics = make_request(&client, &url)?;

    print_topics(&topics);
    let selected_topic = get_topic()?;

    // shadowing url
    let url = BASE_URL.to_owned() + &topic + "/" + &selected_topic;
    let page = make_request(&client, &url)?;
    println!("\n\n{page}");
    Ok(())
}
