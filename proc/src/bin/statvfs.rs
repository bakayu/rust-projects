use std::{collections::BTreeSet, error::Error, fs::File, path::Path};

use procfs::process::Process;
use rustix::fs::fstatvfs;

fn human(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;

    if bytes >= GIB {
        format!("{:.2} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.2} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.2} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mounts = Process::myself()?.mountinfo()?;

    // stable ordering for output
    let mut mount_points = BTreeSet::new();

    for m in mounts {
        if let Some(src) = &m.mount_source {
            if src.starts_with("/dev/") {
                mount_points.insert((m.mount_point.clone(), src.clone(), m.fs_type.clone()));
            }
        }
    }

    for (mount_point, source, fs_type) in mount_points {
        let fd = match File::open(Path::new(&mount_point)) {
            Ok(fd) => fd,
            Err(err) => {
                eprintln!("skipping {} ({}): {}", mount_point.display(), source, err);
                continue;
            }
        };

        let st = fstatvfs(&fd)?;

        let block_size = st.f_bsize as u64;
        let total = (st.f_blocks as u64).saturating_mul(block_size);
        let free = (st.f_bfree as u64).saturating_mul(block_size);
        let avail = (st.f_bavail as u64).saturating_mul(block_size);
        let used = total.saturating_sub(free);

        println!("{source} mounted on {mount_point:?} ({fs_type})");
        println!("  total: {} ({})", total, human(total));
        println!("  used:  {} ({})", used, human(used));
        println!("  free:  {} ({})", free, human(free));
        println!("  avail: {} ({})", avail, human(avail));
        println!();
    }

    Ok(())
}
