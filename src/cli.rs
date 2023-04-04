use clap::Args;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Args)]
pub struct CommandArgs {
    pub args: Vec<String>,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    #[clap(name = "new", about = "Create a new project")]
    New(CommandArgs),
    #[clap(name = "init", about = "Create a new template")]
    Init(CommandArgs),
    #[clap(name = "import", about = "Import a template")]
    Import(CommandArgs),
    #[clap(name = "list", about = "List all templates")]
    List,
    #[clap(name = "remove", about = "Remove a template")]
    Remove(CommandArgs),
    #[clap(name = "version", about = "Show version")]
    Version(CommandArgs),
}
