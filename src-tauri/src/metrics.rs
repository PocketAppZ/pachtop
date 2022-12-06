use crate::models::{Cpu, Disk, GlobalCpu, Memory, Network, Process, Swap, SysInfo, Timestamp};
use std::str::{self, FromStr};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{CpuExt, DiskExt, NetworkExt, Pid, ProcessExt, Signal, System, SystemExt};

use tauri::State;

pub struct MetricsState(Arc<Mutex<Metrics>>);

impl MetricsState {
    pub fn new(sys: System) -> Self {
        MetricsState(Arc::new(Mutex::new(Metrics { sys })))
    }
}

struct Metrics {
    sys: System,
}

impl Metrics {
    fn sysinfo(&mut self) -> SysInfo {
        self.sys.refresh_all();

        let kernel_version = self.sys.kernel_version().unwrap_or("Unknown".to_string());
        let os_version = self.sys.long_os_version().unwrap_or("Unknown".to_string());
        let hostname = self.sys.host_name().unwrap_or("Unknown".to_string());
        let core_count = self.sys.physical_core_count().unwrap_or(0).to_string();

        SysInfo {
            kernel_version,
            os_version,
            hostname,
            core_count,
            timestamp: current_time(),
        }
    }

    fn global_cpu(&mut self) -> GlobalCpu {
        self.sys.refresh_cpu();

        let cpu = self.sys.global_cpu_info();
        let usage = cpu.cpu_usage();
        let brand = cpu.brand().to_owned();
        let frequency = cpu.frequency().to_owned();
        let name = cpu.name().to_owned();
        let vendor = cpu.vendor_id().to_owned();

        GlobalCpu {
            usage,
            brand,
            frequency,
            name,
            vendor,
            timestamp: current_time(),
        }
    }

    fn cpus(&mut self) -> Vec<Cpu> {
        let cpus: Vec<Cpu> = self
            .sys
            .cpus()
            .into_iter()
            .map(|cpu| {
                let name = cpu.name().to_owned();
                let usage = cpu.cpu_usage().to_owned() as u64;
                let total = 100;

                Cpu {
                    name,
                    usage: get_percentage(&usage, &total),
                    timestamp: current_time(),
                }
            })
            .collect();

        cpus
    }

    fn disks(&mut self) -> Vec<Disk> {
        self.sys.refresh_disks_list();
        self.sys.refresh_disks();

        let disks: Vec<Disk> = self
            .sys
            .disks()
            .into_iter()
            .map(|disk| {
                let name = match disk.name().to_str() {
                    Some("") => disk.mount_point().to_str().unwrap_or("Unknown").to_owned(),
                    Some(name) => name.to_owned(),
                    None => "Unknown".to_owned(),
                };

                let disk_type = match disk.type_() {
                    sysinfo::DiskType::HDD => "HDD".to_owned(),
                    sysinfo::DiskType::SSD => "SSD".to_owned(),
                    _ => "Unknown".to_owned(),
                };
                let file_system = match str::from_utf8(disk.file_system()) {
                    Ok(v) => v.to_owned(),
                    Err(e) => {
                        println!("Invalid UTF-8 sequence: {}", e);
                        "Unknown".to_owned()
                    }
                };

                let total = disk.total_space();
                let free = disk.available_space();
                let used = total - free;
                let is_removable = disk.is_removable();
                let mount_point = disk.mount_point().to_owned();

                Disk {
                    name,
                    free,
                    used,
                    total,
                    mount_point,
                    file_system,
                    is_removable,
                    disk_type,
                    timestamp: current_time(),
                }
            })
            .collect();

        disks
    }

    fn memory(&mut self) -> Memory {
        self.sys.refresh_memory();

        Memory {
            free: self.sys.free_memory(),
            total: self.sys.total_memory(),
            used: self.sys.used_memory(),
            used_percentage: get_percentage(&self.sys.used_memory(), &self.sys.total_memory()),
            timestamp: current_time(),
        }
    }

    fn swap(&mut self) -> Swap {
        self.sys.refresh_memory();

        Swap {
            free: self.sys.free_swap(),
            total: self.sys.total_swap(),
            used: self.sys.used_swap(),
            used_percentage: get_percentage(&self.sys.used_swap(), &self.sys.total_swap()),
            timestamp: current_time(),
        }
    }

