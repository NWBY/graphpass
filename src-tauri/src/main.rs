// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::{
    fmt::format,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

#[derive(Serialize, Deserialize, Debug)]
struct Vm {
    name: String,
    state: String,
    ipv4: String,
    image: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn check_if_installed() -> String {
    let cmd_result = Command::new("multipass").output();

    let output = match cmd_result {
        Ok(output) => "Multipass is installed! 🥳".to_string(),
        Err(error) => "Multipass is not installed".to_string(),
    };

    return output;
}

#[tauri::command]
fn list_vms() -> Vec<Vm> {
    let cmd_result = Command::new("multipass").arg("list").output().expect("msg");

    let mut vms: Vec<Vm> = vec![];

    for (index, line) in cmd_result.stdout.lines().enumerate() {
        if index == 0 {
            // This is the first line that contains table headers, we skip this
            continue;
        }

        let val: String = line.unwrap();

        let split = val.split(" ");
        let mut split_vec: Vec<&str> = split.collect();
        split_vec.retain(|&x| x != "");
        

        let mut image: String = format!(
            "{} {}",
            split_vec[3].to_string(),
            split_vec.get(4).unwrap(),
        );
        
        if split_vec.get(5).is_some() {
            image = format!("{} {}", image, split_vec[5].to_string());    
        }

        let vm = Vm {
            name: split_vec[0].to_string(),
            state: split_vec[1].to_string(),
            ipv4: split_vec[2].to_string(),
            image: image,
        };

        vms.push(vm)
    }

    return vms;
}

#[tauri::command]
fn create_vm(name: &str) -> String {
    let cmd_result = Command::new("multipass")
        .args(["launch", "-n", name])
        .output();

    let output = match cmd_result {
        Ok(output) => String::from_utf8(output.stdout).unwrap(),
        Err(error) => "Multipass is not installed".to_string(),
    };

    return output;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            check_if_installed,
            list_vms,
            create_vm
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
