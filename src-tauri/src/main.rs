// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::process::Command;

mod vm_lib;
use vm_lib::vm_lib::{check, index_vms, show_vm};

#[derive(Serialize, Deserialize, Debug)]
struct Vm {
    name: String,
    state: String,
    ipv4: String,
    image: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct FullVm {
    name: String,
    state: String,
    snapshots: String,
    ipv4: String,
    release: String,
    image_hash: String,
    cpus: String,
    load: String,
    disk_usage: String,
    memory_usage: String,
    mounts: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn check_if_installed() -> String {
    let output = check();

    return output;
}

#[tauri::command]
fn list_vms() -> Vec<Vm> {
    let vms = index_vms();

    return vms;
}

#[tauri::command]
fn get_vm(name: &str) -> FullVm {
    let vm = show_vm(name);

    return vm;
}

#[tauri::command]
fn create_vm(name: &str) -> String {
    let cmd_result = Command::new("multipass")
        .args(["launch", "-n", name])
        .output();

    let output = match cmd_result {
        Ok(output) => String::from_utf8(output.stdout).unwrap(),
        Err(_error) => "Multipass is not installed".to_string(),
    };

    return output;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            check_if_installed,
            list_vms,
            get_vm,
            create_vm
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
