use egui::{Color32, Response, RichText, Sense, Separator, TextBuffer, TextEdit, Ui, Widget};
use egui_extras::{Column, Table, TableBuilder};
use regex::Regex;
use std::{borrow::Borrow, fmt::format, os::windows::process::CommandExt, process::{Command, ExitCode, ExitStatus}};

use super::{PortMap, PortMapper, Protocol};
const CREATE_NO_WINDOW: u32 = 0x08000000;
impl PortMapper {
    pub fn drawActivePortMappingsTable(&mut self, ui: &mut Ui) {
        egui::Frame::default().show(ui, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .auto_shrink(false)
                .resizable(true)
                .column(Column::remainder())
                .column(Column::remainder())
                .column(Column::remainder())
                .column(Column::remainder())
                .column(Column::remainder())
                .max_scroll_height(20.0)
                .sense(Sense::click())
                .header(10.0, |mut row| {
                    row.col(|ui| {
                        ui.strong(egui::RichText::from("Listen Address").size(15.0));
                    });
                    row.col(|ui| {
                        ui.strong(egui::RichText::from("Listen Port").size(15.0));
                    });
                    row.col(|ui| {
                        ui.strong(egui::RichText::from("Connect Address").size(15.0));
                    });
                    row.col(|ui| {
                        ui.strong(egui::RichText::from("Connect Port").size(15.0));
                    });
                    row.col(|ui| {
                        ui.strong(egui::RichText::from("Protocol").size(15.0));
                    });
                })
                .body(|mut body| {
                    for x in &self.ActiveMapList {
                        body.row(10.0, |mut row| {
                            if self.selectedPortMapIndex == row.index() {
                                row.set_selected(true);
                            } else {
                                row.set_selected(false);
                            }
                            row.col(|ui| {
                                ui.label(format!("{}", x.listenaddress));
                            });
                            row.col(|ui| {
                                ui.label(format!("{}", x.listenport));
                            });
                            row.col(|ui| {
                                ui.label(format!("{}", x.connectaddress));
                            });
                            row.col(|ui| {
                                ui.label(format!("{}", x.connectport));
                            });
                            row.col(|ui| {
                                ui.label(format!(
                                    "{}",
                                    match x.protocol {
                                        Protocol::V4TOV4 => "V4TOV4",
                                        Protocol::V4TOV6 => "V4TOV6",
                                        Protocol::V6TOV4 => "V6TOV4",
                                        Protocol::V6TOV6 => "V6TOV6",
                                    }
                                ));
                            });
                            if row.response().clicked() {
                                self.selectedPortMapIndex = row.index();
                                self.selectedPortMap = PortMap {
                                    connectaddress: x.connectaddress.clone(),
                                    connectport: x.connectport.clone(),
                                    listenaddress: x.listenaddress.clone(),
                                    listenport: x.listenport.clone(),
                                    protocol: x.protocol.clone(),
                                };
                            }
                        });
                    }
                });
        });
    }

    pub fn drawEditorSection(&mut self, ui: &mut Ui) {
        // ui.columns(2, |columns| {
        //     drawButtons(&mut columns[0]);
        //     drawEditor(&mut columns[1]);
        // });
        egui::Grid::new("Editor")
            .num_columns(2)
            .max_col_width(200.0)
            .spacing([10.0, 5.0])
            .show(ui, |ui| {
                self.drawButtons(ui);
                self.drawEditor(ui);
                ui.end_row()
            });
    }

    fn drawButtons(&mut self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            if ui.button(RichText::new("Refresh").size(20.0)).clicked() {
                println!("Refreshed");
                self.refreshActiveMapping();
                self.lastStatus = String::from("Refreshed");
            };
            ui.add_space(10.0);
            if ui.button(RichText::new("Add/Update").size(20.0)).clicked() {
                println!("Added");
                let output = Command::new("cmd")
                    .args(&[
                        "/c",
                        "netsh",
                        "interface",
                        "portproxy",
                        "add",
                        match self.selectedPortMap.protocol {
                            Protocol::V4TOV4 => "v4tov4",
                            Protocol::V4TOV6 => "v4tov6",
                            Protocol::V6TOV4 => "v6tov4",
                            Protocol::V6TOV6 => "v6tov6",
                        },
                        format!("listenport={}",self.selectedPortMap.listenport).as_str(),
                        format!("listenaddress={}",self.selectedPortMap.listenaddress).as_str(),
                        format!("connectport={}",self.selectedPortMap.connectport).as_str(),
                        format!("connectaddress={}",self.selectedPortMap.listenaddress).as_str(),
                    ])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output()
                    .expect("Error");
                let outputString = String::from_utf8_lossy(&output.stdout);
                let tempLastMessage = format!("Added: {outputString}");
                self.lastStatus = outputString.clone().to_string();
                if output.status.success(){
                    self.lastStatus = format!("{:?}",self.selectedPortMap);
                }
                
                self.refreshActiveMapping();
            };
            // ui.add_space(10.0);
            // if ui.button(RichText::new("Update").size(20.0)).clicked() {
            //     println!("Updated");
            //     println!("{}",self.selectedPortMap.connectaddress);
            // };
            ui.add_space(10.0);
            if ui.button(RichText::new("Delete").size(20.0)).clicked() {
                println!("Deleted {:?}", self.selectedPortMap);
                let output = Command::new("cmd")
                    .args(&[
                        "/c",
                        "netsh",
                        "interface",
                        "portproxy",
                        "delete",
                        match self.selectedPortMap.protocol {
                            Protocol::V4TOV4 => "v4tov4",
                            Protocol::V4TOV6 => "v4tov6",
                            Protocol::V6TOV4 => "v6tov4",
                            Protocol::V6TOV6 => "v6tov6",
                        },
                        format!("listenport={}",self.selectedPortMap.listenport).as_str(),
                        format!("listenaddress={}",self.selectedPortMap.listenaddress).as_str(),
                    ])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output()
                    .expect("Error");
                let outputString = String::from_utf8_lossy(&output.stdout);
                let tempLastMessage = format!("Deleted: {outputString}");
                self.lastStatus = outputString.clone().to_string();
                if output.status.success(){
                    self.lastStatus = format!("{:?}",self.selectedPortMap);
                }
                
                self.refreshActiveMapping();
            };
        });
    }

    fn drawEditor(&mut self, ui: &mut Ui) {
        egui::Grid::new("Editor")
            .num_columns(2)
            .spacing([40.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Connect Address");
                ui.text_edit_singleline(&mut self.selectedPortMap.connectaddress);
                ui.end_row();

                ui.label("Connect Port");
                ui.text_edit_singleline(&mut self.selectedPortMap.connectport);
                ui.end_row();

                ui.label("Listen Address");
                ui.text_edit_singleline(&mut self.selectedPortMap.listenaddress);
                ui.end_row();

                ui.label("Listen Port");
                ui.text_edit_singleline(&mut self.selectedPortMap.listenport);
                ui.end_row();

                ui.label("Protocol");
                egui::ComboBox::from_id_source("Protocol")
                    .selected_text(format!("{:?}", self.selectedPortMap.protocol))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.selectedPortMap.protocol,
                            Protocol::V4TOV4,
                            "V4TOV4",
                        );
                        ui.selectable_value(
                            &mut self.selectedPortMap.protocol,
                            Protocol::V4TOV6,
                            "V4TOV6",
                        );
                        ui.selectable_value(
                            &mut self.selectedPortMap.protocol,
                            Protocol::V6TOV4,
                            "V6TOV4",
                        );
                        ui.selectable_value(
                            &mut self.selectedPortMap.protocol,
                            Protocol::V6TOV6,
                            "V6TOV6",
                        );
                    });
                ui.end_row();
            });

        // println!("{a}");
    }

    fn selectRow(&mut self) {}

    pub fn drawStatusBar(&self,ui: &mut Ui) {
        ui.columns(1, |columns|{
            columns[0].label(RichText::new(&self.lastStatus).size(15.0).color(Color32::RED));
        })
    }
}
