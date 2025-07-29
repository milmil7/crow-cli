use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    text::{Text, Line, Span},
    style::{Style, Color, Modifier},
};
use tokio::sync::mpsc;
use std::time::Duration;
use crossterm::event::KeyEventKind;
use ratatui::prelude::Alignment;
use ratatui::widgets::Wrap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, mut rx) = mpsc::channel(1);

    let mut input_url = String::new();
    let mut input_headers = String::new();
    let mut input_params = String::new();
    let mut input_body = String::new();
    let mut input_auth = String::new();
    let mut response = String::from("Enter a URL and press Send");

    let mut response_scroll = 0u16;
    let mut body_scroll = 0u16;

    let methods = vec!["GET", "POST", "PUT", "DELETE", "PATCH"];
    let auth_types = vec!["None", "Bearer", "Basic"];
    let mut current_method_index = 0;
    let mut current_auth_index = 0;

    let mut field_focus = 0; 

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(7),
                    Constraint::Min(1),
                ])
                .split(f.size());

            
            let row1 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(14),
                    Constraint::Min(1),
                    Constraint::Length(10),
                ])
                .split(chunks[0]);

            let method_box = Paragraph::new(methods[current_method_index])
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("┤ Method ├")
                        .style(Style::default().fg(Color::Magenta)),
                )
                .style(Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD));

            let url_box = Paragraph::new(input_url.as_str())
                .block(Block::default().borders(Borders::ALL).title("┤ URL ├"))
                .style(style_for_focus(field_focus == 0));

            let send_box = Paragraph::new("[ Send ]")
                .block(Block::default().borders(Borders::ALL))
                .style(style_for_focus(field_focus == 6));

            f.render_widget(method_box, row1[0]);
            f.render_widget(url_box, row1[1]);
            f.render_widget(send_box, row1[2]);

            
            let row2 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);

            let params_box = Paragraph::new(input_params.as_str())
                .block(Block::default().borders(Borders::ALL).title("┤ Params ├"))
                .style(style_for_focus(field_focus == 1));

            let auth_box = Paragraph::new(auth_types[current_auth_index])
                .block(Block::default().borders(Borders::ALL).title("┤ Auth ├"))
                .style(style_for_focus(field_focus == 2));

            f.render_widget(params_box, row2[0]);
            f.render_widget(auth_box, row2[1]);

            
            let row3 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[2]);

            let auth_input_box = Paragraph::new(input_auth.as_str())
                .block(Block::default().borders(Borders::ALL).title("┤ Token/Username:Password ├"))
                .style(style_for_focus(field_focus == 3));

            let headers_box = Paragraph::new(input_headers.as_str())
                .block(Block::default().borders(Borders::ALL).title("┤ Headers ├"))
                .style(style_for_focus(field_focus == 4));

            f.render_widget(auth_input_box, row3[0]);
            f.render_widget(headers_box, row3[1]);

            
            let body_highlighted = highlight_json(&input_body);
            let body_box = Paragraph::new(body_highlighted)
                .block(Block::default().borders(Borders::ALL).title("┤ Body ├"))
                .style(style_for_focus(field_focus == 5))
                .scroll((body_scroll, 0));

            f.render_widget(body_box, chunks[3]);

            
            let response_highlighted = highlight_json(&response);
            let response_box = Paragraph::new(response_highlighted)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("╢ Response (↑/↓ to scroll) ╟")
                        .style(Style::default().fg(Color::Blue)),
                )
                .wrap(ratatui::widgets::Wrap { trim: false })
                .scroll((response_scroll, 0));
            let help_text = Span::styled(
                "TAB: Next  |  SHIFT+TAB: Back  |  ENTER: Send  |  ESC: Quit  |  ↑↓: Scroll  |  ←→: Switch Field",
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
            );
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            f.render_widget(response_box, chunks[4]);
            f.render_widget(help_paragraph, chunks[4]);
        })?;

        if let Ok(Some(new_resp)) = rx.try_recv() {
            response = new_resp;
            response_scroll = 0;
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => match field_focus {
                            0 => input_url.push(c),
                            1 => input_params.push(c),
                            3 => input_auth.push(c),
                            4 => input_headers.push(c),
                            5 => input_body.push(c),
                            _ => {}
                        },
                        KeyCode::Enter => {
                            if field_focus == 5 {
                                input_body.push('\n');
                            } else if field_focus == 6 {
                                let url = input_url.clone();
                                let headers = input_headers.clone();
                                let body = input_body.clone();
                                let params = input_params.clone();
                                let method = methods[current_method_index].to_string();
                                let auth_type = auth_types[current_auth_index].to_string();
                                let auth_input = input_auth.clone();
                                let tx = tx.clone();
                                tokio::spawn(async move {
                                    let res = send_request(url, method, headers, params, body, auth_type, auth_input).await;
                                    let _ = tx.send(Some(res)).await;
                                });
                            }
                        }
                        KeyCode::Backspace => match field_focus {
                            0 => { input_url.pop(); }
                            1 => { input_params.pop(); }
                            3 => { input_auth.pop(); }
                            4 => { input_headers.pop(); }
                            5 => { input_body.pop(); }
                            _ => {}
                        },
                        KeyCode::Tab => {
                            field_focus = (field_focus + 1) % 7;
                        }
                        KeyCode::Left => {
                            if field_focus == 2 && current_auth_index > 0 {
                                current_auth_index -= 1;
                            } else if field_focus == 0 && current_method_index > 0 {
                                current_method_index -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if field_focus == 2 && current_auth_index < auth_types.len() - 1 {
                                current_auth_index += 1;
                            } else if field_focus == 0 && current_method_index < methods.len() - 1 {
                                current_method_index += 1;
                            }
                        }
                        KeyCode::Up => {
                            if field_focus == 5 {
                                body_scroll = body_scroll.saturating_sub(1);
                            } else {
                                response_scroll = response_scroll.saturating_sub(1);
                            }
                        }
                        KeyCode::Down => {
                            if field_focus == 5 {
                                body_scroll = body_scroll.saturating_add(1);
                            } else {
                                response_scroll = response_scroll.saturating_add(1);
                            }
                        }
                        KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn style_for_focus(focused: bool) -> Style {
    if focused {
        Style::default()
            .fg(Color::White)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    }
}

fn highlight_json(json: &str) -> Text {
    let mut lines = Vec::new();
    for line in json.lines() {
        let mut spans = Vec::new();
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '"' => {
                    let mut s = String::from("\"");
                    while let Some(nc) = chars.next() {
                        s.push(nc);
                        if nc == '"' {
                            break;
                        }
                    }
                    spans.push(Span::styled(s, Style::default().fg(Color::Yellow)));
                }
                ':' => spans.push(Span::styled(":", Style::default().fg(Color::Gray))),
                '{' | '}' | '[' | ']' => spans.push(Span::styled(c.to_string(), Style::default().fg(Color::Cyan))),
                _ if c.is_numeric() => {
                    let mut num = c.to_string();
                    while let Some(nc) = chars.peek() {
                        if nc.is_numeric() || *nc == '.' {
                            num.push(*nc);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    spans.push(Span::styled(num, Style::default().fg(Color::Green)));
                }
                _ => spans.push(Span::raw(c.to_string())),
            }
        }
        lines.push(Line::from(spans));
    }
    Text::from(lines)
}

async fn send_request(
    mut url: String,
    method: String,
    headers: String,
    params: String,
    body: String,
    auth_type: String,
    auth_input: String,
) -> String {
    if !params.trim().is_empty() {
        if url.contains('?') {
            url.push('&');
        } else {
            url.push('?');
        }

        let formatted_params = params
            .split(',')
            .filter_map(|pair| pair.split_once(':'))
            .map(|(k, v)| format!("{}={}", k.trim(), v.trim()))
            .collect::<Vec<_>>()
            .join("&");

        url.push_str(&formatted_params);
    }

    let client = reqwest::Client::new();
    let mut req = match method.as_str() {
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        "PATCH" => client.patch(&url),
        _ => client.get(&url),
    };

    
    match auth_type.as_str() {
        "Bearer" => {
            req = req.bearer_auth(auth_input.trim());
        }
        "Basic" => {
            if let Some((user, pass)) = auth_input.trim().split_once(':') {
                req = req.basic_auth(user.trim(), Some(pass.trim()));
            }
        }
        _ => {}
    }

    for pair in headers.split(',') {
        if let Some((key, value)) = pair.split_once(':') {
            req = req.header(key.trim(), value.trim());
        }
    }

    if ["POST", "PUT", "PATCH"].contains(&method.as_str()) && !body.is_empty() {
        req = req.body(body);
    }

    match req.send().await {
        Ok(resp) => {
            let status = resp.status();
            match resp.text().await {
                Ok(body) => {
                    let formatted = serde_json::from_str::<serde_json::Value>(&body)
                        .map(|v| serde_json::to_string_pretty(&v).unwrap_or(body.clone()))
                        .unwrap_or(body);
                    format!("Status: {}\n\n{}", status, formatted)
                }
                Err(err) => format!("Error reading body: {}", err),
            }
        }
        Err(err) => format!("Request failed: {}", err),
    }
}
    