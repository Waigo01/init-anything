use crate::{Config, errors::RunError, RunCommand, helpers::{getCommandArgs, executeCommand, replaceVars}};

pub fn runCmd(config: Config, ownFlags: Vec<String>, ownArgs: Vec<String>) -> Result<(), RunError> {
    if config.runCommands.is_some() && config.runCommands.as_ref().unwrap().len() > 0 {
        let commands: Vec<RunCommand> = config.runCommands.unwrap();
        for i in commands {
            if i.name == ownArgs[2] {
                let mut count = 0;
                let runAsync = i.runAsync.is_some() && i.runAsync.unwrap();
                for j in &i.commands {
                    let mut command = j.command.clone();
                    if config.vars.is_some() && config.vars.as_ref().unwrap().len() > 0 {
                        match replaceVars(command.to_string(), config.vars.as_ref().unwrap(), &ownFlags, i.name.clone()) {
                            Ok(s) => command = s,
                            Err(e) => return Err(RunError { message: e.message }),
                        }
                    }
                    let args = getCommandArgs(&command.to_string());
                    let mut commandArgs = vec![];
                    if args.len() > 1 {
                        commandArgs = args[1..].to_vec();
                    }
                    if count == i.commands.len()-1 {
                        match executeCommand(&args[0], &vec![commandArgs], &vec!["-v".to_string()], false, j.execDir.clone()) { Ok(_) => {}, Err(e) => {return Err(RunError { message: format!("Error running command {}: {}", e.command, e.message) })}};
                    } else {
                        match executeCommand(&args[0], &vec![commandArgs], &ownFlags, runAsync, j.execDir.clone()) { Ok(_) => {}, Err(e) => {return Err(RunError { message: format!("Error running command {}: {}", e.command, e.message) })}};
                    }
                    count += 1;
                }
                return Ok(());
            }
        }
        return Err(RunError { message: format!("No command {} found in config!", ownArgs[2]) });
    }else{
        return Err(RunError { message: "No commands found in config!".to_string() });
    }
}
