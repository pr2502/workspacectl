use std::env;
use std::io::{self, Write};
use std::process::Command;

use anyhow::{Context, Result};
use cache::Key;
use workspace::Workspace;

mod cache;
mod workspace;

pub fn init(ssh: Option<String>, path: String, name: Option<String>) -> Result<()> {
    match ssh {
        Some(_) => todo!("new --ssh"),
        None => init_local(path, name),
    }
}

fn init_local(path: String, name: Option<String>) -> Result<()> {
    let dir = env::current_dir()
        .context("get current working directory")?
        .join(path);
    let dir = dir
        .canonicalize()
        .with_context(|| format!("canonicalize path {dir:?}"))?;
    let name = match name {
        Some(name) => name,
        None => dir
            .file_name()
            .with_context(|| format!("cannot infer name for workspace in directory {dir:?}"))?
            .to_str()
            .with_context(|| format!("directory name is an invalid workspace name {dir:?}"))?
            .to_owned(),
    };
    let dir = dir
        .to_str()
        .with_context(|| format!("path {dir:?} is not valid UTF-8"))?
        .to_owned();

    let workspace = Workspace {
        name,
        dir,
        ssh: None,
        editor: None,
        shell: None,
    };
    workspace::create(&workspace).context("create new workspace config")
}

pub fn list() -> Result<()> {
    let mut stdout = io::stdout().lock();
    for workspace in workspace::list() {
        stdout
            .write_all(workspace.as_bytes())
            .context("writing to stdout")?;
        stdout.write_all(b"\n").context("writing to stdout")?;
    }
    Ok(())
}

pub fn open(name: String) -> Result<()> {
    let _workspace = workspace::read(&name).context("reading workpsace definition")?;
    cache::write(Key::Current, name).context("setting currently open workspace")?;
    Ok(())
}

pub fn terminal() -> Result<()> {
    let workspace = workspace::current().context("get current workspace")?;
    let dir = &workspace.dir;
    let shell_cmd = match &workspace.shell {
        Some(shell) => shell.command.as_str(),
        None => "/usr/bin/bash", // TODO find first which exists from a list of shells
    };

    if let Some(ssh) = &workspace.ssh {
        Command::new("kitty")
            .args([
                "ssh",
                "-t",
                &ssh.host,
                &format!("cd {dir}; exec {shell_cmd} --login"),
            ])
            .spawn()
            .context("spawn terminal")?;
    } else {
        Command::new("kitty")
            .arg(shell_cmd)
            .current_dir(dir)
            .spawn()
            .context("spawn terminal")?;
    }
    Ok(())
}

pub fn editor() -> Result<()> {
    let workspace = workspace::current().context("get current workspace")?;
    let dir = &workspace.dir;
    let editor_cmd = match &workspace.editor {
        Some(editor) => editor.command.as_str(),
        None => "vim", // TODO find first which exists from a list of editors
    };

    if let Some(ssh) = &workspace.ssh {
        Command::new("kitty")
            .args([
                "ssh",
                "-t",
                &ssh.host,
                &format!("cd {dir}; exec /usr/bin/bash --login {editor_cmd} .",),
            ])
            .spawn()
            .context("spawn terminal")?;
    } else {
        Command::new("kitty")
            .args([editor_cmd, "."])
            .current_dir(&workspace.dir)
            .spawn()
            .context("spawn terminal")?;
    }
    Ok(())
}
