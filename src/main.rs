pub mod project;
pub mod errors;
pub mod parser;
pub mod environment;
pub mod lisp;

use std::rc::Rc;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use environment::Environment;
use parser::ProjectParser;
use rust_lisp::{interpreter::eval, parser::parse};


#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    subcommand: Option<Action>
}

#[derive(Subcommand, Debug)]
enum Action {
    Run {
        project_root: String
    }
}

fn main() -> Result<()>{
    let mut cmd = Args::command();
    let args = Args::parse();

    match args.subcommand {
        Some(subc) => match subc {
            Action::Run { project_root } => {
                let mut parser = ProjectParser::new(&project_root);
                let project = parser.parse()?;

                let environment = Environment::new().register_all();
                
                dbg!(&project);
                let room = project.rooms.get(&project.meta.settings.first_room).unwrap();
                let p = parse(&room.pre);
                for roots in p {
                    eval(Rc::clone(&environment.context), &roots.unwrap()).unwrap();
                }
            }
        }
        None => {
            cmd.print_help()?;
        }
    }
    Ok(())
}
