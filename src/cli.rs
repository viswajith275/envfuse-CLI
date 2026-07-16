use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "envseal", about = "Encrypted Enviornment Manager")]
#[command(version = "v1.2.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
 
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize and create seal encrypted with "Master Password" to store secrets
    Init,
    ///[GROUP_NAME] > .env  Retrieve the enviornment variable into a .env file
    Export {
        group_name: String,
        keys: Vec<String>,
    },
    ///[GROUP_NAME] [PATH_OF_.ENV]  Loads the variables in the .env to the specified group
    Import {
        group_name: String,
        path: String,
    },
    ///[GROUP_NAME] [KEY]   Set or Update a value of a given key in a group (Creates the group if it doesnt exists)
    Set {
        group_name: String,
        key: String,
    },
    ///[GROUP_NAME] [KEY]   Get the value of a given key in a group
    Get {
        group_name: String,
        key: String,
    },
    ///[GROUP_NAME] [KEYS..]    Load the given keys or group (if keys not given) into current terminal enviornment (Use run command for most usecases)
    Load {
        group_name: String,
        keys: Vec<String>,
    },
    ///[GROUP_NAME] [KEY]   Removes the given key or group (if key not specified)
    Remove {
        group_name: String,
        key: Option<String>,
    },
    ///[GROUP_NAME] List all keys in a group
    List{
        group_name: String,
    },
    ///[GROUP_NAME] [COMMAND]   Loads the enviornment variables into a child process (not into current terminal session) 
    Run {
        group_name: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        cmd_args: Vec<String>,
    },
}