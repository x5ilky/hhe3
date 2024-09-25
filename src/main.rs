pub mod environment;
pub mod errors;
pub mod lisp;
pub mod parser;
pub mod project;

use std::{
    fs, io::{stdout, Write}, process, rc::Rc, time::Duration
};

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use environment::Environment;
use parser::{Metadata, ProjectParser};
use ratatui::{
    backend,
    layout::{Constraint, Direction, Layout},
    prelude::CrosstermBackend,
    style::{Modifier, Style, Styled, Stylize},
    symbols::{self, border},
    widgets::{Block, List, ListDirection, ListState, Paragraph},
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

struct ProjectDetails {
    name: String,
    author: Option<String>
}

fn refresh_projects(projects: &mut Vec<ProjectDetails>) {
    projects.clear();
    if fs::exists("./stories").unwrap() {
        let dir = fs::read_dir("./stories").unwrap();
        for folder in dir {
            let content = fs::read_to_string(folder.unwrap().path().join("meta.toml"));
            match content {
                Ok(v) => {
                    let v: Metadata = toml::from_str(&v).unwrap();
                    projects.push(ProjectDetails { name: v.meta.name, author: v.meta.author });
                },
                Err(_) => {}
            }
        }
    }
}

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut cmd = Args::command();
    let args = Args::parse();

    let mut state = TuiState::Menu {
        selection: ListState::default(),
    };

    let mut projects = vec![];

    refresh_projects(&mut projects);

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
                    TuiState::Folders { selection } => {
                        let mut sel = selection.clone();
                        folders_render(frame, &mut state, &mut sel, &projects).unwrap();
                        state = TuiState::Folders { selection: sel };
                    }
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

fn folders_render(frame: &mut Frame<'_>, state: &mut TuiState, clone: &mut ListState, projects: &Vec<ProjectDetails>) -> Result<()> {
    let block = Block::bordered()
        .border_set(border::ROUNDED)
        .title("Projects");
    if projects.is_empty() {
        let widget = Paragraph::new("Either I couldn't find the stories/ folder\nor the folder was empty :(");
        frame.render_widget(widget.block(block), frame.area());
    } else {
        let list = List::new(projects.iter().map(|v| v.name.as_str()).collect::<Vec<&str>>())
            .block(block)
            .direction(ListDirection::TopToBottom)
            .highlight_style(Style::default().bold())
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, frame.area(), clone);
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
