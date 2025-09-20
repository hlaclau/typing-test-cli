use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

use crate::typing::TypingTest;

pub struct App {
    typing_test: TypingTest,
}

impl App {
    pub fn new() -> Self {
        Self {
            typing_test: TypingTest::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Setup terminal
        enable_raw_mode().map_err(|e| {
            eprintln!("Warning: Could not enable raw mode: {}. Running in fallback mode.", e);
            e
        })?;
        
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).map_err(|e| {
            eprintln!("Warning: Could not enter alternate screen: {}. Running in fallback mode.", e);
            e
        })?;
        
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Run the main loop
        let res = self.run_app(&mut terminal);

        // Restore terminal
        let _ = disable_raw_mode();
        let _ = execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = terminal.show_cursor();

        if let Err(err) = res {
            println!("{err:?}");
        }

        Ok(())
    }

    fn run_app(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        loop {
            terminal.draw(|f| crate::ui::render(f, &self.typing_test))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('r') if self.typing_test.show_results() => {
                            self.typing_test.reset();
                        }
                        KeyCode::Enter if !self.typing_test.is_test_started() => {
                            self.typing_test.start_test();
                        }
                        KeyCode::Char(c) if self.typing_test.is_test_started() && !self.typing_test.is_test_completed() => {
                            self.typing_test.add_char(c);
                        }
                        KeyCode::Backspace if self.typing_test.is_test_started() && !self.typing_test.is_test_completed() => {
                            self.typing_test.remove_char();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
