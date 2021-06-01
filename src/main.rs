mod app;
mod event;
mod ui;

use crate::app::App;
use crate::event::{Event, Events};
use crossterm::{
    event::{
        Event as CEvent,
        KeyCode,
        KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use reqwest;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, prelude::*, stdout, BufReader, Write},
};
use tui::{backend::CrosstermBackend, Terminal};

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

async fn request(client: reqwest::Client, app: &mut App)
    -> Result<(), Box<dyn Error>> {
    let guess: String = app.input.drain(..).collect();
    if app.guesses.contains(&guess) {
        app.last_return = "Already guessed, dipshit.".to_string();
        return Ok(())
    }
    let resp = client.post("https://completethecodetwo.cards/pw")
        .body(guess.to_owned())
        .header("Content-Type", "text/plain")
        .send()
        .await?;



    app.return_code = resp.status().as_u16();
    let code_str = resp.status().as_str().to_string();

    let other_code = format!("No fucking clue. {}", code_str);
    let code_msg: &str = match resp.status().as_u16() {
        400 => "Nope. 400 Bad Request",
        401 => "Nope. 401 Unauthorized",
        403 => "Nope. 403 Forbidden",
        404 => "Nope. 404 Not Found",
        405 => "Nope. 405 Method Not Allowed",
        408 => "Nope. 408 Request Timeout",
        418 => "The fuck? 418 I'm a teapot",
        429 => "You fucked up. 429 Too Many Requests",
        500 => "It's dead, Jim. 500 Internal Server Error",
        501 => "The fuck? 501 Not Implemented",
        503 => "It's dead, Jim. 503 Service Unavailable",
        504 => "It's dead, Jim. 504 Gateway Timeout",
        _ => other_code.as_str(),
    };

    app.last_return = code_msg.to_string();

    if resp.status().is_client_error() {
        app.guesses.push(guess);
        write_guesses(app.guesses.to_owned())?;
    }
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let guesses: Vec<String> = match read_guesses() {
        Ok(g) => g,
        Err(_) => Vec::new()
    };

    enable_raw_mode()?;
    let stdout = stdout();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();
    let mut app = App::new(guesses);

    terminal.clear()?;

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        if let Event::Input(input) = events.next()? {
            match input {
                CEvent::Key(k) => {
                    match k.code {
                        KeyCode::Enter => {
                            request(client.to_owned(), &mut app).await?;
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Char(c) => {
                            if c == 'c' && k.modifiers.contains(KeyModifiers::CONTROL) {
                                break;
                            }
                            else {
                                app.input.push(c);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    terminal.clear()?;

    disable_raw_mode()?;
    Ok(())
}
