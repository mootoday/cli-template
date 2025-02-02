use crate::cli_context::CliContext;
use crate::structs::CommandReturn;
use anyhow::Error;
use clap::Args;
use std::io::{stdout, Write};
use termimad::{crossterm::{
    self, cursor::{Hide, Show}, event::{self, Event, KeyCode::*, KeyEvent}, queue, style::Color::*, terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}
}, Area, MadSkin, MadView};

const CHANGELOG: &str = include_str!("../../../CHANGELOG.md");

/// Print the CLI changelog
#[derive(Debug, Args, Default)]
pub struct Command {}

pub async fn execute(_cli_context: &CliContext, _cmd: Command) -> Result<CommandReturn, Error> {
    print_changelog()?;
    Ok(CommandReturn::default())
}

fn print_changelog() -> Result<(), Error> {
    let mut stdout = stdout();
    queue!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(stdout, Hide)?;
    let mut view = MadView::from(CHANGELOG.to_owned(), view_area(), make_skin());
    loop {
        view.write_on(&mut stdout)?;
        stdout.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                Up | Char('k') => view.try_scroll_lines(-1),
                Down | Char('j') => view.try_scroll_lines(1),
                PageUp => view.try_scroll_pages(-1),
                PageDown => view.try_scroll_pages(1),
                _ => break,
            },
            Ok(Event::Resize(..)) => {
                queue!(stdout, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }
    terminal::disable_raw_mode()?;
    queue!(stdout, Show)?;
    queue!(stdout, LeaveAlternateScreen)?;
    stdout.flush()?;
    Ok(())
}

fn make_skin() -> MadSkin {
    let orange = Rgb {
        r: 255,
        g: 191,
        b: 96,
    };
    let mut skin = MadSkin::default();
    skin.set_headers_fg(orange);
    skin.bold.set_fg(orange);
    skin.italic.set_fg(orange);
    skin.scrollbar.thumb.set_fg(orange);
    skin.headers[1].add_attr(crossterm::style::Attribute::Bold); // Title
    skin.headers[2].left_margin = 2; // Version
    skin.headers[3].left_margin = 2; // Major|Minor|Patch Changes
    skin.headers[4].left_margin = 2;
    skin.headers[5].left_margin = 2;
    skin.headers[6].left_margin = 2;
    skin.headers[7].left_margin = 2;
    skin.paragraph.left_margin = 2;
    skin.code_block.left_margin = 2;
    skin
}

fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(120);
    area
}
