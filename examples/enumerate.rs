use illumos_audio::Mixer;

pub fn main() -> std::io::Result<()> {
    let mixer = Mixer::open()?;

    let (maj, min) = mixer.version();
    println!("OSS version = {maj}.{min}");

    let si = mixer.sysinfo()?;
    println!("sysinfo = {si:#?}");

    println!("AUDIO INFO:");
    for i in 0..si.num_audios {
        let info = mixer.audioinfo(i)?;
        let out = format!("audioinfo[{i}] = {info:#?}").lines()
            .map(|l| format!("    {l}\n")).collect::<String>();
        println!("{out}");
    }

    println!("CARD INFO:");
    for i in 0..si.num_cards {
        let info = mixer.cardinfo(i)?;
        let out = format!("cardinfo[{i}] = {info:#?}").lines()
            .map(|l| format!("    {l}\n")).collect::<String>();
        println!("{out}");
    }

    println!("MIXER INFO:");
    for i in 0..si.num_mixers {
        let info = mixer.mixerinfo(i)?;
        let out = format!("mixerinfo[{i}] = {info:#?}").lines()
            .map(|l| format!("    {l}\n")).collect::<String>();
        println!("{out}");
    }

    Ok(())
}
