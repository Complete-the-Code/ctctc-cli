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
        ::std::process::exit(0);
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

        let code_str = resp.status().as_u16();
        let other_code = format!("Some other code: {}", code_str);
        let code_msg: &str = match resp.status().as_u16() {
            400 => "400 Bad Request",
            401 => "401 Unauthorized",
            403 => "403 Forbidden",
            404 => "404 Not Found",
            405 => "405 Method Not Allowed",
            408 => "408 Request Timeout",
            418 => "418 I'm a teapot",
            429 => "429 Too Many Requests",
            500 => "500 Internal Server Error",
            501 => "501 Not Implemented",
            503 => "503 Service Unavailable",
            504 => "504 Gateway Timeout",
            _ => other_code.as_str(),
        };
        if resp.status().is_client_error() {
            println!("Nope. ({})", &code_msg);
            guesses.push(s);
            write_guesses(guesses)?;
        }
        else if resp.status().is_server_error() {

            println!("Someone fucked the server. ({})", &code_msg);
        }
        else {
            println!("HOLY SHIT. PASSWORD: {}", &s);
        }
    }
    Ok(())
}
