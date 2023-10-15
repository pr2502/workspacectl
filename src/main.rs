use clap::{Parser, Subcommand};
use log::debug;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Create a new workspace
    #[command(name = "new", visible_alias("init"))]
    Init {
        /// SSH host
        #[clap(long)]
        ssh: Option<String>,

        /// Workspace path
        ///
        /// Path can be either relative or absolute. Relative paths are relative to the current
        /// working directory for local workspaces and to the remote `$HOME` for remote workspaces.
        path: String,

        /// Name of the project to initialize.
        ///
        /// Defaults to the last
        name: Option<String>,
    },

    /// List defined workspaces
    List {},

    /// Open a workspace
    Open {
        /// Workspace name
        name: String,
    },

    /// Open a terminal in the current workspace root
    Terminal {},

    /// Open an editor in the current workspace root
    Editor {},
}

fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp(None).init();

    let opts = Opts::parse();
    debug!("opts = {opts:#?}");
    match opts.cmd {
        Cmd::Init { ssh, path, name } => workspacectl::init(ssh, path, name),
        Cmd::List {} => workspacectl::list(),
        Cmd::Open { name } => workspacectl::open(name),
        Cmd::Terminal {} => workspacectl::terminal(),
        Cmd::Editor {} => workspacectl::editor(),
    }
}
