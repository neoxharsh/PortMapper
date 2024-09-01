use std::{os::windows::process::CommandExt, process::Command};

use egui::{ahash::HashMap, ScrollArea};
use egui_extras::TableBuilder;
mod drawui;

const CREATE_NO_WINDOW: u32 = 0x08000000;


#[derive(Clone,Debug,PartialEq)]
pub enum Protocol {
    V4TOV4,
    V4TOV6,
    V6TOV6,
    V6TOV4,
}

#[derive(Debug)]
pub struct PortMap {
    listenaddress: String,
    listenport: String,
    connectport: String,
    connectaddress: String,
    protocol: Protocol,
}
pub struct PortMapper {
    ActiveMapList: Vec<PortMap>,
    selectedPortMapIndex: usize,
    selectedPortMap: PortMap,
    lastStatus: String
}

impl PortMapper {
    pub fn refreshActiveMapping(&mut self) {
        self.ActiveMapList = Self::getAllActiveMapping();
    }

    pub fn getActiveMapping(protocol: &str) -> Vec<PortMap> {
        let mut activePortMaps = Vec::<PortMap>::new();
        let output = Command::new("cmd")
            .args(&["/c", "netsh", "interface", "portproxy", "show", protocol])
        .creation_flags(CREATE_NO_WINDOW)
            .output()
            .expect("Error");
        let outputString = String::from_utf8_lossy(&output.stdout);
        if outputString.len() > 2{
            let outputString: Vec<&str> = outputString.split("\n").collect();
            for line in &outputString[5..] {
                if line == &"\r" {
                    break;
                }
                let data: Vec<&str> = line.split_whitespace().collect();
                activePortMaps.push(PortMap {
                    listenaddress: String::from(data[0]),
                    connectaddress: String::from(data[2]),
                    protocol: match protocol {
                        "v4tov4" => Protocol::V4TOV4,
                        "v4tov6" => Protocol::V4TOV6,
                        "v6tov4" => Protocol::V6TOV4,
                        "v6tov6" => Protocol::V6TOV6,
                        _ => Protocol::V4TOV4,
                    },
                    listenport: String::from(data[1]),
                    connectport: String::from(data[3]),
                })
            }
        }
       
        return activePortMaps;
    }

    pub fn getAllActiveMapping() -> Vec<PortMap> {
        let mut activePortMaps: Vec<PortMap> = self::PortMapper::getActiveMapping("v4tov4");
        activePortMaps.extend(self::PortMapper::getActiveMapping("v4tov6"));
        activePortMaps.extend(self::PortMapper::getActiveMapping("v6tov6"));
        activePortMaps.extend(self::PortMapper::getActiveMapping("v6tov4"));
        return activePortMaps;
    }
}

impl Default for PortMapper {
    fn default() -> Self {
        let mut activePortMaps: Vec<PortMap> = self::PortMapper::getAllActiveMapping();
        PortMapper {
            ActiveMapList: activePortMaps,
            selectedPortMapIndex: 0,
            selectedPortMap: PortMap {
                listenaddress: String::new(),
                connectaddress: String::new(),
                protocol: Protocol::V4TOV4,
                listenport: String::from("0"),
                connectport: String::from("0"),
            },
            lastStatus: String::new()
        }
    }
}

impl eframe::App for PortMapper {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.drawActivePortMappingsTable(ui);
            ui.separator();
            self.drawEditorSection(ui);
            ui.separator();
            self.drawStatusBar(ui); 
        });
    }
}
