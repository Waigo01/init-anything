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
      "(vars)": [{"name": "", "(default)": ""}],
      "commands": [""]
    }
  ],
  "(initCommands)": [""],
  "(forceInitVerbose)": true,
  "(addDeps)": [{"command": "", "deps": [""]}]
}
```

All the fields in parantheses are optional. The config file must be stored at the root of the template as init-anything.json.

## Examples:

After installation you will find a ".init-anything" directory in your home directory. Here you will find all the templates. There are two premade templates to showcase the functionalities of init-anything (feel free to delete them). One of the templates initializes a simple rust project and its config looks as follows:

```json
{
  "name": "Simple Rust",
  "initCommands": ["cargo init", "git init"]
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
      "vars": [{"name": "port0", "default": "3017"}, {"name": "port1", "default": "3018"}],
      "commands": ["live-server --port=$port0 --watch=./,../static --mount=/static:./static --proxy=/api/:http://127.0.0.1:$port1/api/ ./pages", "cargo watch --ignore \"static\" --ignore \"pages\" -x run%20$port1"]
    }
  ],
  "initCommands": ["cargo init", "git init", "sudo npm install -g live-server"],
  "forceInitVerbose": true,
  "addDeps": [{"command": "cargo add", "deps": ["serde --features=derive", "tide", "serde_json", "async-std --features=attributes", "lazy_static", "tera"]  }]
}
```

You can again see the "name" and "initCommands" fields but there are also a lot more fields. The "addDeps" field is a field with which you can specify what dependencies to add and how you would like to add them. You can also see the "forceInitVerbose" field. This field will force the use of the "-v"-flag when initializing a project when set to true. Finally, there is the "runCommands" field. This field specifies any commands you would like to run with one simple usage of the command init-anything. Each command in the list must have a name, it can have variables which can be referenced in the commands themselvs with $var and it must of course have at least one command to execute.

The command "dev" can be run like this:

```
init-anything run dev --port0=8080 --port1=8081
```
