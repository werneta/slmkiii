use clap::Parser;
use slmkiii::midi::split_sysex_msgs;
use std::fs::read;

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

    let data = read(&args.fname).unwrap();
    let msgs = split_sysex_msgs(&data);

    for (n, msg) in msgs.iter().enumerate() {
        println!("Message {}:\n{}", n, msg);
    }
}
