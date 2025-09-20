use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};
// Using standard colors for now

use crate::typing::TypingTest;

pub fn render(f: &mut Frame, typing_test: &TypingTest) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Title
                Constraint::Length(3), // Instructions
                Constraint::Min(6),    // Text area
                Constraint::Length(3), // Progress
                Constraint::Length(3), // Stats
            ]
            .as_ref(),
        )
        .split(f.size());

    // Title
    let title = Paragraph::new("Typing Test CLI")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Magenta)));
    f.render_widget(title, chunks[0]);

    // Instructions
    let instructions = create_instructions(typing_test);
    f.render_widget(instructions, chunks[1]);

    // Text display with highlighting
    let text_area = create_text_display(typing_test);
    f.render_widget(text_area, chunks[2]);

    // Progress bar
    let progress = create_progress_bar(typing_test);
    f.render_widget(progress, chunks[3]);

    // Stats
    let stats = create_stats_display(typing_test);
    f.render_widget(stats, chunks[4]);
}

fn create_instructions(typing_test: &TypingTest) -> Paragraph<'_> {
    let instruction_text = if typing_test.show_results() {
        "Press 'r' to restart or 'q' to quit"
    } else if typing_test.is_test_started() {
        "Type the text below (Backspace to correct)"
    } else {
        "Press Enter to start typing test"
    };

    let style = if typing_test.show_results() {
        Style::default().fg(Color::Green)
    } else if typing_test.is_test_started() {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Magenta)
    };

    Paragraph::new(instruction_text)
        .style(style)
        .alignment(Alignment::Center)
}

fn create_text_display(typing_test: &TypingTest) -> Paragraph<'_> {
    let mut spans = Vec::new();
    
    for (i, char) in typing_test.target_text().chars().enumerate() {
        let style = if i < typing_test.user_input().len() {
            if typing_test.user_input().chars().nth(i) == Some(char) {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            }
        } else if i == typing_test.current_char() {
            Style::default().bg(Color::Yellow).fg(Color::Black)
        } else {
            Style::default().fg(Color::White)
        };
        
        spans.push(Span::styled(char.to_string(), style));
    }
    
    Paragraph::new(Line::from(spans))
        .block(Block::default().title("Text to Type").borders(Borders::ALL).border_style(Style::default().fg(Color::Magenta)))
        .wrap(Wrap { trim: true })
}

fn create_progress_bar(typing_test: &TypingTest) -> Gauge<'_> {
    let progress_ratio = if typing_test.total_chars() > 0 {
        typing_test.current_char() as f64 / typing_test.total_chars() as f64
    } else {
        0.0
    };

    Gauge::default()
        .block(Block::default().title("Progress").borders(Borders::ALL).border_style(Style::default().fg(Color::Magenta)))
        .gauge_style(Style::default().fg(Color::Magenta))
        .ratio(progress_ratio)
}

fn create_stats_display(typing_test: &TypingTest) -> Paragraph<'_> {
    let stats_text = if typing_test.show_results() {
        format!(
            "🎉 Test Complete! | WPM: {} | Accuracy: {:.1}% | Correct: {}/{}",
            typing_test.wpm(),
            typing_test.accuracy(),
            typing_test.correct_chars(),
            typing_test.total_chars()
        )
    } else {
        "Ready to start typing test...".to_string()
    };
    
    Paragraph::new(stats_text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default().title("Statistics").borders(Borders::ALL).border_style(Style::default().fg(Color::Magenta)))
}
