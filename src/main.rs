#![allow(non_snake_case)]
use core::fmt;
use std::{fs, env::{self}, path::{PathBuf, Path}, process::{Command, Stdio}, io::Error};
use init::initTemplate;
use inquire::Select;
use run::runCmd;
use serde::{Deserialize, Serialize};
use crate::errors::LoadError;

mod init;
mod errors;
mod run;

#[derive(Serialize, Deserialize)]
pub struct Var {
    pub name: String,
    pub default: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct RunCommand {
    pub name: String,
    pub vars: Option<Vec<Var>>,
    pub commands: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DependencyAdd {
    pub command: String,
    pub deps: Vec<String>
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    pub runCommands: Option<Vec<RunCommand>>,
    pub initCommands: Option<Vec<String>>,
    pub addDeps: Option<Vec<DependencyAdd>>,
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

pub fn loadTemplates() -> Result<Vec<Template>, LoadError> {
    let mut configs: Vec<Template> = vec![];
    for i in fs::read_dir(home::home_dir().unwrap().join(".init-anything/templates"))? {
        let entry = i?.path();
        if !entry.join("init-anything.json").exists() {
            continue;
        }
        match serde_json::from_str(&fs::read_to_string(entry.join("init-anything.json"))?) {
            Ok(s) => configs.push(Template {config: s, path: entry}),
            Err(e) => return Err(LoadError { message: e.to_string(), configPath: Some(entry.join("init-anything.json").to_str().unwrap().to_string()) })
        }
    }
    if configs.len() == 0 {
        return Err(LoadError{message: "No init-anything.json files found".to_string(), configPath: None});
    }
    Ok(configs)
}

pub fn getCommandArgs(command: &str) -> Vec<String> {
    command.split(" ").map(|x| x.to_string()).map(|x| x.replace("%20", " ")).collect()
}

pub fn executeCommand(command: &String, args: &Vec<Vec<String>>, ownFlags: &Vec<String>, wait: bool) -> Result<(), Error> {
    let mut cmd = Command::new(command);
    for i in args {
        cmd.args(i);
    }
    if ownFlags.contains(&"-v".to_string()) {
        cmd.stdin(Stdio::inherit()).stdout(Stdio::inherit()).stderr(Stdio::inherit());
    } else { 
        cmd.stdin(Stdio::inherit()).stdout(Stdio::null()).stderr(Stdio::null());
    }

    if wait {
        cmd.output()?;
    } else {
        cmd.spawn()?;
    }

    Ok(())
}

fn passFlags(totalArgs: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut flags: Vec<String> = vec![];
    let mut args: Vec<String> = vec![];
    for i in totalArgs {
        if i.chars().nth(0).unwrap() == '-' {
            flags.push(i);
        }else{
            args.push(i);
        }
    }

    (args, flags)
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
                Ok(_) => println!("\x1b[1;32mRunning Command\x1b[0m"),
                Err(e) => { println!("{}", e.message); return;},
            }
        }else{
            println!("\x1b[1;31mCould not find init-anything.json in current directory!\x1b[0m");
            return;
        }
    } else {
        println!("Usage: init-anything [init | run] <run-command>\n\nFlags:\n\t-v: verbose output\n\t--<variable for run-command>=<value>");
    }
}
