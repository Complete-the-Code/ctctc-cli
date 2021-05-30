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
        std::process::exit(0);
    })?;
    while running.load(Ordering::SeqCst) {
        print!("> ");
        io::stdout().flush()?;

        let mut password_try: String = try_read!("{}\n")?;
        password_try = password_try.trim().to_string();

        if password_try.is_empty() {
            std::thread::sleep(std::time::Duration::from_millis(50));
            continue
        } else if guesses.contains(&password_try) {
            println!("Already guessed, dipshit.");
            continue
        }

        let resp = client.post("https://completethecodetwo.cards/pw")
            .body(password_try.to_owned())
            .header("Content-Type", "text/plain")
            .send()
            .await?;

        match resp.status() {
            denied_status if denied_status.as_u16() == 403 => {
                println!("Nope. 403 Forbidden");
                guesses.push(password_try);
                write_guesses(guesses.to_owned())?;
            },
            success_status if success_status.is_success() => {
                println!("HOLY SHIT. PASSWORD: {}", &password_try);
            },
            _other_status => {
                let mut _other_code_string = String::new();
                let code_msg: &str = match resp.status().as_u16() {
                    400 => "400 Bad Request",
                    401 => "401 Unauthorized",
                    404 => "404 Not Found",
                    405 => "405 Method Not Allowed",
                    408 => "408 Request Timeout",
                    418 => "418 I'm a teapot",
                    429 => "429 Too Many Requests",
                    500 => "500 Internal Server Error",
                    501 => "501 Not Implemented",
                    503 => "503 Service Unavailable",
                    504 => "504 Gateway Timeout",
                    other_code => {
                        _other_code_string = format!("Some other code: {}", other_code);
                        &_other_code_string as &str
                    },
                };
                println!("The server returned an unexpected response, this could mean that you found the password or that something's fucked up.\n\
                Error Message: {}\n\
                Tried Password. {}", code_msg, &password_try)
            }
        }
    }
    Ok(())
}
