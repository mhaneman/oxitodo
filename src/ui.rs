use crate::app::{App, AppMode};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

pub fn render_todos(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let items: Vec<ListItem> = app
        .todos
        .iter()
        .map(|todo| {
            let status = if todo.completed { "✓" } else { " " };
            let style = if todo.completed {
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::CROSSED_OUT)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("[{}] ", status), style),
                Span::styled(&todo.text, style),
            ]))
        })
        .collect();

    let title = format!(" Todos ({}) ", app.todos.len());
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, area, &mut app.list_state);
}

pub fn render_input(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let input = Paragraph::new(app.input.value()).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" New Todo ")
            .border_style(Style::default().fg(Color::Green)),
    );

    f.render_widget(input, area);

    // Set cursor position
    f.set_cursor_position((area.x + app.input.visual_cursor() as u16 + 1, area.y + 1));
}

pub fn render_help(f: &mut Frame, area: ratatui::layout::Rect) {
    let help_text = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "Navigation:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  ↑/k    - Move up"),
        Line::from("  ↓/j    - Move down"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Actions:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  i      - Add new todo"),
        Line::from("  Space  - Toggle todo completion"),
        Line::from("  d      - Delete selected todo"),
        Line::from("  ?      - Show this help"),
        Line::from("  q      - Quit"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Insert Mode:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  Enter  - Add todo and return to normal mode"),
        Line::from("  Esc    - Cancel and return to normal mode"),
        Line::from(""),
        Line::from("Press ? or Esc to close this help"),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Help ")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);

    let popup_area = centered_rect(60, 80, area);
    f.render_widget(Clear, popup_area);
    f.render_widget(paragraph, popup_area);
}

pub fn render_status_bar(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let mode_text = match app.mode {
        AppMode::Normal => "NORMAL",
        AppMode::Insert => "INSERT",
        AppMode::Help => "HELP",
    };

    let mode_style = match app.mode {
        AppMode::Normal => Style::default().bg(Color::Blue).fg(Color::White),
        AppMode::Insert => Style::default().bg(Color::Green).fg(Color::Black),
        AppMode::Help => Style::default().bg(Color::Cyan).fg(Color::Black),
    };

    let completed_count = app.completed_count();
    let total_count = app.total_count();

    let status_text = if total_count > 0 {
        format!(
            " {} | {}/{} completed | Press ? for help ",
            mode_text, completed_count, total_count
        )
    } else {
        format!(
            " {} | Press 'i' to add your first todo | Press ? for help ",
            mode_text
        )
    };

    let paragraph = Paragraph::new(status_text)
        .style(mode_style)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

pub fn centered_rect(
    percent_x: u16,
    percent_y: u16,
    r: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn ui(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // Create layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),    // Main content
            Constraint::Length(1), // Status bar
        ])
        .split(size);

    match app.mode {
        AppMode::Insert => {
            // Split main area for todos and input
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(5), Constraint::Length(3)])
                .split(chunks[0]);

            render_todos(f, app, main_chunks[0]);
            render_input(f, app, main_chunks[1]);
        }
        AppMode::Help => {
            render_todos(f, app, chunks[0]);
            render_help(f, size);
        }
        AppMode::Normal => {
            render_todos(f, app, chunks[0]);
        }
    }

    // Always render status bar
    render_status_bar(f, app, chunks[1]);
}
