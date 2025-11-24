use crossterm::{
    cursor,
    execute,
    style::{SetForegroundColor, Color, Print, ResetColor, SetAttribute, Attribute},
};
use std::io::{self, Write};
use crate::ui::handler::Page;

const NAV_ITEM_WIDTH: usize = 12;

pub fn center_text(text: &str, width: usize) -> String {
    let padding = (width.saturating_sub(text.len())) / 2;
    format!("{:padding$}{}{:padding$}", "", text, "", padding = padding)
}

pub fn nav_item(label: &str, highlighted: bool) -> String {
    if highlighted {
        center_text(&format!("● {}", label), NAV_ITEM_WIDTH)
    } else {
        center_text(&format!("○ {}", label), NAV_ITEM_WIDTH)
    }
}

pub fn draw_header(stdout: &mut io::Stdout, start_x: u16, start_y: u16, current_page: &Page) -> crossterm::Result<u16> {
    // Draw animated gradient header banner
    draw_banner(stdout, start_x, start_y)?;
    
    // Navigation below banner
    let nav_y = start_y + 5;
    execute!(stdout, cursor::MoveTo(start_x, nav_y), SetForegroundColor(Color::DarkGrey))?;
    
    let home = nav_item("HOME", matches!(current_page, Page::Home));
    let projects = nav_item("PROJECTS", matches!(current_page, Page::Store));
    let about = nav_item("ABOUT", matches!(current_page, Page::About));
    let contact = nav_item("CONTACT", matches!(current_page, Page::FAQ));
    
    let components = vec![home, projects, about, contact];
    
    let mut x = start_x + 2;
    let mut top_border = String::from("╭");
    let mut bottom_border = String::from("╰");
    
    for (i, component) in components.iter().enumerate() {
        top_border.push_str(&"─".repeat(component.len()));
        bottom_border.push_str(&"─".repeat(component.len()));
        
        if i < components.len() - 1 {
            top_border.push('┬');
            bottom_border.push('┴');
        }
    }
    
    top_border.push('╮');
    bottom_border.push('╯');
    
    execute!(stdout, cursor::MoveTo(start_x, nav_y), Print(&top_border))?;
    execute!(stdout, cursor::MoveTo(start_x, nav_y + 1), Print("│"))?;
    
    for (i, component) in components.iter().enumerate() {
        execute!(stdout, cursor::MoveTo(x, nav_y + 1))?;
        
        let is_highlighted = (i == 0 && matches!(current_page, Page::Home)) ||
                           (i == 1 && matches!(current_page, Page::Store)) ||
                           (i == 2 && matches!(current_page, Page::About)) ||
                           (i == 3 && matches!(current_page, Page::FAQ));
        
        if is_highlighted {
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::Cyan),
                Print(component),
                SetAttribute(Attribute::Reset),
                ResetColor,
                SetForegroundColor(Color::DarkGrey)
            )?;
        } else {
            execute!(stdout, SetForegroundColor(Color::Grey), Print(component), SetForegroundColor(Color::DarkGrey))?;
        }
        
        x += component.len() as u16;
        
        if i < components.len() - 1 {
            execute!(stdout, cursor::MoveTo(x, nav_y + 1), Print("│"))?;
            x += 1;
        }
    }
    
    execute!(stdout, cursor::MoveTo(x, nav_y + 1), Print("│"))?;
    execute!(stdout, cursor::MoveTo(start_x, nav_y + 2), Print(&bottom_border))?;
    execute!(stdout, ResetColor)?;
    
    Ok(8) // Total header height including banner
}

fn draw_banner(stdout: &mut io::Stdout, start_x: u16, start_y: u16) -> crossterm::Result<()> {
    // ASCII art banner with gradient effect
    let banner_lines = vec![
        "  ██████╗  ██████╗ ██████╗ ████████╗███████╗ ██████╗ ██╗     ██╗ ██████╗ ",
        "  ██╔══██╗██╔═══██╗██╔══██╗╚══██╔══╝██╔════╝██╔═══██╗██║     ██║██╔═══██╗",
        "  ██████╔╝██║   ██║██████╔╝   ██║   █████╗  ██║   ██║██║     ██║██║   ██║",
        "  ██╔═══╝ ██║   ██║██╔══██╗   ██║   ██╔══╝  ██║   ██║██║     ██║██║   ██║",
        "  ██║     ╚██████╔╝██║  ██║   ██║   ██║     ╚██████╔╝███████╗██║╚██████╔╝",
        "  ╚═╝      ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝      ╚═════╝ ╚══════╝╚═╝ ╚═════╝ ",
    ];
    
    let colors = vec![
        Color::Magenta,
        Color::Blue,
        Color::Cyan,
        Color::Green,
        Color::Yellow,
        Color::Red,
    ];
    
    for (i, line) in banner_lines.iter().enumerate() {
        execute!(
            stdout,
            cursor::MoveTo(start_x, start_y + i as u16),
            SetForegroundColor(colors[i % colors.len()]),
            SetAttribute(Attribute::Bold),
            Print(line),
            SetAttribute(Attribute::Reset)
        )?;
    }
    
    Ok(())
}
