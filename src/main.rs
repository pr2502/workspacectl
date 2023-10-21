use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Create a new workspace
    New {
        /// SSH host
        #[clap(long)]
        ssh: Option<String>,

        /// Workspace path
        ///
        /// Path can be either relative or absolute. Relative paths are relative
        /// to the current working directory for local workspaces and to the
        /// remote `$HOME` for remote workspaces.
        #[clap(default_value = ".")]
        path: String,

        /// Name for the new workspace
        ///
        /// Defaults to the last segment of canonicalized PATH.
        name: Option<String>,
    },

    /// List defined workspaces
    List {},

    /// Open a workspace
    Open {
        /// Workspace name
        name: String,
    },

    /// Print the workspace config as JSON
    Cat {
        /// Workspace name
        ///
        /// Defaults to the current open workspace.
        name: Option<String>,
    },

    /// Open a terminal in the current workspace
    Terminal {},

    /// Open an editor in the current workspace
    Editor {},
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Cmd::New { ssh, path, name } => workspacectl::init(ssh, path, name),
        Cmd::List {} => workspacectl::list(),
        Cmd::Open { name } => workspacectl::open(name),
        Cmd::Cat { name } => workspacectl::cat(name),
        Cmd::Terminal {} => workspacectl::terminal(),
        Cmd::Editor {} => workspacectl::editor(),
    }
}
