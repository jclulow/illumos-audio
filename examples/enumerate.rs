use illumos_audio::MixerInfo;

pub fn main() -> std::io::Result<()> {
    let mi = MixerInfo::open()?;

    let (maj, min) = mi.version();
    println!("OSS version = {maj}.{min}");

    let si = mi.sysinfo()?;
    println!("sysinfo = {si:#?}");

    Ok(())
}
