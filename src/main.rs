use args::{DecodeArgs, PlayArgs};
use clap::Parser;

pub mod args;
pub mod parse;

fn main() {
  let cli = args::Cli::parse();
  match cli.command {
    args::Commands::Play(args) => {
      let PlayArgs {
        source,
        channels,
        sample_rate,
        bits,
      } = args;
      let _ = parse::play_pcm(&source, channels, sample_rate, bits);
    }
    args::Commands::Decode(args) => {
      let DecodeArgs { source, dest } = args;
      let _ = parse::decode_pcm_save(&source, &dest);
    }
  };
}
