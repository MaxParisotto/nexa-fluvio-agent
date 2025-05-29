use sysinfo::System;

pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub swap_used: u64,
    pub swap_total: u64,
    pub load_avg_1min: f64,
    pub load_avg_5min: f64,
    pub load_avg_15min: f64,
    pub uptime: u64,
    pub processes_count: usize,
}

pub fn collect_metrics() -> SystemMetrics {
    let sys = System::new_all();
    let load_avg = System::load_average();
    
    SystemMetrics {
        cpu_usage: sys.cpus()[0].cpu_usage(),
        memory_used: sys.used_memory(),
        memory_total: sys.total_memory(),
        swap_used: sys.used_swap(),
        swap_total: sys.total_swap(),
        load_avg_1min: load_avg.one,
        load_avg_5min: load_avg.five,
        load_avg_15min: load_avg.fifteen,
        uptime: System::uptime(),
        processes_count: sys.processes().len(),
    }
}
