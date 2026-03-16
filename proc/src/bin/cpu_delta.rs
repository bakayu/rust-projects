use procfs::{CpuTime, CurrentSI, KernelStats};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // first sample
    let stats1 = KernelStats::current().unwrap();
    let cpu1 = &stats1.total;

    sleep(Duration::from_secs(1));

    // second sample
    let stats2 = KernelStats::current().unwrap();
    let cpu2 = &stats2.total;

    let usage_percent = cpu_percent(cpu1, cpu2);
    println!("CPU usage: {:.1}%", usage_percent);
}

fn cpu_percent(a: &CpuTime, b: &CpuTime) -> f64 {
    let total_a = total_duration(a);
    let total_b = total_duration(b);
    let total_delta = total_b - total_a;

    if total_delta == 0.0 {
        return 0.0;
    }

    let idle_delta = (b.idle_duration() - a.idle_duration()).as_secs_f64();

    // used = everything except idle and iowait
    (total_delta - idle_delta) / total_delta * 100.0
}

fn total_duration(c: &CpuTime) -> f64 {
    (c.user_duration()
        + c.nice_duration()
        + c.system_duration()
        + c.idle_duration()
        + c.iowait_duration().unwrap_or_default()
        + c.irq_duration().unwrap_or_default()
        + c.softirq_duration().unwrap_or_default()
        + c.steal_duration().unwrap_or_default()
        + c.guest_duration().unwrap_or_default()
        + c.guest_nice_duration().unwrap_or_default())
    .as_secs_f64()
}
