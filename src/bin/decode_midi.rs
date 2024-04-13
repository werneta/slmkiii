use clap::Parser;
// use slmkiii::midi::{EOX, SOX};

/// MIDI SysEx decoder
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// SysEx file to decode
    #[arg()]
    fname: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("{:?}", args.fname);
}
