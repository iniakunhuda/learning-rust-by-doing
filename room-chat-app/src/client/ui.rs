use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use tokio::sync::mpsc;
use crate::common::{ChatError, Message};

pub struct UI {
    messages: Vec<Message>,
    input: String,
    current_room: String,
    tx: mpsc::Sender<Message>,
}

impl UI {
    pub fn new(tx: mpsc::Sender<Message>) -> Self {
        UI {
            messages: Vec::new(),
            input: String::new(),
            current_room: "lobby".to_string(),
            tx,
        }
    }

    pub async fn run(&mut self) -> Result<(), ChatError> {
        // Terminal initialization
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_app(&mut terminal).await;

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    async fn run_app<B: tui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), ChatError> {
        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(1),  // Room name
                            Constraint::Min(1),     // Messages
                            Constraint::Length(3),  // Input
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                // Room name
                let room_name = Paragraph::new(format!("Room: {}", self.current_room))
                    .style(Style::default().fg(Color::Yellow));
                f.render_widget(room_name, chunks[0]);

                // Messages
                let messages: Vec<ListItem> = self
                    .messages
                    .iter()
                    .map(|m| {
                        let time = format!("{:02}:{:02}", 
                            m.timestamp.elapsed().unwrap().as_secs() / 3600,
                            (m.timestamp.elapsed().unwrap().as_secs() % 3600) / 60);
                        let content = format!("{} | {} > {}", time, m.sender, m.content);
                        ListItem::new(vec![Spans::from(vec![
                            Span::styled(content, Style::default().fg(Color::White)),
                        ])])
                    })
                    .collect();

                let messages = List::new(messages)
                    .block(Block::default().borders(Borders::ALL).title("Messages"))
                    .style(Style::default().fg(Color::White));
                f.render_widget(messages, chunks[1]);

                // Input
                let input = Paragraph::new(self.input.as_ref())
                    .style(Style::default())
                    .block(Block::default().borders(Borders::ALL).title("Input"));
                f.render_widget(input, chunks[2]);
            })?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Enter => {
                            if !self.input.is_empty() {
                                let msg = Message::new(
                                    self.current_room.clone(),
                                    "you".to_string(),
                                    self.input.clone(),
                                );
                                self.tx.send(msg).await.map_err(|e| ChatError {
                                    kind: crate::common::ChatErrorKind::Message,
                                    message: e.to_string(),
                                })?;
                                self.input.clear();
                            }
                        }
                        KeyCode::Char(c) => {
                            self.input.push(c);
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    pub fn add_message(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    pub fn set_room(&mut self, room: String) {
        self.current_room = room;
    }
}