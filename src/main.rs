use clap::error::Result;
use clap::{Parser, Subcommand};
use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(name = "SnippetStore")]
#[command(about = "Simple application for reading and writing short notes or snippets", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize SnippetStore directory!
    Init { snippetstore_dir: Option<String> },
    /// Read content of the specified snippet to standard output
    #[command(arg_required_else_help = true)]
    Read { snippet_name: Option<String> },
    /// Adds files to myapp
    #[command(arg_required_else_help = true)]
    New {
        #[arg(short = 'n', long = "name")]
        snippet_name: Option<String>,
        #[arg(short = 'c', long = "content")]
        content: Option<String>,
    },
    /// List out all snippets
    List,
}

#[derive(Debug)]
struct SnippetStoreError {
    message: String,
}

impl fmt::Display for SnippetStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SnippetStoreError {}

impl From<io::Error> for SnippetStoreError {
    fn from(error: io::Error) -> Self {
        SnippetStoreError {
            message: error.to_string(),
        }
    }
}

fn get_dir() -> PathBuf {
    let home_dir = env::var("HOME").expect("ENV variable HOME not found");
    let dir = PathBuf::from(home_dir).join(".local/share/snippetstore");

    if !dir.exists() {
        // let input = String::new();
        println!(
            "Couldn't find snippetstore folder, please initialize with 
                `snippetstore init <full path to folder>` or leave empty if
                if default path of ~/.local/share/snippetstore should be created>`"
        );
        io::stdout().flush().unwrap();
        fs::create_dir_all(&dir).expect("Could not create directory");
        // io::stdin()
        //     .read_line(&mut input)
        //     .expect("Failed to read line");
    }
    dir
}

fn create_script_dir<E: std::error::Error + From<io::Error>>(
    path: Option<String>,
) -> Result<(), E> {
    let env_key_path = String::from("SNIPPETSTORE_DIR");
    let dir = match path {
        Some(path) => PathBuf::from(path),
        None => {
            let home_dir = env::var("HOME").expect("ENV variable HOME not found");
            PathBuf::from(home_dir).join(".local/share/snippetstore")
        }
    };
    env::set_var(env_key_path, &dir);
    fs::create_dir_all(&dir)?;
    Ok(())
}

fn read_snippet<E: std::error::Error + From<io::Error>>(
    snippet_name: Option<String>,
) -> Result<(), E> {
    let dir = get_dir();
    let file = dir.join(snippet_name.expect("Wrong snippet name"));
    let content = fs::read_to_string(file)?;
    println!("Snippets content:");
    print!("{}", content);
    io::stdout().flush()?;
    Ok(())
}

fn new_snippet<E: std::error::Error + From<io::Error>>(
    file_name: Option<String>,
    content: Option<String>,
) -> Result<(), E> {
    let dir = get_dir();
    let file = dir.join(file_name.expect("Wrong file name"));
    fs::write(file, content.expect("Couldn't write to a file"))?;
    println!("Saved new snippet");
    Ok(())
}

fn list_snippets<E: std::error::Error + From<io::Error>>() -> Result<(), E> {
    let dir = get_dir();

    for (idx, snippet) in fs::read_dir(dir)?.enumerate() {
        let file = snippet?;
        let file = file.path();
        if !file.is_dir() {
            println!("{}: {}", idx, file.file_name().unwrap().to_string_lossy());
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    // let file_path = cli.file_path.as_deref().unwrap();
    // let content = cli.content.as_deref().unwrap();

    match cli.command {
        Commands::Init { snippetstore_dir } => {
            create_script_dir::<SnippetStoreError>(snippetstore_dir)?
        }
        Commands::Read { snippet_name } => read_snippet::<SnippetStoreError>(snippet_name)?,
        Commands::New {
            snippet_name,
            content,
        } => new_snippet::<SnippetStoreError>(snippet_name, content)?,
        Commands::List {} => list_snippets::<SnippetStoreError>()?,
    };
    // fs::write(dir.join(file_path), content).expect("something bad happened mate");
    // println!("In file {file_path}");
    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
