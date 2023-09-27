use std::fs;

use crate::{errors::InitError, Template, getCommandArgs, executeCommand};

pub fn initTemplate(template: Template, ownFlags: Vec<String>) -> Result<(), InitError> {
    let config = template.config;
    if config.initCommands.is_some() {
        for i in config.initCommands.unwrap() {
            let args: Vec<String> = getCommandArgs(&i);
            executeCommand(&args[0], &vec![args[1..].to_vec()], &ownFlags, true)?;
        }
    }
    for i in fs::read_dir(template.path)? {
        let entry = i?.path();
        let args = vec!["-r".to_string(), entry.to_str().unwrap().to_string(), "./".to_string()];
        executeCommand(&"cp".to_string(), &vec![args], &ownFlags, true)?;
    }
    if config.addDeps.is_some() {
        for i in config.addDeps.unwrap() {
            let args: Vec<String> = getCommandArgs(&i.command);
            for j in i.deps {
                let addArgs: Vec<String> = getCommandArgs(&j);
                executeCommand(&args[0], &vec![args[1..].to_vec(), addArgs], &ownFlags, true)?;
            }
        }
    }

    Ok(())
}
