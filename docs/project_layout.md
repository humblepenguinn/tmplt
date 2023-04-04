# Project Layout

The project structure of the Rust CLI tool Tmplt can be explained as follows:

## src folder

The `src` folder contains the source code for the `tmplt` CLI tool. It contains the following files:

- ``cli.rs`: This file contains the code for defining the command line interface (CLI) using the `clap` crate.
- `command.rs`: This file contains the code that implements the subcommands of the `tmplt` CLI tool.
- `error_macro.rs`: This file contains a custom macro for generating error messages.
- `main.rs`: This file contains the main function that is executed when `tmplt` is run.
- `parser.rs`: This file contains the code for parsing template files.
- `progress_bar.rs`: This file contains the code for displaying progress spinner while the template is running.
- `utils.rs`: This file contains utility functions used throughout the project.
