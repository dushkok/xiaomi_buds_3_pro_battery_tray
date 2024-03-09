use bluetooth_battery::Device;
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};
use tray_icon::menu::{Menu, MenuItem};

use crate::factory::icon_factory::create_tray_icon;

pub fn create_tray(device: &Device) -> TrayIcon {

  let tray_tooltip = create_tooltip(device);
  let tray_icon = create_icon(device);
  let tray_menu = create_menu();

  TrayIconBuilder::new()
    .with_menu(Box::new(tray_menu))
    .with_tooltip(tray_tooltip)
    .with_icon(tray_icon)
    .build()
    .unwrap()
}

fn create_tooltip(device: &Device) -> String {
  let device_name = &device.name;
  let device_battery = &device.battery;

  return format!("{}: {}%", device_name, device_battery);
}

fn create_icon(device: &Device) -> Icon {
  let battery = device.battery;

  create_tray_icon(battery as i32)
}

fn create_menu() -> Menu {
  let tray_menu = Menu::new();
  let quit_i = MenuItem::new("Exit".to_owned(), true, None);
  tray_menu.append_items(&[&quit_i]).ok();

  tray_menu
}