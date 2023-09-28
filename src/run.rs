use crate::{Config, errors::RunError, RunCommand, getCommandArgs, executeCommand, replaceVars};

pub fn runCmd(config: Config, ownFlags: Vec<String>, ownArgs: Vec<String>) -> Result<(), RunError> {
    if config.runCommands.is_some() && config.runCommands.as_ref().unwrap().len() > 0 {
        let commands: Vec<RunCommand> = config.runCommands.unwrap();
        for i in commands {
            if i.name == ownArgs[2] {
                for j in &i.commands {
                    let mut command = j.clone();
                    if config.vars.is_some() && config.vars.as_ref().unwrap().len() > 0 {
                        match replaceVars(command.to_string(), config.vars.as_ref().unwrap(), &ownFlags) {
                            Ok(s) => command = s,
                            Err(e) => return Err(RunError { message: e.message }),
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
