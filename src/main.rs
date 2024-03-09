use std::{fs, thread};
use std::env;
use std::path::PathBuf;
use std::time::Duration;

use bluetooth_battery::{Device, get_bluetooth_battery};
use resvg::{tiny_skia, usvg};
use resvg::tiny_skia::Pixmap;
use resvg::usvg::fontdb::Database;
use resvg::usvg::Options;
use tray_icon::{Icon, TrayIconBuilder};
use tray_icon::menu::{Menu, MenuItem};
use usvg::Tree;
use xmltree::Element;

fn convert_svg_to_png(svg_data: &str) {
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
  pixmap.save_png("resources/result.png").expect("Failed to save pixmap to PNG");
}

fn main() {
  loop {
    let mut devices = get_bluetooth_battery();

    while !devices.is_empty() {
      let device = devices
        .get(0)
        .unwrap();

      let tray_tooltip = get_tray_tooltip(device);
      let tray_icon = get_tray_icon(device);
      let tray_menu = get_tray_menu();

      let _tray = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip(tray_tooltip)
        .with_icon(tray_icon)
        .build()
        .unwrap();

      let minute = Duration::from_secs(60);
      thread::sleep(minute);
      devices = get_bluetooth_battery();
    }

    let minute = Duration::from_secs(1);
    thread::sleep(minute);
  }
}

fn get_tray_menu() -> Menu {
  let tray_menu = Menu::new();
  let quit_i = MenuItem::new("Exit".to_owned(), true, None);
  tray_menu.append_items(&[&quit_i]).ok();
  tray_menu
}

fn get_tray_icon(device: &Device) -> Icon {
  let battery = device.battery;
  create_icon(battery as i32);
  let icon = load_icon();
  cleanup_icon();

  icon
}

fn create_icon(percentage: i32) {
  let factor: f32 = percentage as f32 / 100.0;

  let data = fs::read_to_string("resources/icon.svg")
    .expect("Unable to read file");

  let percentage_fill = 1256.0 * factor;
  let mut names_element = Element::parse(data.as_bytes()).unwrap();

  let stroke_dasharray = format!("{}, 1256", percentage_fill);

  let name = names_element
    .get_mut_child("g")
    .unwrap()
    .get_mut_child("g")
    .unwrap()
    .get_mut_child("path")
    .expect("Can't find name element");

  name.attributes.remove("stroke-dasharray");
  name.attributes.insert("stroke-dasharray".to_owned(), stroke_dasharray.to_owned());

  let mut xml_data: Vec<u8> = Vec::new();
  names_element.write(&mut xml_data).unwrap();
  let xml_string = String::from_utf8(xml_data).unwrap();

  convert_svg_to_png(&xml_string);
}

fn get_tray_tooltip(device: &Device) -> String {
  let device_name = &device.name;
  let device_battery = &device.battery;

  return format!("{}: {}%", device_name, device_battery);
}

fn load_icon() -> Icon {
  let (icon_rgba, icon_width, icon_height) = {
    let icon_path = get_icon_path();

    let image_bytes = fs::read(icon_path)
      .expect("Failed to read icon file");

    let image = image::load_from_memory_with_format(&image_bytes, image::ImageFormat::Png)
      .expect("Failed to open icon path")
      .into_rgba8();

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
  };

  Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

fn cleanup_icon() {
  let buf = get_icon_path();

  fs::remove_file(buf).unwrap()
}

fn get_icon_path() -> PathBuf {
  let mut current_dir = env::current_dir()
    .expect("Failed to get current directory");

  current_dir.push("resources/result.png");

  current_dir
}
