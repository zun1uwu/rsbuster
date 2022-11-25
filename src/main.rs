#![feature(str_split_as_str)]

use std::{env, fs::File, io::Read, ops::Index, process};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().into_iter().collect();

    if args.len() < 3 {
        println!("Usage: 'rsbuster [target domain] [wordlist path]'");
        process::exit(0);
    }

    let url = args.index(1);
    let wordlist = args.index(2);

    let mut file = File::open(wordlist)?;
    let mut buffer: [u8; 128] = [0; 128];
    File::read(&mut file, &mut buffer)?;
    let content = String::from_utf8(buffer.to_vec())?;
    let lines = content.lines();

    for line in lines {
        let url = format!("http://{}/{}", url, line);
        let request = reqwest::get(&url).await?;

        if request.status().to_string().starts_with("200") {
            println!("Found directory: {}", &url)
        }
    }

    Ok(())
}
