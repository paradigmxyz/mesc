use crate::MescCliError;

use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};
use toolstr::Colorize;

#[derive(Hash, PartialEq, Eq)]
pub(crate) struct Mention {
    pub(crate) path: PathBuf,
    pub(crate) env_var: String,
    pub(crate) simple_value: Option<String>,
    pub(crate) line: String,
}

pub(crate) fn get_shell_config_paths() -> Result<Vec<PathBuf>, MescCliError> {
    let home_dir =
        std::env::var("HOME").map_err(|_| MescCliError::Error("home dir not found".to_string()))?;
    let home_path = PathBuf::from(home_dir);
    let mut candidates = vec![home_path.join(".bashrc"), home_path.join(".profile")];

    // paths added only if they exist
    for path in [home_path.join(".zshrc")] {
        if path.exists() {
            candidates.push(path)
        }
    }

    Ok(candidates)
}

/// return list of modified files
pub(crate) fn modify_shell_config_var(
    var_name: &str,
    value: String,
    paths: Option<Vec<std::path::PathBuf>>,
) -> Result<Vec<PathBuf>, MescCliError> {
    // decide where to look
    let shell_configs = match paths {
        Some(paths) => paths,
        None => get_shell_config_paths()?,
    };

    // detect where env var is currently defined
    let mut not_set: Vec<PathBuf> = Vec::new();
    let mut complex_mentions: Vec<PathBuf> = Vec::new();
    let mut set_already: Vec<PathBuf> = Vec::new();
    let mut set_wrong: Vec<PathBuf> = Vec::new();
    for path in shell_configs.into_iter() {
        let mentions = find_env_var_mentions(var_name, &path)?;
        if mentions.is_empty() {
            not_set.push(path)
        } else if mentions.iter().all(|m| m.simple_value.is_none()) {
            complex_mentions.push(path)
        } else if mentions.iter().all(|m| m.simple_value == Some(value.clone())) {
            set_already.push(path)
        } else {
            set_wrong.push(path)
        }
    }

    // modify shell configs accordingly
    match (not_set.len(), complex_mentions.len(), set_wrong.len(), set_already.len()) {
        (0, 0, 0, 0) => {
            println!(" No shell config files specified");
            Ok(vec![])
        }
        (n_not_set, n_set_complex, n_set_wrong, n_set_already) => {
            println!(" Need to set {} in order to use this file", "MESC_PATH".green().bold());
            if n_set_already > 0 {
                println!(
                    " {} is already set to the desired value in {} shell config files",
                    var_name.bold().green(),
                    n_set_already.to_string().green().bold()
                );
                for (i, path) in set_already.iter().enumerate() {
                    println!("     {}. {}", i + 1, path.to_string_lossy().green().bold());
                }
            }
            if n_set_complex > 0 {
                println!(" {} is already set in a complex, non-standard way for {} shell config files. You must edit these files manually: {:?}", var_name.green().bold(), n_set_complex.to_string().green().bold(), get_file_names(&complex_mentions));
                for (i, path) in complex_mentions.iter().enumerate() {
                    println!("     {}. {}", i + 1, path.to_string_lossy().green().bold());
                }
            }
            if n_not_set + n_set_wrong > 0 {
                let to_change: Vec<_> = not_set.into_iter().chain(set_wrong).collect();
                edit_shell_config_files_to_value(var_name, value, to_change)
            } else {
                Ok(vec![])
            }
        }
    }
}

