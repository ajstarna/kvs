use clap::{Parser, Subcommand};
use kvs::KvStore;

/// Simple program to greet a person
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet

    /*
    #[clap(short, long)]    
    name: String,
    
    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
     */

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Set {
        key: String,
        value: String        
    },
    Get {
        key: String,        
    },
    Rm {
        key: String,        
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Set { key, value } => {
            println!("'kvs set' was used, key = {:?}, value = {:?}", key, value);
            eprintln!("unimplemented");
            panic!();
        },
        Commands::Get { key } => {
            println!("'kvs get' was used, key = {:?}", key);
            eprintln!("unimplemented");
            panic!();
        }
        Commands::Rm { key } => {
            println!("'kvs rm' was used, key = {:?}", key);
            eprintln!("unimplemented");
            panic!();
        }
    }
}
