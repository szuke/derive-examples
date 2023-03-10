use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add { name: Option<String> },
    /// Zuke - almighty developer
    Zuke { name: Option<String> },
    /// Prints hhllo world
    Hello { name: Option<String> },
}

fn hello() {
    println!("Hello, world!");
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add { name } => {
            println!("'myapp add' was used, name is: {:?}", name)
        }
        Commands::Zuke { name } => {
            println!("'zuke add' was used, name is: {:?}", name)
        }        
        Commands::Hello { name } => {
            println!("'myapp hello' was used, name is: {:?}", name);
            hello();
        }
    }
}