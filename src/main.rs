// main.rs
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor, SetAttribute, Attribute},
};
use std::io::{self, Write};
use std::time::Duration;

mod ui;
use ui::handler::{Page, PageContent};
use ui::header::draw_header;

struct Portfolio {
    current_page: Page,
    scroll_offset: u16,
    terminal_height: u16,
    terminal_width: u16,
}

impl Portfolio {
    fn new() -> Self {
        let (width, height) = crossterm::terminal::size().unwrap_or((80, 24));
        Self {
            current_page: Page::Home,
            scroll_offset: 0,
            terminal_height: height,
            terminal_width: width,
        }
    }

    fn render(&mut self, stdout: &mut io::Stdout) -> crossterm::Result<()> {
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        
        // Draw header
        let header_height = draw_header(stdout, 0, 0, &self.current_page)?;
        
        // Draw page content
        let content_start_y = header_height + 1;
        self.draw_content(stdout, content_start_y)?;
        
        // Draw footer with instructions
        self.draw_footer(stdout)?;
        
        stdout.flush()?;
        Ok(())
    }

    fn draw_content(&self, stdout: &mut io::Stdout, start_y: u16) -> crossterm::Result<()> {
        let content = PageContent::get_content(&self.current_page);
        let max_height = self.terminal_height.saturating_sub(start_y + 3);
        
        for (i, line) in content.lines().enumerate().skip(self.scroll_offset as usize) {
            if i >= (self.scroll_offset as usize + max_height as usize) {
                break;
            }
            
            let y = start_y + (i - self.scroll_offset as usize) as u16;
            execute!(stdout, cursor::MoveTo(2, y))?;
            
            // Parse and render colored lines
            if line.starts_with("# ") {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Cyan),
                    SetAttribute(Attribute::Bold),
                    Print(&line[2..]),
                    SetAttribute(Attribute::Reset),
                    ResetColor
                )?;
            } else if line.starts_with("## ") {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Green),
                    SetAttribute(Attribute::Bold),
                    Print(&line[3..]),
                    SetAttribute(Attribute::Reset),
                    ResetColor
                )?;
            } else if line.starts_with("- ") {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Yellow),
                    Print("● "),
                    SetForegroundColor(Color::White),
                    Print(&line[2..]),
                    ResetColor
                )?;
            } else if line.starts_with("```") {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Magenta),
                    SetAttribute(Attribute::Dim),
                    Print(line),
                    SetAttribute(Attribute::Reset),
                    ResetColor
                )?;
            } else {
                execute!(stdout, SetForegroundColor(Color::White), Print(line), ResetColor)?;
            }
        }
        
        Ok(())
    }

    fn draw_footer(&self, stdout: &mut io::Stdout) -> crossterm::Result<()> {
        let footer_y = self.terminal_height - 2;
        
        execute!(
            stdout,
            cursor::MoveTo(0, footer_y),
            SetForegroundColor(Color::DarkGrey),
            Print("─".repeat(self.terminal_width as usize)),
            cursor::MoveTo(2, footer_y + 1),
            SetForegroundColor(Color::Grey),
            Print("Navigation: "),
            SetForegroundColor(Color::Cyan),
            Print("a"),
            SetForegroundColor(Color::Grey),
            Print(" Projects | "),
            SetForegroundColor(Color::Cyan),
            Print("s"),
            SetForegroundColor(Color::Grey),
            Print(" About | "),
            SetForegroundColor(Color::Cyan),
            Print("d"),
            SetForegroundColor(Color::Grey),
            Print(" Contact | "),
            SetForegroundColor(Color::Cyan),
            Print("h"),
            SetForegroundColor(Color::Grey),
            Print(" Home | "),
            SetForegroundColor(Color::Cyan),
            Print("q"),
            SetForegroundColor(Color::Grey),
            Print(" Quit | "),
            SetForegroundColor(Color::Cyan),
            Print("↑↓"),
            SetForegroundColor(Color::Grey),
            Print(" Scroll"),
            ResetColor
        )?;
        
        Ok(())
    }

    fn handle_input(&mut self) -> crossterm::Result<bool> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(false),
                    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => return Ok(false),
                    KeyCode::Char('h') | KeyCode::Home => {
                        self.current_page = Page::Home;
                        self.scroll_offset = 0;
                    }
                    KeyCode::Char('a') => {
                        self.current_page = Page::Store;
                        self.scroll_offset = 0;
                    }
                    KeyCode::Char('s') => {
                        self.current_page = Page::About;
                        self.scroll_offset = 0;
                    }
                    KeyCode::Char('d') => {
                        self.current_page = Page::FAQ;
                        self.scroll_offset = 0;
                    }
                    KeyCode::Up => {
                        self.scroll_offset = self.scroll_offset.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        self.scroll_offset = self.scroll_offset.saturating_add(1);
                    }
                    KeyCode::PageUp => {
                        self.scroll_offset = self.scroll_offset.saturating_sub(10);
                    }
                    KeyCode::PageDown => {
                        self.scroll_offset = self.scroll_offset.saturating_add(10);
                    }
                    _ => {}
                }
            }
        }
        Ok(true)
    }

    fn run(&mut self) -> crossterm::Result<()> {
        let mut stdout = io::stdout();
        
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
        
        let result = self.main_loop(&mut stdout);
        
        execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
        disable_raw_mode()?;
        
        result
    }

    fn main_loop(&mut self, stdout: &mut io::Stdout) -> crossterm::Result<()> {
        loop {
            // Update terminal size
            let (width, height) = crossterm::terminal::size()?;
            self.terminal_width = width;
            self.terminal_height = height;
            
            self.render(stdout)?;
            
            if !self.handle_input()? {
                break;
            }
        }
        Ok(())
    }
}

fn main() -> crossterm::Result<()> {
    let mut portfolio = Portfolio::new();
    portfolio.run()?;
    
    println!("\n✨ Thanks for visiting my portfolio! ✨\n");
    
    Ok(())
}
