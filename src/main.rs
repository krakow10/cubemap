use clap::{Args, Parser, Subcommand};
use glam::Vec3Swizzles;
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
	CylindricalToCubemap(CylindricalToCubemapSubcommand),
}

/// Reproject an cylindrical image to a cubemap
#[derive(Args)]
struct CylindricalToCubemapSubcommand {
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
		Commands::CylindricalToCubemap(subcommand) => subcommand.run().unwrap(),
	}
}

impl CylindricalToCubemapSubcommand {
	fn run(self) -> anyhow::Result<()> {
		let res = self.resolution;
		let file_name = self.input_file.file_stem().unwrap();

		// input image in cylindrical coordinates
		let src_image = ImageReader::open(self.input_file.as_path())?
			.decode()?
			.into_rgba8();
		let (src_w, src_h) = src_image.dimensions();

		// let's project a cylinder of radius 1 and height 2
		// onto our cubemap around the origin of size 2

		// === BACK ===
		// X right
		// Y up
		// Z backwards
		let mut dst_image = image::RgbaImage::new(res, res);
		for x in 0..res {
			for y in 0..res {
				// this is the position of the point on the cubemap
				let cubemap_pos =
					glam::Vec3A::new(x as f32 / res as f32, y as f32 / res as f32, -1.0);

				// Find cylinder yaw angle
				let cylinder_x = cubemap_pos.x.atan2(cubemap_pos.z);
				// Find cylinder height
				let cylinder_y = cubemap_pos.y / cubemap_pos.xz().length();

				// convert to src image pixels
				let src_x =
					(cylinder_x * src_w as f32 / core::f32::consts::TAU).rem_euclid(src_w as f32);
				let src_y = (cylinder_y + 1.0) * src_h as f32 / 2.0;

				// copy src color to dst pixel
				let src_pixel = src_image.get_pixel(src_x as u32, src_y as u32);
				dst_image.put_pixel(x, y, *src_pixel);
			}
		}

		let mut dst_path = self.output_folder;
		dst_path.push(file_name);
		dst_path.set_extension("png");
		dst_image.save(dst_path)?;

		Ok(())
	}
}
