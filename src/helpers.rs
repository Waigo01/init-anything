use std::{fs, io::Error, env, path::Path, process::{Command, Stdio}};

use crate::{Template, errors::{LoadError, ReplaceError}, Var};

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

pub fn executeCommand(command: &String, args: &Vec<Vec<String>>, ownFlags: &Vec<String>, wait: bool, workDir: Option<String>) -> Result<(), Error> {
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

    if wait {
        cmd.output()?;
    } else {
        cmd.spawn()?;
    }

    Ok(())
}

pub fn replaceVars(replaceString: String, vars: &Vec<Var>, ownFlags: &Vec<String>) -> Result<String, ReplaceError> {
    let mut builtReplaceString = replaceString;
    for i in vars {
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
