use procfs::{Current, Meminfo};

fn main() {
    let meminfo = Meminfo::current().unwrap();

    let total = meminfo.mem_total;
    let available = meminfo.mem_available.unwrap_or_default();
    let free = meminfo.mem_free;
    let used = total - available;

    let swap_total = meminfo.swap_total;
    let swap_free = meminfo.swap_free;
    let swap_cached = meminfo.swap_cached;

    let buffer = meminfo.buffers;

    let cache = meminfo.cached;

    let zswap = meminfo.z_swap.unwrap_or_default();
    let zswapped = meminfo.z_swapped.unwrap_or_default();

    display_size("total memory", total);
    display_size("used memory", used);
    display_size("available memory", available);
    display_size("free memory", free);

    display_size("total swap", swap_total);
    display_size("free swap", swap_free);
    display_size("cached swap", swap_cached);

    display_size("buffers", buffer);

    display_size("cache", cache);

    display_size("zswap", zswap);
    display_size("zswapped", zswapped);
}

fn display_size(mem_type: &str, bytes: u64) {
    let kibibytes = bytes / 1024;
    let memibytes = kibibytes / 1024;
    let gebibytes = memibytes / 1024;

    println!("{}:", mem_type);
    println!("\tsize in bytes \t: {}", bytes);
    println!("\tsize in KiB \t: {}", kibibytes);
    println!("\tsize in MiB \t: {}", memibytes);
    println!("\tsize in GiB \t: {}", gebibytes);
}
