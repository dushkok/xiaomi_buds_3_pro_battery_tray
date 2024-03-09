use std::fs;
use std::string::FromUtf8Error;

use xmltree::Element;

const SVG_FILE_PATH: &'static str = "resources/icon.svg";
const LOW_BATTERY_COLOR: &'static str = "#E53C33";

pub fn create_icon(percentage: i32) -> Result<String, FromUtf8Error> {
  let binding = read_svg()
    .expect("Error reading svg");

  let bytes = binding.as_bytes();
  let mut names_element = Element::parse(bytes).unwrap();

  adapt_battery_gauge_length(percentage, &mut names_element);

  if percentage < 20 {
    adapt_battery_gauge_color(&mut names_element);
  }

  let mut xml_data: Vec<u8> = Vec::new();
  names_element.write(&mut xml_data).unwrap();

  String::from_utf8(xml_data)
}

fn read_svg() -> std::io::Result<String> {
  fs::read_to_string(SVG_FILE_PATH)
}

fn adapt_battery_gauge_color(names_element: &mut Element) {
  let path: &[&str] = &["g", "g", "defs", "linearGradient", "stop"];
  let attribute_name = String::from("stop-color");
  change_attribute(path, names_element, attribute_name, LOW_BATTERY_COLOR.to_owned());
}

fn adapt_battery_gauge_length(percentage: i32, names_element: &mut Element) {
  let factor: f32 = percentage as f32 / 100.0;
  let percentage_fill = 1256.0 * factor;
  let stroke_dasharray = format!("{}, 1256", percentage_fill);

  let path: &[&str] = &["g", "g", "path"];
  let attribute_name = String::from("stroke-dasharray");

  change_attribute(path, names_element, attribute_name, stroke_dasharray);
}

fn change_attribute(
  nested_element_path: &[&str],
  names_element: &mut Element,
  attribute_name: String,
  attribute_new_value: String) {
  let mut element: &mut Element = names_element;

  for &nested_element in nested_element_path {
    element = element.get_mut_child(nested_element).unwrap()
  }

  let found_element = element;

  found_element.attributes.remove(&attribute_name);
  found_element.attributes.insert(attribute_name, attribute_new_value.to_owned());
}