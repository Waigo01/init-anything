use std::{fs, env, path::Path, process::{Command, Stdio}};

use crate::{Template, errors::{LoadError, ReplaceError, CommandExecuteError, ValidationError}, Var, Config};

pub fn validateConfig(config: Config) -> Result<(), ValidationError> {
    if config.vars.is_some() {
        let mut commands: Vec<String> = vec!["init".to_string()];
        if config.runCommands.is_some() {
            for i in config.runCommands.unwrap() {
                for j in i.commands {
                    commands.push(j.command);
                }
            }
        }
        for i in &config.vars.unwrap() {
            if i.reqFor.is_some() && i.reqFor.as_ref().unwrap() != "" { 
                for j in i.reqFor.as_ref().unwrap().split(",").map(|x| x.to_string()).collect::<Vec<String>>() {
                    if !commands.contains(&j) {
                        return Err(ValidationError { message: format!("No command {} found in config!", j)});
                    } 
                }
            }
        }
    }else{
        return Ok(());
    }

    Ok(())
}

pub fn loadTemplates() -> Result<Vec<Template>, LoadError> {
    let mut configs: Vec<Template> = vec![];
    for i in fs::read_dir(home::home_dir().unwrap().join(".init-anything/templates"))? {
        let entry = i?.path();
        if !entry.join("init-anything.json").exists() {
            continue;
        }
        match serde_json::from_str::<Config>(&fs::read_to_string(entry.join("init-anything.json"))?) {
            Ok(s) => match validateConfig(s.clone()) {
                Ok(_) => {configs.push(Template {config: s, path: entry.clone()})},
                Err(x) => return Err(LoadError { message: x.message, configPath: Some(entry.join("init-anything.json").to_str().unwrap().to_string()) })
            },     
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

pub fn executeCommand(command: &String, args: &Vec<Vec<String>>, ownFlags: &Vec<String>, runAsync: bool, workDir: Option<String>) -> Result<(), CommandExecuteError> {
    if workDir.is_some() {
        env::set_current_dir(Path::new(&workDir.unwrap()))?;
    } 

    let mut cmd = Command::new(command);
    for i in args {
        cmd.args(i);
    }
    if ownFlags.contains(&"-v".to_string()) {
        cmd.stdin(Stdio::inherit()).stdout(Stdio::inherit()).stderr(Stdio::inherit());
    } else { 
        cmd.stdin(Stdio::inherit()).stdout(Stdio::null()).stderr(Stdio::null());
    }

    if runAsync {
        match cmd.spawn() {
            Ok(_) => {},
            Err(e) => {return Err(CommandExecuteError { message: e.to_string(), command: command.to_string() })}
        };
    } else {
        match cmd.output() {
            Ok(_) => {},
            Err(e) => {return Err(CommandExecuteError { message: e.to_string(), command: command.to_string() })}
        };
    }

    Ok(())
}

pub fn replaceVars(replaceString: String, vars: &Vec<Var>, ownFlags: &Vec<String>, currentCommand: String) -> Result<String, ReplaceError> {
    let mut builtReplaceString = replaceString;
    for i in vars {
        if i.reqFor.is_some() && !i.reqFor.clone().unwrap().split(",").collect::<String>().contains(&currentCommand) {
            continue;
        }
        let mut found = false;
        for j in ownFlags {
            if i.name == j.trim_matches('-').split("=").collect::<Vec<&str>>()[0] {
                builtReplaceString = builtReplaceString.replace(&("$".to_string() + &i.name), j.split("=").collect::<Vec<&str>>()[1]);
                found = true;
                break;
            }
        }
        if !found && i.default.is_some() {
            builtReplaceString = builtReplaceString.replace(&("$".to_string() + &i.name), &i.default.as_ref().unwrap());
        } else if !found && i.default.is_none() {
            return Err(ReplaceError { message: format!("Could not find variable {} and no default was given!", i.name) });
        }
    }

    Ok(builtReplaceString)
}

pub fn passFlags(totalArgs: Vec<String>) -> (Vec<String>, Vec<String>) {
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
