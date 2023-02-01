mod server;

use crossterm::{
    cursor, event::{self, KeyCode}, queue, style,
    style::Attributes,
    style::Attribute,
    style::Color::*,
    terminal,
};
use std::io::Write;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    // server initalization
    let mut server = server::Server::new();

    // terminal initalization
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode().unwrap();
    queue!(
        stdout,
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All)
    )
    .unwrap();

    // set up text box
    let mut current_text = String::new();

    draw_text_box("");

    // main input loop
    'main: loop {
        while event::poll(Duration::from_secs(0)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break 'main, // use `q` to quit right now
                        KeyCode::Char(ch) => {
                            current_text.push(ch);
                        }
                        KeyCode::Backspace => {
                            current_text.pop();
                        }
                        KeyCode::Enter => {
                            server.send_message(&current_text);
                        }
                        _ => {},
                    }
                    draw_text_box(&current_text);
                }
            }
        }
        sleep(Duration::from_millis(20)); // sleep so that the process doesn't eat up cpu
    }

    // put terminal back to normal
    queue!(stdout, terminal::LeaveAlternateScreen).unwrap();
}

/// Draws an input box at the bottom of the screen, containing the provided `text`. It returns the number of lines drawn.
fn draw_text_box(text: &str) -> usize {
    let mut stdout = std::io::stdout();
    let terminal_size = terminal::size().unwrap();

    let n_lines = (text.len() / terminal_size.0 as usize) + 1;

    // queue!(stdout, style::SetColors(style::Colors::new(Black, White))).unwrap();
    queue!(stdout, style::SetAttribute(Attribute::Reverse)).unwrap();

    let lines = wrap_text(text, terminal_size.0 as usize);

    for i in 0..n_lines {
        queue!(
            stdout,
            cursor::MoveTo(0, (terminal_size.1 - n_lines as u16) + i as u16)
        ).unwrap();
        // stdout.flush().unwrap();
        // stdout.write_all(format!("{1:<0$}", terminal_size.0 as usize, &text[(terminal_size.0 as usize * i)..terminal_size.0 as usize * (i + 1)]).as_bytes()).unwrap();
        stdout.write_all(
            format!(
                 "{1:<0$}",
                terminal_size.0 as usize,
                &text[lines[i]..lines[i + 1]]
            )
            .as_bytes(),
        ).unwrap();
    }

    queue!(stdout, style::SetAttribute(Attribute::NoReverse)).unwrap();

    stdout.flush().unwrap();

    return n_lines;
}

/// Returns the points at which to split the text in order to make it fit within `line_length`. Both the start of the first line and the end of the last line are included in the result.
/// # Example
/// ```
/// let result = wrap_text(&text, 20);
/// for i in 0..result.len() - 1 {
///     println!("{}", &text[lines[i]..lines[i + 1]])
/// }
/// ```
fn wrap_text(text: &str, line_length: usize) -> Vec<usize> {
    let n_breaks = text.chars().count() / line_length;
    let mut result: Vec<usize> = Vec::with_capacity(n_breaks + 2);
    result.extend((0..=n_breaks).map(|i| i * line_length));
    result.push(text.chars().count());
    return result;
}
