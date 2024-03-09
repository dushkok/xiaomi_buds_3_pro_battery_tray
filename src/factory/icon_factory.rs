use resvg::tiny_skia;
use resvg::tiny_skia::Pixmap;
use resvg::usvg::{Options, Tree};
use resvg::usvg::fontdb::Database;
use tray_icon::Icon;

use crate::factory::svg_factory;

pub fn create_tray_icon(percentage: i32) -> Icon {
  let binding = svg_factory::create_icon(percentage)
    .expect("Error while creating svg icon");

  let svg = binding.as_str();
  let png_bytes = convert_svg_to_png(svg);

  create_icon_from_png(png_bytes)
}

fn create_icon_from_png(image_bytes: Vec<u8>) -> Icon {
  let (icon_rgba, icon_width, icon_height) = {
    let image = image::load_from_memory_with_format(&image_bytes, image::ImageFormat::Png)
      .expect("Failed to open icon path")
      .into_rgba8();

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
  };

  Icon::from_rgba(icon_rgba, icon_width, icon_height)
    .expect("Failed to open icon")
}

fn convert_svg_to_png(svg_data: &str) -> Vec<u8> {
  let opt = Options::default();

  let font_db = Database::new();

  let rtree = Tree::from_str(svg_data, &opt, &font_db)
    .unwrap();

  let height = rtree
    .size()
    .height();

  let width = rtree
    .size()
    .width();

  let mut pixmap = Pixmap::new(width as u32, height as u32).unwrap();
  resvg::render(&rtree, tiny_skia::Transform::identity(), &mut pixmap.as_mut());

  pixmap
    .encode_png()
    .expect("Error while encoding png")
}
