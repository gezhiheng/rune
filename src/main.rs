use std::io::{self, stdout};

use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{
    Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};

fn main() -> io::Result<()> {
    if std::env::args().any(|arg| arg == "--smoke") {
        println!("rune ready");
        return Ok(());
    }

    let _raw = RawTerm::enter()?;
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0),
        Print("Rune\r\npress q to quit\r\n"),
    )?;

    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            if should_quit(key.code, key.modifiers) {
                break;
            }
        }
    }
    Ok(())
}

fn should_quit(code: KeyCode, modifiers: KeyModifiers) -> bool {
    code == KeyCode::Char('q')
        || (code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL))
}

struct RawTerm;

impl RawTerm {
    fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        if let Err(err) = execute!(stdout(), EnterAlternateScreen) {
            let _ = disable_raw_mode();
            return Err(err);
        }
        Ok(Self)
    }
}

impl Drop for RawTerm {
    fn drop(&mut self) {
        let _ = execute!(stdout(), LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_and_ctrl_c_quit() {
        assert!(should_quit(KeyCode::Char('q'), KeyModifiers::empty()));
        assert!(should_quit(KeyCode::Char('c'), KeyModifiers::CONTROL));
        assert!(!should_quit(KeyCode::Char('c'), KeyModifiers::empty()));
    }
}
