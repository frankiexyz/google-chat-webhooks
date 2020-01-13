use std::collections::HashMap;
extern crate getopts;
use getopts::Options;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("m", "", "Message to send", "NAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let output = matches.opt_str("m");
    let mut map = HashMap::new();
    if args.len() > 1 {
    map.insert("text", output);

    let api = "***YOUR WEBHOOK URL***";
    let client = reqwest::Client::new();
    let res = client.post(api)
        .json(&map)
        .send()
        .await?;
    println!("{:#?}", res);
    }
    Ok(())
}