fn edit_shell_config_files_to_value(
    var_name: &str,
    value: String,
    to_edit: Vec<PathBuf>,
) -> Result<Vec<PathBuf>, MescCliError> {
    println!(
        " {} can be automatically updated for {} shell config files:",
        var_name.bold().green(),
        format!("{}", to_edit.len()).green().bold(),
    );
    for (i, path) in to_edit.iter().enumerate() {
        println!("     {}. {}", i + 1, path.to_string_lossy().green().bold());
    }
    let mut to_edit: std::collections::HashSet<PathBuf> = to_edit.into_iter().collect();

    let prompt = "What do you want to do?";
    let options = vec![
        "Edit these files automatically (recommended)",
        "Edit these files manually in editor",
        "Choose a subset of these files",
        "Ignore this for now",
        "Exit setup",
    ];
    let original_to_edit = to_edit.clone();
    let edited = loop {
        match inquire::Select::new(prompt, options.clone()).prompt() {
            Ok("Edit these files automatically (recommended)") => {
                for path in to_edit.iter() {
                    replace_mentions(path, var_name, &value)?
                }
                break to_edit.into_iter().collect();
            }
            Ok("Edit these files manually in editor") => {
                for path in to_edit.iter() {
                    edit::edit_file(path)?;
                }
                break to_edit.into_iter().collect();
            }
            Ok("Choose a subset of these files") => {
                to_edit = match inquire::MultiSelect::new(
                    "Select which files to edit",
                    original_to_edit
                        .clone()
                        .into_iter()
                        .map(|x| x.to_string_lossy().to_string())
                        .collect(),
                )
                .prompt()
                {
                    Ok(paths) => {
                        println!(" Selected {} files", paths.len().to_string().bold().green());
                        paths.iter().map(|p| PathBuf::from(p.to_string())).collect()
                    }
                    Err(_) => continue,
                }
            }
            Ok("Ignore this for now") => break vec![],
            Ok("Exit setup") | Ok(_) | Err(_) => std::process::exit(0),
        };
    };

    match edited.len() {
        0 => println!(" {} not updated in any shell config files", var_name.green().bold()),
        1 => println!(" {} updated in 1 config file", var_name.green().bold()),
        n => println!(" Edited {} config files", n.to_string().green().bold()),
    };

    Ok(edited)
}

/// Find mentions of environment variable in path
pub(crate) fn find_env_var_mentions(
    env_var: &str,
    path: &PathBuf,
) -> Result<Vec<Mention>, MescCliError> {
    let mut mentions: Vec<Mention> = Vec::new();
    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            if line.trim_start().starts_with('#') {
                continue;
            }
            if let Some(value) = extract_simple_value(&line, env_var) {
                mentions.push(Mention {
                    path: path.clone(),
                    env_var: env_var.to_string(),
                    simple_value: Some(value),
                    line: line.clone(),
                });
            } else if line.contains(env_var) {
                mentions.push(Mention {
                    path: path.clone(),
                    env_var: env_var.to_string(),
                    simple_value: None,
                    line,
                });
            }
        }
    }
    Ok(mentions)
}

/// Helper function to extract the simple value from a line, if present
fn extract_simple_value(line: &str, env_var: &str) -> Option<String> {
    if line.starts_with(&format!("export {}=", env_var)) {
        line.split_once('=').map(|(_, value)| value.to_string())
    } else {
        None
    }
}

/// Helper function to get a list of file names from a list of paths
fn get_file_names(paths: &[PathBuf]) -> Vec<String> {
    paths
        .iter()
        .filter_map(|path| path.file_name())
        .filter_map(|os_str| os_str.to_str())
        .map(|str_slice| str_slice.to_owned())
        .collect()
}

/// Replace mentions of an environment variable in a file with a new value
pub(crate) fn replace_mentions<P: AsRef<Path>>(
    path: P,
    env_var: &str,
    new_value: &str,
) -> Result<(), MescCliError> {
    let path = path.as_ref();

    if !path.exists() {
        let _file = File::create(path)?;
    }
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    let mut modified = false;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with(&format!("export {}=", env_var)) {
            lines.push(format!("export {}={}", env_var, new_value));
            modified = true;
        } else {
            lines.push(line);
        }
    }
    if !modified {
        lines.push("".to_string());
        lines.push(format!("export {}={}", env_var, new_value));
        modified = true;
    }

    if modified {
        let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
        for line in lines {
            writeln!(file, "{}", line)?;
        }
    }

    Ok(())
}
