use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  Play(PlayArgs),
  Decode(DecodeArgs),
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct PlayArgs {
  #[clap(value_parser)]
  #[arg(short, long)]
  pub source: PathBuf,
  #[arg(short, long, default_value_t = 1)]
  pub channels: u16,
  #[arg(short = 'r', long, default_value_t = 16000)]
  pub sample_rate: u32,
  #[arg(short, long, default_value_t = 16)]
  pub bits: u16,
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct DecodeArgs {
  #[clap(value_parser)]
  #[arg(short, long)]
  pub source: PathBuf,
  #[arg(short, long)]
  pub dest: PathBuf,
}
