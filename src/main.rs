mod app;
mod events;
mod todo;
mod ui;

use app::App;
use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use events::EventHandler;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use ui::ui;

fn main() -> Result<()> {
    // Initialize error handling
    color_eyre::install()?;

    // Setup terminal
    let mut terminal = setup_terminal()?;

    // Create app and event handler
    let mut app = App::new()?;
    let event_handler = EventHandler::new();

    // Run the main application loop
    let result = run_app(&mut terminal, &mut app, &event_handler);

    // Restore terminal
    restore_terminal(&mut terminal)?;

    // Handle any errors that occurred during app execution
    if let Err(err) = result {
        eprintln!("Application error: {}", err);
        return Err(err);
    }

    println!("Thanks for using the todo app!");
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    event_handler: &EventHandler,
) -> Result<()> {
    loop {
        // Draw the UI
        terminal.draw(|f| ui(f, app))?;

        // Handle events
        event_handler.handle_events(app)?;

        // Check if we should quit
        if app.should_quit {
            break;
        }
    }
    Ok(())
}
