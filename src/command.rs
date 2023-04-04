use std::env;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;

use colored::Colorize;
use url::Url;

use crate::cli::Command;
use crate::error;
use crate::parser::parse;
use crate::progress_bar::{
    get_progress_bar, get_progress_bar_handle, get_progress_bar_handle_with_rx,
};
use crate::utils::{download_file, get_configdir};

impl Command {
    pub fn run(&self) {
        match self {
            Command::New(command_args) => {
                if command_args.args.len() != 2 {
                    error!("Invalid number of arguments");
                    return;
                }

                let project_name = command_args.args[0].to_string();

                let project_dir = std::path::PathBuf::from(project_name);
                if project_dir.exists() {
                    error!("Directory with the same name already exists");
                    return;
                }

                std::fs::create_dir(&project_dir).unwrap();

                if let Err(e) = std::env::set_current_dir(&project_dir) {
                    error!(e.to_string());
                    std::process::exit(1);
                }

                let template_file_path = get_configdir()
                    .join("templates")
                    .join(command_args.args[1].to_string() + ".yaml");

                if !(template_file_path.exists()) {
                    error!("Template not found");
                    std::process::exit(1);
                }

                let template = parse(template_file_path.to_str().unwrap()).unwrap();

                let done = Arc::new(AtomicBool::new(false));
                let pb = Arc::new(Mutex::new(get_progress_bar()));

                fn create_execution_handle(
                    command: Box<dyn FnOnce() + Send>,
                ) -> std::thread::JoinHandle<()> {
                    std::thread::spawn(command)
                }

                fn run(
                    execution_handle: std::thread::JoinHandle<()>,
                    progress_bar_handle: std::thread::JoinHandle<()>,
                ) {
                    let handles = vec![execution_handle, progress_bar_handle];
                    for handle in handles {
                        handle.join().unwrap();
                    }
                }

                if let Some(setup_commands) = template.setup_commands {
                    let done_clone = done.clone();
                    let pb_clone = pb.clone();

                    let (tx, rx) = channel();
                    run(
                        create_execution_handle(Box::new(move || {
                            for setup_command in setup_commands {
                                let command = setup_command.split(' ').collect::<Vec<&str>>()[0];
                                let args = setup_command.split(' ').skip(1).collect::<Vec<&str>>();

                                if let Err(e) = tx.send(format!("Running Command: {}", command)) {
                                    error!(e.to_string());
                                    std::process::exit(1);
                                }

                                let output = std::process::Command::new(command)
                                    .args(args)
                                    .output()
                                    .expect(command);

                                if !output.stderr.is_empty() {
                                    error!(String::from_utf8_lossy(&output.stderr).to_string());
                                    error!(command.to_string());
                                    std::process::exit(1);
                                }
                            }
                            done_clone.store(true, Ordering::SeqCst);
                        })),
                        get_progress_bar_handle_with_rx(
                            done.clone(),
                            pb_clone,
                            "Running setup commands...".to_string(),
                            rx,
                        ),
                    );
                }

                if let Some(files) = template.files {
                    let done_clone = done.clone();
                    let pb_clone = pb.clone();

                    run(
                        create_execution_handle(Box::new(move || {
                            for (filepath, buffer) in files {
                                let path = std::path::Path::new(&filepath);
                                let parent = path.parent();

                                if let Some(parent) = parent {
                                    if !parent.to_str().unwrap().is_empty() {
                                        std::fs::create_dir_all(parent).unwrap();
                                    }
                                }

                                let mut file = if let Err(e) = std::fs::File::create(path) {
                                    error!(e.to_string());
                                    std::process::exit(1);
                                } else {
                                    std::fs::File::create(path).unwrap()
                                };

                                if let Err(e) = file.write_all(buffer.as_bytes()) {
                                    error!(e.to_string());
                                    std::process::exit(1);
                                };
                            }
                            done_clone.store(true, Ordering::SeqCst);
                        })),
                        get_progress_bar_handle(
                            done.clone(),
                            pb_clone,
                            "Creating files...".to_string(),
                        ),
                    );
                }

                if let Some(dependencies) = template.dependencies {
                    let done_clone = done.clone();
                    let pb_clone = pb.clone();

                    let (tx, rx) = channel();
                    run(
                        create_execution_handle(Box::new(move || {
                            for (name, install_command) in dependencies {
                                if let Err(e) = tx.send(format!("Installing {}", name.clone())) {
                                    error!(e.to_string());
                                    std::process::exit(1);
                                }

                                let command = install_command.split(' ').collect::<Vec<&str>>()[0];
                                let args =
                                    install_command.split(' ').skip(1).collect::<Vec<&str>>();

                                let output = std::process::Command::new(command)
                                    .args(args)
                                    .output()
                                    .expect("failed to execute process");

                                if !output.status.success() {
                                    error!(String::from_utf8_lossy(&output.stderr).to_string());

                                    std::process::exit(1);
                                }
                            }

                            done_clone.store(true, Ordering::SeqCst);
                        })),
                        get_progress_bar_handle_with_rx(
                            done.clone(),
                            pb_clone,
                            "Installing dependencies...".to_string(),
                            rx,
                        ),
                    );
                }

                if let Some(post_setup_commands) = template.post_setup_commands {
                    let done_clone = done.clone();
                    let pb_clone = pb;

                    let (tx, rx) = channel();

                    run(
                        create_execution_handle(Box::new(move || {
                            for post_setup_command in post_setup_commands {
                                let command =
                                    post_setup_command.split(' ').collect::<Vec<&str>>()[0];
                                let args =
                                    post_setup_command.split(' ').skip(1).collect::<Vec<&str>>();

                                if let Err(e) = tx.send(format!("Running Command: {}", command)) {
                                    error!(e.to_string());
                                    std::process::exit(1);
                                }

                                let output = std::process::Command::new(command)
                                    .args(args)
                                    .output()
                                    .expect("failed to execute process");

                                if !output.stderr.is_empty() {
                                    error!(String::from_utf8_lossy(&output.stderr).to_string());
                                    std::process::exit(1);
                                }
                            }
                            done_clone.store(true, Ordering::SeqCst);
                        })),
                        get_progress_bar_handle_with_rx(
                            done,
                            pb_clone,
                            "Running post setup commands...".to_string(),
                            rx,
                        ),
                    );
                }

                println!(
                    "{}: Ran Template {}",
                    "Success".green(),
                    command_args.args[1]
                );
            }
            Command::Init(command_args) => {
                if command_args.args.is_empty() {
                    error!("No Template name provided");
                    return;
                }

                let template_name = command_args.args[0].to_string();

                let template_file_path = get_configdir()
                    .join("templates")
                    .join(template_name.clone() + ".yaml");

                if !(template_file_path.exists()) {
                    let mut file = if let Err(e) = std::fs::File::create(&template_file_path) {
                        error!(e.to_string());
                        std::process::exit(1);
                    } else {
                        std::fs::File::create(&template_file_path).unwrap()
                    };

                    let buffer = format!(
                        "
# Go to the GitHub repo https://github.com/humblepenguinn/tmplt for more information

# Template information
name: {}
description: Description of the template

# Commands to run during setup
setup_commands:
  - command_1
  - command_2

# Dependency information
dependencies:
  - name: dependency_1
    install_command: install_command_1

  - name: dependency_2
    install_command: install_command_2

# Files to generate
files:
  - name: file_name_1
    content: |
      file contents
  - name: file_name_2
    content: |
      file contents

# Post-setup command to run after setup is complete
post_setup_command: post_setup_command

variables:
  - name: my_variable_name
    description: description of the variable
    default: the default value of the variable

",
                        &template_name
                    );
                    file.write_all(buffer.as_bytes()).unwrap();
                    println!(
                        "{}: {}",
                        "Success".green(),
                        "Template created you can go edit it at ".to_string()
                            + template_file_path.to_str().unwrap()
                    );
                } else {
                    error!("Template already exists");
                }
            }
            Command::Import(command_args) => {
                if command_args.args.len() < 2 {
                    error!("Invalid number of arguments");
                    return;
                }

                let url = &command_args.args[0];
                if Url::parse(command_args.args[0].as_str()).is_err() {
                    error!("Invalid URL");
                    return;
                }

                let file_to_save_as = command_args.args[1].to_string();

                let template_file_path = get_configdir()
                    .join("templates")
                    .join(file_to_save_as + ".yaml");

                if template_file_path.exists() {
                    error!("Template with the same name already exists");
                    return;
                }

                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap_or_else(|e| {
                        error!(e.to_string());
                        std::process::exit(1);
                    })
                    .block_on(download_file(url, template_file_path.to_str().unwrap()));
            }

            Command::List => {
                let templates_dir = get_configdir().join("templates");

                println!("{}", "Templates:".bold());
                for entry in std::fs::read_dir(templates_dir).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    let filename = path.file_name().unwrap().to_str().unwrap().to_owned();

                    println!("{}", filename);
                }
            }
            Command::Remove(command_args) => {
                if command_args.args.is_empty() {
                    error!("No Template name provided");
                    return;
                }

                let template_name = command_args.args[0].to_string();

                let template_file_path = get_configdir()
                    .join("templates")
                    .join(template_name + ".yaml");

                if template_file_path.exists() {
                    std::fs::remove_file(template_file_path).unwrap();
                    println!("{}: Template removed", "Success".green());
                } else {
                    error!("Template does not exist");
                }
            }

            Command::Version(command_args) => {
                if command_args.args.is_empty() {
                    println!("{} {}", "Version".green(), env!("BUILD_VERSION"));
                } else if command_args.args[0] == "verbose" {
                    println!("{} {}", "Version".green(), env!("BUILD_VERSION"));
                    println!("{} {}", "Build Timestamp".green(), env!("BUILD_TIMESTAMP"));
                    println!("{} {}", "Author".green(), env!("CARGO_PKG_AUTHORS"));
                    println!("{} {}", "License".green(), env!("CARGO_PKG_LICENSE"));
                    println!("{} {}", "Repository".green(), env!("CARGO_PKG_REPOSITORY"));
                } else {
                    error!("Invalid argument");
                }
            }
        }
    }
}
