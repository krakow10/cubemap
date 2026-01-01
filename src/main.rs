use clap::{Args, Parser, Subcommand};
use image::ImageReader;
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
	resolution: u32,
	#[arg(long)]
	input_file: PathBuf,
	#[arg(long)]
	output_folder: PathBuf,
}

fn main() {
	let cli = Cli::parse();
	match cli.command {
		Commands::EquirectangularToCubemap(subcommand) => subcommand.run().unwrap(),
	}
}

impl EquirectangularToCubemapSubcommand {
	fn run(self) -> anyhow::Result<()> {
		let file_name = self.input_file.file_stem().unwrap();
		let src_image = ImageReader::open(self.input_file.as_path())?
			.decode()?
			.into_rgba8();

		// our pixel type
		use image::Rgba;

		let mut dst_image = image::RgbaImage::new(self.resolution, self.resolution);
		for x in 0..self.resolution {
			for y in 0..self.resolution {
				dst_image.put_pixel(x, y, Rgba([255, 0, 0, 255]));
			}
		}

		let mut dst_path = self.output_folder;
		dst_path.push(file_name);
		dst_path.set_extension("png");
		dst_image.save(dst_path)?;

		Ok(())
	}
}
