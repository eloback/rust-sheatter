mod args;

use args::SheatterArgs;
use clap::Parser;
use reqwest::blocking::Client;
use std::io::stdin;

const BASE_URL: &str = "http://cheat.sh/";

fn make_request(client: &Client, url: &str) -> String {
    let req = client.get(url).header("User-Agent", "curl");
    req.send()
        .expect("Error on request.")
        .text()
        .expect("Error on text parse.")
}

fn main() {
    let args = SheatterArgs::parse();

    let topic = args.topic;

    let mut url = BASE_URL.to_owned() + &topic + "/:list";

    let client = Client::new();

    let body = make_request(&client, &url);

    let topics = body.lines();
    topics
        .clone()
        .enumerate()
        .for_each(|(index, topic)| println!("{index}:{topic}"));
    println!("select a topic");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("No input");
    let index: usize = input.trim().parse().expect("failed to parse to int");
    let desc_topic = topics
        .clone()
        .nth(index)
        .expect("no option with that index");

    url = BASE_URL.to_owned() + &topic + "/" + desc_topic;

    let final_response = make_request(&client, &url);
    println!(
        "######### --- ########
    ######### --- ########
    ######### --- ########\n
    {final_response}"
    );
}
