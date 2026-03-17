use std::{
    collections::HashMap,
    thread::sleep,
    time::{Duration, Instant},
};

use procfs::diskstats;

fn main() -> anyhow::Result<()> {
    // sample 1
    let start = Instant::now();
    let disk_stats_1 = diskstats()?;
    sleep(Duration::from_secs(3));
    let elapsed = start.elapsed().as_secs_f64();

    // sample 2
    let disk_stats_2 = diskstats()?;

    let map1: HashMap<_, _> = disk_stats_1
        .into_iter()
        .filter(|d| is_physical_disk(&d.name))
        .map(|d| (d.name.clone(), d))
        .collect();

    let map2: HashMap<_, _> = disk_stats_2
        .into_iter()
        .filter(|d| is_physical_disk(&d.name))
        .map(|d| (d.name.clone(), d))
        .collect();

    for name in map2.keys() {
        println!("kept disk: {}", name);
    }

    for (name, cur) in &map2 {
        if let Some(prev) = map1.get(name) {
            let delta_reads = cur.reads.saturating_sub(prev.reads);
            let delta_writes = cur.writes.saturating_sub(prev.writes);
            let delta_sector_read = cur.sectors_read.saturating_sub(prev.sectors_read);
            let delta_sector_write = cur.sectors_written.saturating_sub(prev.sectors_written);

            let read_bytes = delta_sector_read as f64 * 512.0;
            let write_bytes = delta_sector_write as f64 * 512.0;

            println!(
                "{name:>8} read_iops={:>.1}  write_iops={:>.1}  read_B/s={:>.1}  write_B/s={:>.1}",
                delta_reads as f64 / elapsed,
                delta_writes as f64 / elapsed,
                read_bytes / elapsed,
                write_bytes / elapsed
            );
        }
    }

    Ok(())
}

fn is_physical_disk(name: &str) -> bool {
    // nvmeXnY (checks for trailing "pN")
    if name.starts_with("nvme") {
        return name
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_digit())
            .count()
            > 0
            && !name.contains('p');
    }

    // sdX, hdX, vdX, xvdX (checks for no trailing digit)
    if let Some(prefix) = ["sd", "hd", "vd", "xvd"]
        .iter()
        .find(|p| name.starts_with(*p))
    {
        return name[prefix.len()..].chars().all(|c| c.is_ascii_lowercase());
    }

    false
}
