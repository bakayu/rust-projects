use procfs::{Current, LoadAverage};

fn main() -> anyhow::Result<()> {
    let loadavg = LoadAverage::current()?;

    let one = loadavg.one;
    let five = loadavg.five;
    let fifteen = loadavg.fifteen;

    println!("Loadavg -> one: {one:>.3} five: {five:>.3} fifteen: {fifteen:>.3}");

    Ok(())
}
