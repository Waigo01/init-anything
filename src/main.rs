#![allow(non_snake_case)]
use core::fmt;
use std::{fs, env::{self}, path::{PathBuf, Path}};
use helpers::{passFlags, loadTemplates};
use init::initTemplate;
use inquire::Select;
use run::runCmd;
use serde::{Deserialize, Serialize};

mod init;
mod errors;
mod run;
mod helpers;

#[derive(Serialize, Deserialize, Clone)]
pub struct Var {
    pub name: String,
    pub reqFor: Option<String>,
    pub default: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandString {
    pub command: String,
    pub execDir: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RunCommand {
    pub name: String,
    pub runAsync: Option<bool>,
    pub commands: Vec<CommandString>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DependencyAdd {
    pub command: String,
    pub deps: Vec<String>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub name: String,
    pub runCommands: Option<Vec<RunCommand>>,
    pub initCommands: Option<Vec<String>>,
    pub addDeps: Option<Vec<DependencyAdd>>,
    pub vars: Option<Vec<Var>>,
    pub varFiles: Option<Vec<String>>,
    pub forceInitVerbose: Option<bool>,
}

pub struct Template {
    pub config: Config,
    pub path: PathBuf,
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.config.name)
    }
}

fn main() {
    let totalArgs: Vec<String> = env::args().collect();
    let (args, mut flags) = passFlags(totalArgs);
    if args.len() == 2 && args[1] == "init" {
        if fs::read_dir("./").unwrap().map(|x| x.unwrap().path().to_str().unwrap().to_string()).collect::<Vec<String>>().len() != 0 {
            println!("\x1b[1;31mYour current working directory is not empty!\x1b[0m");
            return;
        }
        let mut templates: Vec<Template> = vec![];
        match loadTemplates() {
            Ok(s) => templates = s,
            Err(e) => {match e.configPath.is_some(){
                true => println!("\x1b[1;31mCould not pass config at {} with error: {}\x1b[0m", e.configPath.unwrap(), e.message),
                false => println!("\x1b[1;31mThere was an error loading the templates: {}\x1b[0m", e.message),
            }; return;},
        }
        let selectedTemplate = Select::new("Please select a template", templates).prompt().unwrap();
        if selectedTemplate.config.forceInitVerbose.is_some() && selectedTemplate.config.forceInitVerbose.unwrap() == true {
            flags.push("-v".to_string());
        }
        match initTemplate(selectedTemplate, flags) {
            Ok(_) => println!("\x1b[1;32mProject initialized!\x1b[0m"),
            Err(e) => { println!("\x1b[1;31mThere was an error initializing the template: {}\x1b[0m", e.message); return; },
        }
    } else if args.len() > 2 && args[1] == "run" {
        if fs::read_dir("./").unwrap().map(|x| x.unwrap().path()).collect::<Vec<PathBuf>>().contains(&Path::new("./").join("init-anything.json")) {
            let config: Config = match serde_json::from_str(&fs::read_to_string("./init-anything.json").unwrap()) { Ok(s) => s, Err(e) => { println!("\x1b[1;31mAn error occured while passing the json: {}\x1b[0m", e); return;}};
            match runCmd(config, flags, args) {
                Ok(_) => print!(""),
                Err(e) => { println!("{}", e.message); return;},
            }
        }else{
            println!("\x1b[1;31mCould not find init-anything.json in current directory!\x1b[0m");
            return;
        }
    } else {
        println!("Usage: init-anything [init | run] <run-command>\n\nFlags:\n\t-v: verbose output\n\t--<variable>=<value>");
    }
}
