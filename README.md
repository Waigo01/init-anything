# init-anything

## Description:

init-anything is a simple init and task cli-tool written in rust. It can create projects from templates and add functionality like command shorthands using the init-anything.json file.

## Installation:

Just download the git repo and use make:


```
git clone https://git.webbybrains.com/JanErhard/init-anything.git \
cd init-anything \
make \
make install
```

## Usage:

### Command Usage:

```
init-anything [init | run] <run-command>

Flags:
    -v: verbose output
    --<veriable for run-command>=<value>
```

### Config File Usage:

```json
{
  "name": "",
  "(runCommands)": [
    {
      "name": "",
      "(runAsync)": true,
      "commands": [{"command": "", "(execDir)": ""}]
    }
  ],
  "(initCommands)": [{"command": "", "(execDir)": ""}],
  "(forceInitVerbose)": true,
  "(addDeps)": [{"command": "", "deps": [""]}],
  "(vars)": [{"name": "", "(reqFor)": "", "(default)": ""}],
  "(varFiles)": [""]
}
```

All the fields in parantheses are optional. The config file must be stored at the root of the template as init-anything.json.

## Examples:

After installation you will find a ".init-anything" directory in your home directory. Here you will find all the templates. There are two premade templates to showcase the functionalities of init-anything (feel free to delete them). One of the templates initializes a simple rust project and its config looks as follows:

```json
{
  "name": "Simple Rust",
  "initCommands": [{"command": "cargo init"}, {"command": "git init"}]
}
```

The field "name" is the only required field. This is the name shown to the user when trying to initialize a project. The field "initCommands" is a simple list of commands that are run when initializing the project. **It may be important to know that init-anything first runs the init-commands then copies the files and directories in the template path and then adds any dependencies that may be specified in the "addDeps" section of the config.**

Here is the second example template which initializes a simple environment for developing and running a webserver using tide and using htmx and alpinejs on the frontend. Its config looks like this:

```json
{
  "name": "Rust - Tide - HTMX - Alpine",
  "runCommands": [
    {
      "name": "dev",
      "runAsync": true,
      "commands": [{"command": "live-server --port=$port0 --watch=./,../static --mount=/static:./static --proxy=/api/:http://127.0.0.1:$port1/api/ ./pages"}, {"command": "cargo watch --ignore \"static\" --ignore \"pages\" -x run%20$port1"}]
    }
  ],
  "initCommands": [{"command": "cargo init"}, {"command": "git init"}, {"command": "sudo npm install -g live-server"}],
  "forceInitVerbose": true,
  "addDeps": [{"command": "cargo add", "deps": ["serde --features=derive", "tide", "serde_json", "async-std --features=attributes", "lazy_static", "tera"]}],
  "vars": [{"name": "port0", "default": "3017"}, {"name": "port1", "default": "3018"}]
}
```

You can again see the "name" and "initCommands" fields but there are also a lot more fields. The "addDeps" field is a field with which you can specify what dependencies to add and how you would like to add them. You can also see the "forceInitVerbose" field. This field will force the use of the "-v"-flag when initializing a project when set to true. Also, there is the "runCommands" field. This field specifies any commands you would like to run with one simple usage of the command init-anything. Each command in the list must have a command assigned to it. The filed "runAsync" is also optional and will run the commands in order but asynchronously. The "vars" field contains variables that can be set using the flag --<variable>=<value> when executing the run or init command. It can be referenced using the $variable shorthand in any init or run command. If no default is given the flag for setting the variable is required. Another little quirk you will find is the "%20" string in the cargo watch command. The "%20" shorthand prevents the splitting up of the command at this position into different arguments. This is useful and required in this case as cargo watch wants the run command and its arguments to be one argument to the cargo watch command. The "%20" gets replaced with " " after splitting up of the command into its different arguments.

The command "dev" can be run like this:

```
init-anything run dev --port0=8080 --port1=8081

```

Finally, you can find one last config:

```json
{
  "name": "Cmake",
  "runCommands": [
    {
      "name": "start",
      "commands": [{"command": "cmake ..", "execDir": "./build"}, {"command": "make"}, {"command": "./$projectName"}]
    }
  ],
  "varFiles": ["./CMakeLists.txt"],
  "vars": [{"name": "projectName", "reqFor": "init, start"}]
}
```

This config initializes a simple Cmake project. This config showcases the abbility to set variables in certain files using the "varFiles" field. This will replace any variables in the files using the $variable shorthand at init time. You can also find the "reqFor" field in the variables. This field can be used in order to require that the variable is set for specific commands. If it is left empty the variable is required for all commands if no default is set. If the field is set, it will require the variable only for the set commands. You can specify commands using their name and the init commands using "init". It also showcases the "execDir" field. This field can be added to any command in the runCommands.commands section. As it is not currently possible to execute a cd command in rust, this field can be used to change the directory in which the command is run. Please note that the execDir carries over to the next commands. If you for example want to change the execution directory back to the root directory after executing cmake you would have to set the execDir to be "../" on the next command. The rest of the config should look familiar.

The project can be initialized using the following command, when not using the default project name:

```
init-anything init --projectName=Example
```

The command "start" can be run like this:

```
init-anything run start --projectName=Example
```
