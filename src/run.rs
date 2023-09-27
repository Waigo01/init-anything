use crate::{Config, errors::RunError, RunCommand, Var, getCommandArgs, executeCommand};

pub fn replaceVars(command: String, vars: &Vec<Var>, ownFlags: &Vec<String>) -> Result<String, RunError> {
    let mut buildCommand = command;
    for i in vars {
        let mut found = false;
        for j in ownFlags {
            if i.name == j.trim_matches('-').split("=").collect::<Vec<&str>>()[0] {
                buildCommand = buildCommand.replace(&("$".to_string() + &i.name), j.split("=").collect::<Vec<&str>>()[1]);
                found = true;
                break;
            }
        }
        if !found && i.default.is_some() {
            buildCommand = buildCommand.replace(&("$".to_string() + &i.name), &i.default.as_ref().unwrap());
        } else if !found && i.default.is_none() {
            return Err(RunError { message: format!("Could not find variable {} and no default was given!", i.name) });
        }
    }

    Ok(buildCommand)
}

pub fn runCmd(config: Config, ownFlags: Vec<String>, ownArgs: Vec<String>) -> Result<(), RunError> {
    if config.runCommands.is_some() && config.runCommands.as_ref().unwrap().len() > 0 {
        let commands: Vec<RunCommand> = config.runCommands.unwrap();
        for i in commands {
            if i.name == ownArgs[2] {
                for j in &i.commands {
                    let mut command = j.clone();
                    if i.vars.is_some() && i.vars.as_ref().unwrap().len() > 0 {
                        match replaceVars(command.to_string(), i.vars.as_ref().unwrap(), &ownFlags) {
                            Ok(s) => command = s,
                            Err(e) => return Err(e),
                        }
                    }
                    let args = getCommandArgs(&command.to_string());
                    if i.commands.last().unwrap() == j {
                        executeCommand(&args[0], &vec![args[1..].to_vec()], &vec!["-v".to_string()], true)?;
                    } else {
                        executeCommand(&args[0], &vec![args[1..].to_vec()], &ownFlags, false)?;
                    }
                }
                return Ok(());
            }
        }
        return Err(RunError { message: format!("No command {} found in config!", ownArgs[2]) });
    }else{
        return Err(RunError { message: "No commands found in config!".to_string() });
    }
}