    fn processes(&mut self) -> Vec<Process> {
        self.sys.refresh_processes();
        let cpu_count = self.sys.physical_core_count().unwrap_or(1);

        let processes: Vec<Process> = self
            .sys
            .processes()
            .into_iter()
            .map(|(pid, process)| {
                let name = process.name().to_owned();
                let cpu_usage = round(process.cpu_usage() / cpu_count as f32);
                let pid = pid.to_string();
                let memory_usage = process.memory();

                let status = match process.status() {
                    sysinfo::ProcessStatus::Run => "Running".to_owned(),
                    sysinfo::ProcessStatus::Sleep => "Sleeping".to_owned(),
                    sysinfo::ProcessStatus::Stop => "Stopped".to_owned(),
                    sysinfo::ProcessStatus::Idle => "Idle".to_owned(),
                    sysinfo::ProcessStatus::Zombie => "Zombie".to_owned(),
                    _ => "Unknown".to_owned(),
                };

                Process {
                    name,
                    pid,
                    cpu_usage,
                    memory_usage,
                    status,
                }
            })
            .collect();

        // TODO: modify processes so it can deliver the grouped processes
        // let mut grouped_processes: Vec<Process> = Vec::new();
        // for process in processes {
        //     let mut found = false;
        //     for grouped_process in &mut grouped_processes {
        //         if grouped_process.name == process.name {
        //             grouped_process.cpu_usage += process.cpu_usage;
        //             grouped_process.memory_usage += process.memory_usage;
        //             found = true;
        //             break;
        //         }
        //     }

        //     if !found {
        //         grouped_processes.push(process);
        //     }
        // }

        // grouped_processes

        processes
    }

    fn networks(&mut self) -> Vec<Network> {
        self.sys.refresh_networks();

        let networks: Vec<Network> = self
            .sys
            .networks()
            .into_iter()
            .map(|(name, network)| {
                let name = name.to_owned();

                Network {
                    name,
                    received: network.received(),
                    transmitted: network.transmitted(),
                    timestamp: current_time(),
                }
            })
            .collect();

        networks
    }

    fn kill_process(&mut self, pid: String) -> bool {
        let pid = match Pid::from_str(&pid) {
            Ok(pid) => pid,
            Err(_) => return false,
        };

        let process = match self.sys.process(pid) {
            Some(process) => process,
            None => return false,
        };

        let result = match process.kill_with(Signal::Kill) {
            Some(result) => result,
            None => return false,
        };

        result
    }
}

fn current_time() -> Timestamp {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH);
    Timestamp(since_the_epoch.unwrap().as_millis() as i64)
}

fn get_percentage(value: &u64, total: &u64) -> f64 {
    let percentage = (*value as f64 / *total as f64) * 100.0;
    (percentage * 100.0).round() / 100.0
}
fn round(x: f32) -> f32 {
    (x * 100.0).round() / 100.0
}

#[tauri::command]
pub fn get_sysinfo(state: State<'_, MetricsState>) -> SysInfo {
    state.0.lock().unwrap().sysinfo()
}

#[tauri::command]
pub fn get_global_cpu(state: State<'_, MetricsState>) -> GlobalCpu {
    state.0.lock().unwrap().global_cpu()
}

#[tauri::command]
pub fn get_cpus(state: State<'_, MetricsState>) -> Vec<Cpu> {
    state.0.lock().unwrap().cpus()
}

#[tauri::command]
pub fn get_memory(state: State<'_, MetricsState>) -> Memory {
    state.0.lock().unwrap().memory()
}

#[tauri::command]
pub fn get_swap(state: State<'_, MetricsState>) -> Swap {
    state.0.lock().unwrap().swap()
}

#[tauri::command]
pub fn get_networks(state: State<'_, MetricsState>) -> Vec<Network> {
    state.0.lock().unwrap().networks()
}
#[tauri::command]
pub fn get_disks(state: State<'_, MetricsState>) -> Vec<Disk> {
    state.0.lock().unwrap().disks()
}

#[tauri::command]
pub fn get_processes(state: State<'_, MetricsState>) -> Vec<Process> {
    state.0.lock().unwrap().processes()
}

#[tauri::command]
pub fn kill_process(state: State<'_, MetricsState>, pid: String) -> bool {
    state.0.lock().unwrap().kill_process(pid)
}
