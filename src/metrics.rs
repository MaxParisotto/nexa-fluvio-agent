use sysinfo::System;

pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
}

pub fn collect_metrics() -> SystemMetrics {
    let sys = System::new_all();
    
    // Get global CPU usage as a percentage
    let cpu_usage = sys.cpus()[0].cpu_usage();
    
    // Get memory information
    let memory_used = sys.used_memory();
    let memory_total = sys.total_memory();

    SystemMetrics {
        cpu_usage,
        memory_used,
        memory_total,
    }
}
