use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    /// Name of the workspace is definied in the file name
    #[serde(skip)]
    pub name: String,

    /// Root directory for workspace
    pub dir: String,

    /// SSH configuration for remote workspace
    pub ssh: Option<Ssh>,

    /// Editor configuration
    pub editor: Option<Editor>,

    /// Shell configuration
    pub shell: Option<Shell>,
}

/// SSH connection options
#[derive(Debug, Serialize, Deserialize)]
pub struct Ssh {
    /// The ssh command. Defaults to `ssh`
    pub command: Option<String>,

    /// Destination `user`
    ///
    /// Passed as the `-l` option to the `ssh` command if present.
    pub user: Option<String>,

    /// Destination `host`
    ///
    /// Passed directly to the `ssh` command after all options but before the target command.
    pub host: String,

    /// Destination `port`
    ///
    /// Passed as the `-p` option to the `ssh` command if present.
    pub port: Option<u16>,

    /// Identity file
    ///
    /// Passed as the `-i` option to the `ssh` command if present.
    pub identity_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Editor {
    /// Editor command
    pub command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shell {
    /// Shell command
    pub command: String,
}
