use std::io;
use std::env;

struct Bot;

fn main() {
    let token = env::var("TOKEN").expect("Expected a token in the environment");
    let channel_id = env::var("CHANNEL_ID").expect("Expected a channel ID in the environment");


}
