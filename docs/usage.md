# Usage
Before reading the usage make sure you understand what `templates` (See [templates](https://github.com/humblepenguinn/tmplt#templates)) are in `tmplt`

## Creating new Template
To use `tmplt`, users need to first create or [import](#import-a-template) a template. Running `tmplt init <template_name>` will generate a new template with the provided name, which you can then find in the tools config directory `HOME_DIR/.tmplt/templates` (note that `HOME_DIR` refers to the operating systems home directory). You can now modify this template using a text editor.

Lets create a new `flask app` template using `tmplt`

```sh
$ tmplt init flask-app
```

This will create a new template named "flask-app" in the `HOME_DIR/.tmplt/templates` directory.

Open the template with a text editor. This step is specific to your operating system and text editor of choice

When you open the template file using a text editor you should see the following contents:
```yaml
# Go to the GitHub repo https://github.com/humblepenguinn/tmplt for more information

# Template information
name: flask-app
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
post_setup_commands:
  - command_1
  - command_2

variables:
  - name: my_variable_name
    description: description of the variable
    default: the default value of the variable
```

Modify the contents of the template:
```yaml
# Flask Project Template

# Template information
name: Flask Project
description: A template for creating a Flask project

# Dependency information
dependencies:
  - name: Flask
    install_command: pip install flask

# Files to generate
files:
  - name: app.py
    content: |
      from flask import Flask

      app = Flask(__name__)

      @app.route("/")
      def hello_world():
          return "<p>Hello, World from {app_name}!</p>"

      if __name__ == "__main__":
          app.run()


# Variables
variables:
  - name: app_name
    description: The name of the Flask app
    default: my_flask_app

```

We now have successfully created our first template!

## Create a new project
To create a new project using a template, users can run the `tmplt create <proj_name> <template_to_use>` command. This command will create a new directory with the name specified in the `<proj_name>` argument, generate all the files, download all the dependencies and run all the `post` and `setup` commands specified in the template.

If the template has any `variables`, `tmplt` will prompt you to provide a value for each variable. You can choose to use the default value or provide a new value.

Let's use our flask template to create a new flask project:
```sh
$ tmplt create myflaskapp flask-app
```

This will create a new directory with the name `myflaskapp`, based on the `flask-app` template we created earlier. The command will prompt you to provide a value for the `app_name` variable.

## Import a Template

In addition to being able to create your own templates, users can also import templates created by other users!

The `tmplt import <url> <template_name_to_save_as>` command is used to import a template from a URL. Running `tmplt import` will download the template from the URL and save it with the name you provided.

You can then use the imported template to create a new project as showed in the [Create a new project section](#create-a-new-project)

## List Available Templates
This command is used to list all the available templates. Running `tmplt list` will display a list of all the templates that you can use to create projects.

```sh
tmplt list
```

That's it! With these simple commands, you can use `tmplt` to create and manage projects based on templates.
