pub mod environment;
pub mod errors;
pub mod lisp;
pub mod parser;
pub mod project;

use std::{
    io::{stdout, Write}, process, rc::Rc, time::Duration
};

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use environment::Environment;
use parser::ProjectParser;
use ratatui::{
    backend,
    layout::{Constraint, Direction, Layout},
    prelude::CrosstermBackend,
    style::{Modifier, Style, Styled},
    symbols,
    widgets::{Block, List, ListState, Paragraph},
    Frame, Terminal,
};
use rust_lisp::{interpreter::eval, parser::parse};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    subcommand: Option<Action>,
}

#[derive(Subcommand, Debug)]
enum Action {
    Run { project_root: String },
}

#[derive(Clone, Debug)]
enum TuiState {
    Menu { selection: ListState },
    Folders { selection: ListState },
}

fn menu_render(frame: &mut Frame, state: &mut TuiState, selection: &ListState) {
    let menu_options = ["Open folder", "Settings", "Quit"];
    let list = List::new(menu_options)
        .block(Block::bordered().title("Options"))
        .highlight_style(Style::new().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(5), Constraint::Min(1)]);
    let area = layout.split(frame.area());
    let mut selection = selection.clone();
    frame.render_widget(
        Paragraph::new("NHHE\nEngine\nV3")
            .centered()
            .set_style(Style::default().add_modifier(Modifier::BOLD))
            .block(Block::bordered().border_set(symbols::border::DOUBLE)),
        area[0],
    );
    frame.render_stateful_widget(list, area[1], &mut selection);
    *state = TuiState::Menu { selection };
}

fn menu_input(state: &mut TuiState, selection: &ListState) -> Result<()> {
    match read()? {
        Event::Key(ev) => match ev {
            KeyEvent {
                code: KeyCode::Char('j'),
                kind: KeyEventKind::Press,
                ..
            } => {
                let mut selection = selection.clone();
                selection.select_next();

                *state = TuiState::Menu { selection };
            }
            KeyEvent {
                code: KeyCode::Char('k'),
                kind: KeyEventKind::Press,
                ..
            } => {
                let mut selection = selection.clone();
                selection.select_previous();
                *state = TuiState::Menu { selection };
            }
            KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            } => {
                match selection.selected() {
                    Some(0) => {
                        *state = TuiState::Folders { selection: ListState::default() }
                    },
                    Some(1) => {},
                    Some(2) => {
                        process::exit(0);
                    },
                    _ => {}
                }
            }
            
            _ => {}
        },
        _ => {}
    };
    Ok(())
}

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut cmd = Args::command();
    let args = Args::parse();

    let mut state = TuiState::Menu {
        selection: ListState::default(),
    };

    stdout().execute(Clear(ClearType::All))?.flush()?;

    match args.subcommand {
        Some(subc) => cli(subc)?,
        None => {
            let mut environment = Environment::new().register_all();
            while !environment.data.borrow().quit {
                terminal.draw(|frame| match state.clone() {
                    TuiState::Menu { selection } => {
                        menu_render(frame, &mut state, &selection.clone());
                    },
                    TuiState::Folders { selection } => {}
                })?;
                if poll(Duration::from_millis(0))? {
                    match state.clone() {
                        TuiState::Menu { ref selection } => menu_input(&mut state, selection)?,
                        TuiState::Folders { ref selection } => {}
                    }
                }
            }
        }
    }
    Ok(())
}

fn cli(subc: Action) -> Result<()> {
    match subc {
        Action::Run { project_root } => {
            let mut parser = ProjectParser::new(&project_root);
            let project = parser.parse()?;

            let environment = Environment::new().register_all();

            let room = project
                .rooms
                .get(&project.meta.settings.first_room)
                .unwrap();
            let p = parse(&room.pre);
            for roots in p {
                let e = eval(Rc::clone(&environment.context), &roots.unwrap());
                match e {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("error: {}", e.msg);
                    }
                }
            }
        }
    }
    Ok(())
}
