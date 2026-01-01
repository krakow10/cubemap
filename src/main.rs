use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author,version,about,long_about=None)]
#[command(propagate_version = true)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	EquirectangularToCubemap(EquirectangularToCubemapSubcommand),
}

/// Reproject an equirectanular image to a cubemap
#[derive(Args)]
struct EquirectangularToCubemapSubcommand {
	#[arg(long)]
	input_file: PathBuf,
	#[arg(long)]
	output_folder: PathBuf,
}

fn main() {
	let cli = Cli::parse();
	match cli.command {
		Commands::EquirectangularToCubemap(subcommand) => subcommand.run(),
	}
}

impl EquirectangularToCubemapSubcommand {
	fn run(self) {
		println!("Hello world!");
	}
}
