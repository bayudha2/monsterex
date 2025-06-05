use std::io::{stdout, Stdout};

use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{
            disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, EnterAlternateScreen,
            LeaveAlternateScreen,
        },
    },
    Terminal,
};

// pub type Tui = Terminal<CrosstermBackend<Stdout>>;
pub struct Tui {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn init() -> Result<Self> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout());
        Ok(Self {
            terminal: Terminal::new(backend)?,
        })
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        if is_raw_mode_enabled().unwrap() {
            let _ = execute!(
                self.terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            );
            let _ = disable_raw_mode();
            let _ = self.terminal.show_cursor();
        }
    }
}
