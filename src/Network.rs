extern crate sysinfo;

use std::io::BufRead;
use std::sync::*;
use std::thread;

use tracing::{error, info, warn};
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

struct Network{ 
    
}

pub struct MonitorApp {
    include : Vec<f64>,
  //  measurements : Arc<Mutex<MeasurementWindow>>,
}

/* 
trait SysInfo{
    fn Get_Cpu_Use() -> f32{
    //    (req_sys.get_used_memory() as f32) / (req_sys.get_total_memory() as f32) * 100.
    }
}

pub mod Wireless {

    

 //   fn get_ram_use(req_sys: &sysinfo::System) -> f32
 //   {
 //       (req_sys.get_used_memory() as f32) / (req_sys.get_total_memory() as f32) * 100.
 //   }
//
 //   pub fn get_cpu_use(req_sys: &sysinfo::System) -> f32
 //   {
 //       // Put all of the core loads into a vector
 //       let mut cpus: Vec<f32> = Vec::new();
 //       for core in req_sys.get_processors() { cpus.push(core.get_cpu_usage()); }
 //   
 //       // Get the average load
 //       let cpu_tot: f32 = cpus.iter().sum();
 //       let cpu_avg: f32 = cpu_tot / cpus.len() as f32;
 //   
 //       cpu_avg
 //   }

    pub fn MaxSpeed(){
         
        
    }
}*/