use illumos_audio::MixerInfo;

pub fn main() -> std::io::Result<()> {
    let mi = MixerInfo::open()?;

    let (maj, min) = mi.version();
    println!("OSS version = {maj}.{min}");

    let si = mi.sysinfo()?;
    println!("sysinfo = {si:#?}");

    for i in 0..si.num_audios {
        let ai = mi.audioinfo(i)?;
        let out = format!("audioinfo[{i}] = {ai:#?}").lines()
            .map(|l| format!("    {l}\n")).collect::<String>();
        println!("{out}");
    }

    Ok(())
}
