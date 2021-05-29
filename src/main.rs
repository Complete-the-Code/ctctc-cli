use reqwest;
use std::{
    fs::{File, OpenOptions},
    io::{self, prelude::*, BufReader, Write},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering}
    }
};
use text_io::try_read;

fn read_guesses() -> io::Result<Vec<String>> {
    BufReader::new(File::open("guesses.txt")?).lines().collect()
}

fn write_guesses(guesses: Vec<String>) -> io::Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .open("guesses.txt")?;
    for line in guesses {
        if !line.is_empty() {
            write!(f, "{}", format!("{}\n", line.as_str()))?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut guesses: Vec<String> = match read_guesses() {
        Ok(g) => g,
        Err(_) => Vec::new()
    };
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;
    while running.load(Ordering::SeqCst) {
        print!("> ");
        io::stdout().flush()?;
        let mut s: String = try_read!("{}\n")?;
        s = s.trim().to_string();
        if s.is_empty() {
            std::thread::sleep(std::time::Duration::from_millis(50));
            continue
        }
        else if guesses.contains(&s) {
            println!("Already guessed, dipshit.");
            continue
        }

        let resp = client.post("https://completethecodetwo.cards/pw")
            .body(s.to_owned())
            .header("Content-Type", "text/plain")
            .send()
            .await?;

        if resp.status().is_client_error() {
            println!("Nope.");
            guesses.push(s);
        }
        else if resp.status().is_server_error() {
            println!("Someone fucked the server");
        }
        else {
            println!("HOLY SHIT. PASSWORD: {}", &s);
        }
    }
    write_guesses(guesses)?;
    Ok(())
}
