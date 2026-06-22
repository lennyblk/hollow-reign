use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    style::{Color, Print, ResetColor, SetForegroundColor},
    execute,
};
use std::io::{self, Write};

pub fn check_and_update() {
    let mut out = io::stdout();
    execute!(
        out,
        SetForegroundColor(Color::DarkGrey),
        Print("  Verification des mises a jour..."),
        ResetColor
    )
    .ok();
    out.flush().ok();

    let update = match self_update::backends::github::Update::configure()
        .repo_owner("lennyblk")
        .repo_name("hollow-reign")
        .bin_name("hollow-reign")
        .current_version(env!("CARGO_PKG_VERSION"))
        .no_confirm(true)
        .build()
    {
        Ok(u) => u,
        Err(_) => {
            println!("\r");
            return;
        }
    };

    let release = match update.get_latest_release() {
        Ok(r) => r,
        Err(_) => {
            println!("\r");
            return;
        }
    };

    let is_newer = self_update::version::bump_is_greater(
        env!("CARGO_PKG_VERSION"),
        &release.version,
    )
    .unwrap_or(false);

    if !is_newer {
        execute!(io::stdout(), Print(" OK\r\n"), ResetColor).ok();
        return;
    }

    execute!(
        io::stdout(),
        Print(format!(
            "\r\n  Nouvelle version : v{} (actuelle : v{})\r\n",
            release.version,
            env!("CARGO_PKG_VERSION")
        )),
        SetForegroundColor(Color::DarkYellow),
        Print("  Mettre a jour maintenant ? [O/n] "),
        ResetColor,
    )
    .ok();
    io::stdout().flush().ok();

    crossterm::terminal::enable_raw_mode().ok();
    let answer = loop {
        if let Ok(Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        })) = event::read()
        {
            match code {
                KeyCode::Char('o') | KeyCode::Char('O') | KeyCode::Enter => break true,
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => break false,
                _ => {}
            }
        }
    };
    crossterm::terminal::disable_raw_mode().ok();

    if !answer {
        println!("\r\n  Mise a jour ignoree.\r");
        return;
    }

    println!("\r\n  Telechargement...\r");
    match update.update() {
        Ok(_) => {
            println!("  Mise a jour reussie. Relancez le jeu.\r");
            println!("  Appuyez sur une touche...\r");
            crossterm::terminal::enable_raw_mode().ok();
            loop {
                if let Ok(Event::Key(KeyEvent {
                    kind: KeyEventKind::Press,
                    ..
                })) = event::read()
                {
                    break;
                }
            }
            crossterm::terminal::disable_raw_mode().ok();
            std::process::exit(0);
        }
        Err(e) => {
            println!("  Echec de la mise a jour : {}\r", e);
        }
    }
}
