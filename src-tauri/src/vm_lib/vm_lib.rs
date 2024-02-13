use std::{io::BufRead, process::Command};

use crate::{FullVm, Vm};

pub fn check() -> String {
    let cmd_result = Command::new("multipass").output();

    let output = match cmd_result {
        Ok(_output) => "Multipass is installed! 🥳".to_string(),
        Err(_error) => "Multipass is not installed".to_string(),
    };

    return output;
}

pub fn index_vms() -> Vec<Vm> {
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

        let mut image: String =
            format!("{} {}", split_vec[3].to_string(), split_vec.get(4).unwrap(),);

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

pub fn show_vm(name: &str) -> FullVm {
    let cmd_result = Command::new("multipass")
        .args(["info", name])
        .output()
        .expect("msg");

    let mut vm = FullVm::default();
    let mut vm_details: Vec<(String, String)> = vec![];

    for line in cmd_result.stdout.lines() {
        let val: String = line.unwrap();

        let split = val.split(" ");
        let mut split_vec: Vec<&str> = split.collect();
        split_vec.retain(|&x| x != "");

        match split_vec[0] {
            "Name:" | "State:" | "Snapshots:" | "IPv4:" | "Release:" | "CPU(s):" | "Mounts:" => {
                vm_details.push((split_vec[0].to_string(), split_vec[1].to_string()))
            }
            "Disk" | "Memory" => vm_details.push((
                format!("{} {}", split_vec[0].to_string(), split_vec[1].to_string()),
                format!(
                    "{} {} {} {}",
                    split_vec[2].to_string(),
                    split_vec[3].to_string(),
                    split_vec[4].to_string(),
                    split_vec[5].to_string()
                ),
            )),
            "Image" => vm_details.push((
                format!("{} {}", split_vec[0].to_string(), split_vec[1].to_string()),
                format!(
                    "{} {} {}",
                    split_vec[2].to_string(),
                    split_vec[3].to_string(),
                    split_vec[4].to_string(),
                ),
            )),
            "Load:" => vm_details.push((
                split_vec[0].to_string(),
                format!(
                    "{} {} {}",
                    split_vec[1].to_string(),
                    split_vec[2].to_string(),
                    split_vec[3].to_string(),
                ),
            )),

            _ => println!("something else!"),
        }

        // if split_vec.get(3).is_some() {
        //     vm_details.push((
        //         format!("{} {}", split_vec[0].to_string(), split_vec[1].to_string()),
        //         split_vec[2].to_string(),
        //     ));
        //     continue;
        // }

        // vm_details.push((split_vec[0].to_string(), split_vec[1].to_string()))
    }

    for (key, value) in vm_details {
        match key.as_str() {
            "Name:" => vm.name = value,
            "State:" => vm.state = value,
            "Snapshots:" => vm.snapshots = value,
            "IPv4:" => vm.ipv4 = value,
            "Release:" => vm.release = value,
            "Image hash:" => vm.image_hash = value,
            "CPU(s):" => vm.cpus = value,
            "Load:" => vm.load = value,
            "Disk usage:" => vm.disk_usage = value,
            "Memory usage:" => vm.memory_usage = value,
            "Mounts:" => vm.mounts = value,
            _ => println!("something else!"),
        }
    }
    
    println!("{:?}", vm);

    return vm;
}
