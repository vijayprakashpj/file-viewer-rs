use ratatui::{
    crossterm::{
        self,
        event::{Event, KeyCode, KeyEventKind},
    },
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
};

struct FileViewerState {
    file_path: String,
    content: String,
    line_count: usize,
    scroll_offset: usize,
}

impl FileViewerState {
    pub fn new(file_path: &str) -> Self {
        // Assume the file can be read as UTF-8 string
        let content = std::fs::read_to_string(file_path).unwrap_or_default();
        let line_count = content.lines().count();
        Self {
            file_path: file_path.to_string(),
            content,
            line_count,
            scroll_offset: 0,
        }
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset = match self.scroll_offset.saturating_sub(1) {
                0 => 0,
                offset => offset,
            };
        }
    }

    pub fn scroll_down(&mut self) {
        if self.scroll_offset < self.line_count {
            self.scroll_offset = match self.scroll_offset.saturating_add(1) {
                offset if offset >= self.line_count => self.line_count.saturating_sub(1),
                offset => offset,
            };
        }
    }

    pub fn get_file_content(&self) -> (&str, usize) {
        (&self.content, self.line_count)
    }

    pub fn get_file_name(&self) -> &str {
        self.file_path
            .rsplit_once('/')
            .map_or(&self.file_path, |(_, name)| name)
    }

    fn get_scroll_offset(&self) -> u16 {
        self.scroll_offset as u16
    }
}

impl Default for FileViewerState {
    fn default() -> Self {
        Self {
            file_path: String::new(),
            content: String::new(),
            line_count: 0,
            scroll_offset: 0,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let file_path = std::env::args().nth(1).unwrap_or_default();
    let mut state = FileViewerState::new(&file_path);

    loop {
        terminal.clear()?;
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(5),
                    Constraint::Length(3),
                ])
                .split(frame.area());

            let title = Paragraph::new(format!("File Viewer: {}", state.get_file_name()))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(title, chunks[0]);

            let (content, line_count) = state.get_file_content();
            let content = Paragraph::new(content)
                .block(Block::default().borders(Borders::ALL))
                .scroll((state.get_scroll_offset(), 0));
            frame.render_widget(content, chunks[1]);

            let footer_layout = Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(10),
                    Constraint::Percentage(70),
                    Constraint::Percentage(20),
                ])
                .split(chunks[2]);

            let version_text = Paragraph::new("q - Quit")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(version_text, footer_layout[0]);

            let footer_text = Paragraph::new("Footer")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(footer_text, footer_layout[1]);

            let file_line_count_text = Paragraph::new(format!("{} lines", line_count))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(file_line_count_text, footer_layout[2]);
        })?;

        if let Some(event) = crossterm::event::read().ok() {
            if let Event::Key(key) = event
                && key.kind == KeyEventKind::Press
            {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => state.scroll_up(),
                    KeyCode::Down => state.scroll_down(),
                    _ => {}
                }
            }
        }
    }

    ratatui::restore();

    Ok(())
}
