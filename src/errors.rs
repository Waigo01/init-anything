use std::io;

pub struct LoadError {
    pub message: String,
    pub configPath: Option<String>
}

impl From<io::Error> for LoadError {
    fn from(error: io::Error) -> Self {
        LoadError {
            message: error.to_string(),
            configPath: None
        }
    } 
}

impl From<serde_json::Error> for LoadError {
    fn from(error: serde_json::Error) -> Self {
        LoadError {
            message: error.to_string(),
            configPath: None
        }
    }
}

pub struct InitError {
    pub message: String,
}

impl From<io::Error> for InitError {
    fn from(error: io::Error) -> Self {
        InitError {
            message: error.to_string()
        }
    }
}

impl From<serde_json::Error> for InitError {
    fn from(error: serde_json::Error) -> Self {
        InitError {
            message: error.to_string()
        }
    } 
}

pub struct RunError {
    pub message: String,
}

impl From<io::Error> for RunError {
    fn from(error: io::Error) -> Self {
        RunError {
            message: error.to_string()
        }
    }
}

impl From<serde_json::Error> for RunError {
    fn from(error: serde_json::Error) -> Self {
        RunError {
            message: error.to_string()
        }
    } 
}

pub struct ReplaceError {
    pub message: String,
}

impl From<io::Error> for ReplaceError {
    fn from(error: io::Error) -> Self {
        ReplaceError {
            message: error.to_string()
        }
    } 
}

impl From<serde_json::Error> for ReplaceError {
    fn from(error: serde_json::Error) -> Self {
        ReplaceError {
            message: error.to_string()
        }
    }
}

pub struct commandExecuteError {
    pub message: String,
    pub command: String
}

impl From<io::Error> for commandExecuteError {
    fn from(error: io::Error) -> Self {
        commandExecuteError {
            message: error.to_string(),
            command: "".to_string(),
        }
    }
}
