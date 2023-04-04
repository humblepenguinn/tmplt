
# tmplt

[![CICD](https://github.com/humblepenguinn/tmplt/actions/workflows/CICD.yml/badge.svg)](https://github.com/humblepenguinn/tmplt/workflows/CICD.yml)
[![Version info](https://img.shields.io/crates/v/tmplt.svg)](https://crates.io/crates/tmplt)

A User Friendly CLI Tool For Creating New Projects With Templates


<img alt="Demo" src="https://vhs.charm.sh/vhs-68JGlaLkwlzBKpo3wm0UEo.gif" width="800" />

## About

`tmplt` is a command-line tool that lets users quickly create new projects based on templates. With `tmplt`, users can create a new project that is set up with all the necessary files and dependencies, so they can get started on their project right away.

With `tmplt`, users can create templates that define the structure and dependencies of a new project. These templates can be customized to fit specific project needs and shared with others. `tmplt` comes with a simple yet flexible syntax for defining templates that can include variables, files, and dependencies.

Templates are defined in a `yaml` file that lists all the files to be generated, the dependencies to be installed, and the variables to be replaced.

Here's an example template for a Pygame project:

```yaml
# Pygame Project Template

# Template information
name: Pygame Project
description: A template for creating a Pygame project

# Dependency information
dependencies:
  - name: Pygame
    install_command: pip install pygame

# Files to generate
files:
  - name: main.py
    content: |
      import pygame

      # Set up pygame
      pygame.init()

      # Set up the display
      screen_width = {screen_width}
      screen_height = {screen_height}
      screen = pygame.display.set_mode((screen_width, screen_height))
      pygame.display.set_caption("{app_name}")

      # Game loop
      running = True
      while running:
          # Event handling
          for event in pygame.event.get():
              if event.type == pygame.QUIT:
                  running = False

          # Game logic

          # Draw to screen
          screen.fill((255, 255, 255))
          pygame.display.flip()

      # Clean up pygame
      pygame.quit()


# Variables
variables:
  - name: app_name
    description: The name of the pygame app
    default: my_pygame_app

  - name: screen_width
    description: The screen width of the pygame app
    default: 800

  - name: screen_height
    description: The screen height of the pygame app
    default: 600
```

## Templates
Templates are files that contain information about the project, its dependencies, files to generate, and variables. Users can use these templates to set up projects easily.

A template file is written in `YAML` format and contains the following information:

* Name: The name of the template.

* Description: A short description of the template.

* Dependencies: A list of dependencies required by the project. Each dependency contains a name and an install command.

* Files: A list of files to generate for the project. Each file contains a name and the content to be
written to the file.

* Variables: A list of variables that can be used in the template. Each variable contains a name, description, and default value.

Users can create their own templates or download them from the internet. To create a new project from a template, simply run the `tmplt new` command and provide the name of the template. The tool will generate all the necessary files and install the required dependencies.


## Installation

You can install `tmplt` through a few methods

### Releases

You can head over to the [releases page](https://github.com/humblepenguinn/tmplt/releases/latest) and download the official `tmplt` binaries from there for your target operating system. `Windows MSI installers` are also available

### Cargo Repository

You can install `tmplt` through the Cargo repository using the following command:

```sh
$ cargo install tmplt
```

### Source

Go [here](./docs/build_from_source.md) to see how

More methods of installation will be added in the future!

## Usage

Go [here](./docs/usage.md) to see how to use the tool


## Contributing

Contributions to `tmplt` are always welcome! Please see the [Contributing Guidelines](CONTRIBUTING.md) for more information.

## License

This project is licensed under the [MIT](LICENSE-MIT) License and the [Apache](LICENSE-APACHE) License
