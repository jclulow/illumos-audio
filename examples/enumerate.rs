use illumos_audio::MixerInfo;

pub fn main() -> std::io::Result<()> {
    let mi = MixerInfo::open()?;

    let (maj, min) = mi.version();
    println!("OSS version = {maj}.{min}");

    let si = mi.sysinfo()?;
    println!("sysinfo = {si:#?}");

    println!("AUDIO INFO:");
    for i in 0..si.num_audios {
        let info = mi.audioinfo(i)?;
        let out = format!("audioinfo[{i}] = {info:#?}").lines()
            .map(|l| format!("    {l}\n")).collect::<String>();
        println!("{out}");
    }

    println!("CARD INFO:");
    for i in 0..si.num_cards {
        let info = mi.cardinfo(i)?;
        let out = format!("cardinfo[{i}] = {info:#?}").lines()
            .map(|l| format!("    {l}\n")).collect::<String>();
        println!("{out}");
    }

    Ok(())
}
