use clap::Parser;
use std::fs::{read_to_string, File};
use std::io::Write;

#[derive(Parser)]
struct Args {
    /// JSON to update
    file: String,

    /// New file to create
    newFile: String,

    /// Optional file to update card attacks
    #[clap(short, long)]
    attacks: Option<String>,

    /// Optional file to update front character card - NOT YET IMPLEMENTED
    #[clap(short, long)]
    front: Option<String>,

    /// Optional file to update back character card - NOT YET IMPLEMENTED
    #[clap(short, long)]
    back: Option<String>,
}

const CARD_BACK: &str = "https://i.imgur.com/igYZhPh.png";
const S1: &str        = "https://i.imgur.com/PFHTq9B.jpg";
const S2: &str        = "https://i.imgur.com/jWGO1lx.jpg";
const S3: &str        = "https://i.imgur.com/tXuqP40.jpg";
const S4: &str        = "https://i.imgur.com/TuFOJVQ.jpg";


fn isNormal(link: &str) -> bool {
    match link {
        S1 => true,
        S2 => true,
        S3 => true,
        S4 => true,
        _  => false
    }
}

fn main() {
    let args = Args::parse();
    let orig_json = read_to_string(args.file).unwrap();

    let mut orig_deck = json::parse(&orig_json).unwrap();

    for (_card, value) in orig_deck["ObjectStates"][0]["CustomDeck"].entries_mut() {
        if value["BackURL"] == CARD_BACK {
            match &value["FaceURL"] {
                json::JsonValue::String(url) => if !isNormal(&url) {
                    // If updating the card attacks, replace
                    match &args.attacks {
                        Some(s) => value["FaceURL"] = json::JsonValue::String(s.to_string()),
                        _ => (),
                    }
                },
                _ => continue
            };
        }
    }

    for card in orig_deck["ObjectStates"][0]["ContainedObjects"].members_mut() {
        for (_id, value) in card["CustomDeck"].entries_mut() {
            if value["BackURL"] == CARD_BACK {
                match &value["FaceURL"] {
                    json::JsonValue::String(url) => if !isNormal(&url) {
                        // updating card attacks
                        match &args.attacks {
                            Some(s) => value["FaceURL"] = json::JsonValue::String(s.to_string()),
                            _ => (),
                        }
                    },
                    _ => continue
                }
            }
        }
    }
 
    let result = json::stringify(orig_deck);

 

    let mut output = File::create(args.newFile).unwrap();
    output.write_all(result.as_bytes()).unwrap();
}
