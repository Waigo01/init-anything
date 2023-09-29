use std::{fs, path::{PathBuf, Path}};

use crate::{errors::InitError, Template, helpers::{getCommandArgs, executeCommand, replaceVars}};

pub fn initTemplate(template: Template, ownFlags: Vec<String>) -> Result<(), InitError> {
    let config = template.config;
    if config.initCommands.is_some() {
        for i in config.initCommands.unwrap() {
            let mut command = i.clone();
            if config.vars.is_some() && config.vars.as_ref().unwrap().len() > 0 {
                match replaceVars(command.to_string(), config.vars.as_ref().unwrap(), &ownFlags) {
                    Ok(s) => command = s,
                    Err(e) => return Err(InitError { message: e.message }),
                }
            }
            let args: Vec<String> = getCommandArgs(&command);
            executeCommand(&args[0], &vec![args[1..].to_vec()], &ownFlags, false, None)?;
        }
    }
    for i in fs::read_dir(template.path)? {
        let entry = i?.path();
        let args = vec!["-r".to_string(), entry.to_str().unwrap().to_string(), "./".to_string()];
        executeCommand(&"cp".to_string(), &vec![args], &ownFlags, false, None)?;
    }
    if config.varFiles.is_some() && config.varFiles.as_ref().unwrap().len() > 0 {
        for i in config.varFiles.unwrap() {
            if fs::read_dir("./").unwrap().map(|x| x.unwrap().path()).collect::<Vec<PathBuf>>().contains(&Path::new(&i).into()) {
                fs::write(i.clone(), match &replaceVars(fs::read_to_string(i.clone()).unwrap(), &config.vars.as_ref().unwrap(), &ownFlags) {
                    Ok(s) => s,
                    Err(e) => return Err(InitError {message: e.message.clone()}),
                })?;
            } 
        }
    }
    if config.addDeps.is_some() {
        for i in config.addDeps.unwrap() {
            let args: Vec<String> = getCommandArgs(&i.command);
            for j in i.deps {
                let addArgs: Vec<String> = getCommandArgs(&j);
                executeCommand(&args[0], &vec![args[1..].to_vec(), addArgs], &ownFlags, false, None)?;
            }
        }
    }

    Ok(())
}
