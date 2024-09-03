#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)]
mod libs;

use eframe::egui;
use libs::gui_builder::PortMapper;


fn main() -> eframe::Result 
{
   
 let options = eframe::NativeOptions{
   
    viewport: egui::ViewportBuilder::default().with_inner_size([640.0,420.0]).with_icon(
      eframe::icon_data::from_png_bytes(include_bytes!("icon.png"))
          .unwrap_or_default(),
  ),
    ..Default::default()
 };

 eframe::run_native("Port Mapper", options, Box::new(|cc|{
    Ok(Box::<PortMapper>::default())
 }))
}