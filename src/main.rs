mod server;

use crossterm::{
    cursor, event, queue, style,
    style::Color::{Black, White},
    terminal,
};
use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode().unwrap();
    queue!(stdout, terminal::EnterAlternateScreen, terminal::Clear(terminal::ClearType::All)).unwrap();

    let mut current_text = String::new();

    draw_text_box("");

    loop {
        match event::read().unwrap() {
            event::Event::Key(key) => {
                if let event::KeyCode::Char('q') = key.code {
                    break;
                } else if let event::KeyCode::Char(c) = key.code {
                    current_text.push(c);
                } else if event::KeyCode::Backspace == key.code {
                    current_text.pop();
                }
                draw_text_box(&current_text);
            }
            _ => {}
        }
    }
    queue!(stdout, terminal::LeaveAlternateScreen).unwrap();
}

fn draw_text_box(text: &str) {
    let mut stdout = std::io::stdout();
    let terminal_size = terminal::size().unwrap();

    let n_lines = text.len() / terminal_size.0 as usize;

    queue!(stdout, style::SetColors(style::Colors::new(Black, White))).unwrap();
    // stdout.write_all(&b" ".repeat(terminal_size.0 as usize - 1)).unwrap();
    // stdout.flush().unwrap();
    // execute!(stdout, cursor::MoveToColumn(0)).unwrap();

    let lines = wrap_text(text, terminal_size.0 as usize);

    for i in 0..n_lines + 1 {
        queue!(
            stdout,
            cursor::MoveTo(0, (terminal_size.0 - n_lines as u16) + i as u16)
        ).unwrap();
        // stdout.write_all(format!("{1:<0$}", terminal_size.0 as usize, &text[(terminal_size.0 as usize * i)..terminal_size.0 as usize * (i + 1)]).as_bytes()).unwrap();
        stdout.write_all(
            format!(
                "{1:<0$}",
                terminal_size.0 as usize,
                &text[lines[i]..lines[i+1]]
            )
            .as_bytes(),
        ).unwrap();
    }
    stdout.flush().unwrap();
}

fn wrap_text(text: &str, line_length: usize) -> Vec<usize> {
    let n_breaks = text.len() / line_length;
    let mut result: Vec<usize> = Vec::with_capacity(n_breaks + 2);
    result.extend((0..=n_breaks).map(|i| i * line_length));
    result.push(text.len());
    return result;
}
