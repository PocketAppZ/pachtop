use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
pub struct Timestamp(pub i64);

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/lib/bindings/")]
pub struct Memory {
    #[ts(type = "number")]
    pub free: u64,
    #[ts(type = "number")]
    pub total: u64,
    #[ts(type = "number")]
    pub used: u64,
    #[ts(type = "number")]
    pub used_percentage: f64,
    pub timestamp: Timestamp,
}

pub trait MemoryTrait {
    fn get_memory(&mut self) -> Memory;
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/lib/bindings/")]
pub struct GlobalCpu {
    pub usage: f32,
    pub brand: String,
    pub frequency: u64,
    pub name: String,
    pub vendor: String,
    pub timestamp: Timestamp,
}

pub trait GlobalCpuTrait {
    fn get_global_cpu(&mut self) -> GlobalCpu;
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/lib/bindings/")]
pub struct Cpu {
    pub name: String,
    pub usage: f64,
    pub timestamp: Timestamp,
}

pub trait CpuTrait {
    fn get_cpus(&mut self) -> Vec<Cpu>;
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/lib/bindings/")]
pub struct Swap {
    #[ts(type = "number")]
    pub free: u64,
    #[ts(type = "number")]
    pub total: u64,
    #[ts(type = "number")]
    pub used: u64,
    #[ts(type = "number")]
    pub used_percentage: f64,
    pub timestamp: Timestamp,
}

pub trait SwapTrait {
    fn get_swap(&mut self) -> Swap;
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/lib/bindings/")]
pub struct SysInfo {
    pub kernel_version: String,
    pub os_version: String,
    pub hostname: String,
    pub core_count: String,
    pub timestamp: Timestamp,
}

pub trait SystemInformationTrait {
    fn get_system_information(&mut self) -> SysInfo;
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/lib/bindings/")]
pub struct Network {
    pub name: String,
    #[ts(type = "number")]
    pub received: u64,
    #[ts(type = "number")]
    pub transmitted: u64,
    pub timestamp: Timestamp,
}

pub trait NetworkTrait {
    fn get_networks(&mut self) -> Vec<Network>;
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/lib/bindings/")]
pub struct Disk {
    pub name: String,
    #[ts(type = "number")]
    pub free: u64,
    #[ts(type = "number")]
    pub total: u64,
    #[ts(type = "number")]
    pub used: u64,
    #[ts(type = "number")]
    pub used_percentage: f64,
    pub mount_point: PathBuf,
    pub file_system: String,
    pub disk_type: String,
    pub is_removable: bool,
    pub timestamp: Timestamp,
}

pub trait DisksTrait {
    fn get_disks(&mut self) -> Vec<Disk>;
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/lib/bindings/")]
pub struct Process {
    pub name: String,
    pub pid: String,
    #[ts(type = "number")]
    pub cpu_usage: f32,
    #[ts(type = "number")]
    pub memory_usage: u64,
    pub status: String,
}

pub trait ProcessesTrait {
    fn get_processes(&mut self) -> Vec<Process>;
    fn kill_process(&mut self, pid: &str) -> bool;
}
