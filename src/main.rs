use bluetooth_battery::get_bluetooth_battery;

use crate::factory::tray_factory::create_tray;

mod factory;

fn main() {
  loop {
    let mut devices = get_bluetooth_battery();

    while !devices.is_empty() {
      let device = devices
        .get(0)
        .unwrap();

      let _icon = create_tray(device);

      devices = get_bluetooth_battery();
    }
  }
}
