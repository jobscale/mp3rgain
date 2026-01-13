use anyhow::Result;
use mp3rgain::play_mp3;
use std::env;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let mut files = Vec::new();
    let mut start: Option<f64> = None;
    let mut duration: Option<f64> = None;
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];

        if arg == "--help" || arg == "-h" {
            print_usage();
            return Ok(());
        }

        if arg == "--version" || arg == "-v" {
            print_version();
            return Ok(());
        }

        if arg == "--play" {
            // Parse optional start and duration parameters
            if i + 1 < args.len() {
                if let Ok(v) = args[i + 1].parse::<f64>() {
                    start = Some(v);
                    i += 1;

                    if i + 1 < args.len() {
                        if let Ok(v2) = args[i + 1].parse::<f64>() {
                            duration = Some(v2);
                            i += 1;
                        }
                    }
                }
            }
            i += 1;
            continue;
        }

        if arg.starts_with('-') {
            eprintln!("Unknown option: {}", arg);
            std::process::exit(1);
        }

        files.push(PathBuf::from(arg));
        i += 1;
    }

    if files.is_empty() {
        eprintln!("No files specified");
        std::process::exit(1);
    }

    // Play all files
    for file in files {
        play_mp3(&file, start, duration)?;
    }

    Ok(())
}

fn print_version() {
    println!("mp3rgain version {}", VERSION);
    println!("A modern mp3gain replacement written in Rust");
}

fn print_usage() {
    println!("mp3rgain version {}", VERSION);
    println!("Play MP3 files");
    println!();
    println!("USAGE:");
    println!("    mp3rgain <FILE>...");
    println!("    mp3rgain --play [START] [DURATION] <FILE>...");
    println!();
    println!("OPTIONS:");
    println!("    --play              Play MP3 files");
    println!("    --help, -h          Show this help");
    println!("    --version, -v       Show version");
    println!();
    println!("EXAMPLES:");
    println!("    mp3rgain song.mp3                    Play song");
    println!("    mp3rgain song1.mp3 song2.mp3         Play multiple files");
    println!("    mp3rgain --play 10.2 5.8 song.mp3        Play from 10s for 5s");
}
