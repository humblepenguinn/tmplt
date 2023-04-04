use std::error::Error;
use std::{fs::File, io::BufReader};

use inquire::Text;
use serde_yaml::Value;

use crate::error;

pub struct Template {
    pub name: String,
    pub description: String,
    pub setup_commands: Option<Vec<String>>,
    pub dependencies: Option<Vec<(String, String)>>,
    pub files: Option<Vec<(String, String)>>,
    pub post_setup_commands: Option<Vec<String>>,
}

fn get_variable_value(name: &str, desc: &str, default_value: &str) -> String {
    let value = Text::new(&format!("Enter value for variable [{}]:", name))
        .with_help_message(&format!(
            "{}\nPress enter to use default value: {}",
            desc, default_value,
        ))
        .with_default(default_value)
        .prompt();

    if let Ok(value) = value {
        value.trim_matches('"').to_owned()
    } else {
        error!(format!("Failed to get value for variable: {}", name));
        std::process::exit(1);
    }
}

pub fn parse(filepath: &str) -> Result<Template, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let data: Value = serde_yaml::from_reader(reader)?;

    let mut files: Option<Vec<(String, String)>>;

    files = if let Some(files_data) = data["files"].as_sequence() {
        let mut buffer = Vec::new();

        for file_data in files_data {
            let file_data = file_data.as_mapping().ok_or("Expected mapping")?;
            let name = file_data["name"]
                .as_str()
                .ok_or(format!(
                    "Expected file name [{:?}] as str",
                    file_data["name"]
                ))?
                .to_owned();

            let content = file_data["content"]
                .as_str()
                .ok_or(format!("Expected content for file [{}] as str", name))?
                .to_owned();
            buffer.push((name, content));
        }
        Some(buffer)
    } else {
        None
    };

    let variables = if let Some(variables) = data["variables"].as_sequence() {
        Some(
            variables
                .iter()
                .map(|x| {
                    let x = x.as_mapping().ok_or("Expected mapping")?;
                    let name = x["name"].as_str().ok_or("Expected name as str")?.to_owned();
                    let default_value = serde_json::to_string(&x["default"]).unwrap_or_else(|_| {
                        error!(format!(
                            "Failed to parse default value for variable: {}",
                            name
                        ));
                        std::process::exit(1);
                    });
                    let description = x["description"]
                        .as_str()
                        .ok_or(format!(
                            "Expected description [{:?}] as str",
                            x["description"]
                        ))?
                        .to_owned();
                    Ok((name, description, default_value))
                })
                .collect::<Result<Vec<(String, String, String)>, Box<dyn Error>>>()?,
        )
    } else {
        None
    };

    let mut name = data["name"]
        .as_str()
        .ok_or(format!("Expected name [{:?}] as str", data["name"]))?
        .to_owned();

    let mut description = data["description"]
        .as_str()
        .ok_or(format!("Expected description for template {} as str", name))?
        .to_owned();

    let mut setup_commands = if let Some(setup_commands) = data["setup_commands"].as_sequence() {
        Some(
            setup_commands
                .iter()
                .map(|x| Ok(x.as_str().ok_or("Expected str").unwrap().to_owned()))
                .collect::<Result<Vec<String>, Box<dyn Error>>>()?,
        )
    } else {
        None
    };

    let mut dependencies = if let Some(dependencies) = data["dependencies"].as_sequence() {
        Some(
            dependencies
                .iter()
                .map(|x| {
                    let x = x.as_mapping().ok_or("Expected mapping")?;
                    let name = x["name"].as_str().ok_or("Expected name as str")?.to_owned();
                    let install_command = x["install_command"]
                        .as_str()
                        .ok_or("Expected install_command as str")?
                        .to_owned();
                    Ok((name, install_command))
                })
                .collect::<Result<Vec<(String, String)>, Box<dyn Error>>>()?,
        )
    } else {
        None
    };

    let mut post_setup_commands =
        if let Some(post_setup_commands) = data["post_setup_commands"].as_sequence() {
            Some(
                post_setup_commands
                    .iter()
                    .map(|x| Ok(x.as_str().ok_or("Expected str").unwrap().to_owned()))
                    .collect::<Result<Vec<String>, Box<dyn Error>>>()?,
            )
        } else {
            None
        };

    if let Some(variables) = variables {
        for (var_name, desc, default_value) in variables {
            let value = get_variable_value(&var_name, &desc, &default_value);

            name = name.replace(&format!("{{{}}}", var_name), &value);
            description = description.replace(&format!("{{{}}}", var_name), &value);

            if let Some(commands) = &mut setup_commands {
                for command in commands.iter_mut() {
                    *command = command.replace(&format!("{{{}}}", var_name), &value);
                }
            }

            if let Some(dependencies) = &mut dependencies {
                for (name, install_command) in dependencies.iter_mut() {
                    *name = name.replace(&format!("{{{}}}", var_name), &value);
                    *install_command =
                        install_command.replace(&format!("{{{}}}", var_name), &value);
                }
            };

            if let Some(post_setup_commands) = &mut post_setup_commands {
                for command in post_setup_commands.iter_mut() {
                    *command = command.replace(&format!("{{{}}}", var_name), &value);
                }
            }

            if let Some(files) = &mut files {
                for (name, content) in files.iter_mut() {
                    *name = name.replace(&format!("{{{}}}", name), &value);
                    *content = content.replace(&format!("{{{}}}", var_name), &value);
                }
            }
        }
    }

    Ok(Template {
        name,
        description,
        setup_commands,
        dependencies,
        files,
        post_setup_commands,
    })
}
