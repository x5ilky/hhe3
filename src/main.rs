pub mod environment;
pub mod errors;
pub mod lisp;
pub mod parser;
pub mod project;

use std::{
    fs,
    io::{stdout, Write},
    process,
    rc::Rc,
    time::Duration,
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use environment::Environment;
use parser::{Metadata, ProjectParser};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::CrosstermBackend,
    style::{Modifier, Style, Styled, Stylize},
    symbols::{self, border},
    text::Text,
    widgets::{Block, List, ListDirection, ListState, Paragraph},
    Frame, Terminal,
};
use rust_lisp::{interpreter::eval, lisp, parser::parse};

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
    Story,
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
            } => match selection.selected() {
                Some(0) => {
                    *state = TuiState::Folders {
                        selection: ListState::default(),
                    }
                }
                Some(1) => {}
                Some(2) => {
                    process::exit(0);
                }
                _ => {}
            },

            _ => {}
        },
        _ => {}
    };
    Ok(())
}

fn folder_input(
    state: &mut TuiState,
    projects: &Vec<ProjectDetails>,
    selection: &ListState,
    environment: &mut Environment,
) -> Result<()> {
    match read()? {
        Event::Key(ev) => match ev {
            KeyEvent {
                code: KeyCode::Char('j'),
                kind: KeyEventKind::Press,
                ..
            } => {
                let mut selection = selection.clone();
                selection.select_next();

                *state = TuiState::Folders { selection };
            }
            KeyEvent {
                code: KeyCode::Char('k'),
                kind: KeyEventKind::Press,
                ..
            } => {
                let mut selection = selection.clone();
                selection.select_previous();
                *state = TuiState::Folders { selection };
            }
            KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            } => {
                if selection.selected().is_some() {
                    let p = &projects[selection.selected().unwrap()];
                    let mut parser = ProjectParser::new(&p.path);
                    let project = parser.parse().unwrap();
                    {
                        let mut write = environment.data.write().unwrap();
                        write.project = project.clone();
                        write.current_room = project.meta.settings.first_room;
                    };
                    *state = TuiState::Story;
                }
            }

            KeyEvent {
                code: KeyCode::Esc | KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                ..
            } => {
                *state = TuiState::Menu {
                    selection: ListState::default(),
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
    author: Option<String>,
    path: String,
}

fn refresh_projects(projects: &mut Vec<ProjectDetails>) {
    projects.clear();
    if fs::exists("./stories").unwrap() {
        let dir = fs::read_dir("./stories").unwrap();
        for folder in dir {
            let path = folder.unwrap().path();
            let content = fs::read_to_string(path.clone().join("meta.toml"));
            match content {
                Ok(v) => {
                    let v: Metadata = toml::from_str(&v).unwrap();
                    projects.push(ProjectDetails {
                        name: v.meta.name,
                        author: v.meta.author,
                        path: path.to_str().unwrap().to_string(),
                    });
                }
                Err(_) => {}
            }
        }
    }
}

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
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
            loop {
                terminal.draw(|frame| match state.clone() {
                    TuiState::Menu { selection } => {
                        menu_render(frame, &mut state, &selection.clone());
                    }
                    TuiState::Folders { selection } => {
                        let mut sel = selection.clone();
                        folders_render(frame, &mut state, &mut sel, &projects).unwrap();
                        state = TuiState::Folders { selection: sel };
                    }
                    TuiState::Story => {
                        story_render(frame, &mut environment).unwrap();
                    }
                })?;
                if poll(Duration::from_millis(2))? {
                    match state.clone() {
                        TuiState::Menu { ref selection } => menu_input(&mut state, selection)?,
                        TuiState::Folders { ref selection } => {
                            folder_input(&mut state, &projects, selection, &mut environment)?
                        }
                        TuiState::Story { .. } => story_input(read()?, &mut environment)?,
                    }
                }

                environment.update();
                if environment.data.read().unwrap().quit {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn story_input(ev: Event, environment: &mut Environment) -> Result<()> {
    match ev {
        Event::Key(key) => {
            match key {
                KeyEvent {
                    code: KeyCode::Down | KeyCode::Up,
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    let mut write = environment.data.write().unwrap();
                    if write.options.options.len() > 0 {
                        if let KeyEvent {
                            code: KeyCode::Down,
                            ..
                        } = key
                        {
                            write.options.selected.select_next();
                        } else {
                            write.options.selected.select_previous();
                        }
                    }
                }

                KeyEvent {
                    code: KeyCode::Enter,
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    let selected = {
                        let read = environment.data.read().unwrap();
                        read.options.selected.selected().is_some()
                    };
                    if selected {
                        let option = {
                            let read = environment.data.read().unwrap();
                            let option = read.options.options
                                [read.options.selected.selected().unwrap()]
                            .clone();
                            option
                        };

                        let _ = eval(
                            Rc::clone(&environment.context),
                            &lisp! {
                                ( {option.action} )
                            },
                        )
                        .expect("Failed to evaluate lisp block");
                        // TODO(silky): proper error handling instead of just .expect-ing everything
                    }
                }
                _ => {}
            };
        }
        _ => {}
    }
    Ok(())
}

fn story_render(frame: &mut Frame, environment: &mut Environment) -> Result<()> {
    let data = environment.data.read().unwrap();
    let constraints = if data.title.show {
        vec![
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(3),
        ]
    } else {
        vec![
            Constraint::Length(0),
            Constraint::Min(0),
            Constraint::Length(3),
        ]
    };
    let layout_vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(frame.area());
    let layout_hor = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Fill(1), Constraint::Fill(4)])
        .split(layout_vert[1]);
    let displays = Paragraph::new(data.display.content.to_text());
    let displays = displays.block(Block::bordered().border_set(border::ROUNDED));

    let buttons: Vec<Text> = data
        .options
        .options
        .iter()
        .map(|v| v.name.to_text())
        .collect();
    let buttons_bar = List::new(buttons)
        .direction(ListDirection::TopToBottom)
        .highlight_symbol("<> ")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .block(Block::bordered().border_set(border::DOUBLE));

    let title_bar = Paragraph::new(data.title.content.clone())
        .set_style(Style::default().fg(data.title.color.to_ratatui_color()));

    frame.render_widget(title_bar, layout_vert[0]);
    frame.render_widget(displays, layout_hor[1]);

    let mut selected_clone = data.options.selected.clone();
    frame.render_stateful_widget(buttons_bar, layout_hor[0], &mut selected_clone);

    let debug = Paragraph::new(
        data.debug
            .iter()
            .rev()
            .take(3)
            .map(|v| v.clone())
            .collect::<Vec<String>>()
            .join("\n"),
    );
    frame.render_widget(debug, layout_vert[2]);

    Ok(())
}

fn folders_render(
    frame: &mut Frame<'_>,
    _state: &mut TuiState,
    clone: &mut ListState,
    projects: &Vec<ProjectDetails>,
) -> Result<()> {
    let block = Block::bordered()
        .border_set(border::ROUNDED)
        .title("Projects");
    if projects.is_empty() {
        let widget = Paragraph::new(
            "Either I couldn't find the stories/ folder\nor the folder was empty :(",
        );
        frame.render_widget(widget.block(block), frame.area());
    } else {
        let list = List::new(
            projects
                .iter()
                .map(|v| {
                    if v.author.is_some() {
                        format!("{} - {}", v.name, v.author.clone().unwrap())
                    } else {
                        v.name.clone()
                    }
                })
                .collect::<Vec<String>>(),
        )
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
