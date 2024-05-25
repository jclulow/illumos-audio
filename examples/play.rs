use illumos_audio::{Dsp, Mixer};

pub fn main() -> std::io::Result<()> {
    let mixer = Mixer::open()?;

    let (maj, min) = mixer.version();
    println!("OSS version = {maj}.{min}");

    let si = mixer.sysinfo()?;
    println!("sysinfo = {si:#?}");

    println!("AUDIO INFO:");
    for i in 0..si.num_audios {
        let info = mixer.audioinfo(i)?;
        let out = format!("audioinfo[{i}] = {info:#?}")
            .lines()
            .map(|l| format!("    {l}\n"))
            .collect::<String>();
        println!("{out}");

        let mut dsp = Dsp::open_path(&info.devnode)?;

        println!("    syncing...");
        dsp.sync()?;
        println!("    ok!");
        println!();

        let errs = dsp.errors()?;
        let out = format!("errors? = {errs:#?}")
            .lines()
            .map(|l| format!("    {l}\n"))
            .collect::<String>();
        println!("{out}");
        println!();

        println!("    channel count = {}", dsp.channels()?);
        println!();

        println!("    setting to mono...");
        dsp.channels_set(1)?;
        println!();

        println!("    channel count = {}", dsp.channels()?);
        println!();

        println!("    play volume = {}", dsp.volume_play()?);
        println!();

        let outspace = dsp.space_output()?;
        let out = format!("outspace? = {outspace:#?}")
            .lines()
            .map(|l| format!("    {l}\n"))
            .collect::<String>();
        println!("{out}");
        println!();

        let speed = dsp.speed()?;
        let formats = dsp.formats()?;
        let format = dsp.format()?;
        println!("    speed = {speed}");
        println!("    formats = {formats:?}");
        println!("    format = {format:?}");
        println!();

        println!("    setting format to 24-bit little endian...");
        dsp.format_set(illumos_audio::sys::AudioFormats::AFMT_S24_LE)?;
        println!();

        /*
         * Generate a tone, I guess?
         */
        let samps_per = speed / 261;
        let mut on = true;
        let mut nsamps = 0;

        /*
         * Audio buffer with enough space for 10ms of audio...
         */
        let mut buf = Vec::with_capacity((speed * 4 / 100).try_into().unwrap());
        for _ in 0..100 {
            buf.clear();
            for i in 0..(speed / 100) {
                nsamps += 1;
                if nsamps > samps_per {
                    nsamps = 0;
                    on = !on;
                }

                let v =
                    if on { 1000i32 } else { -1000i32 }.to_le_bytes();

                buf.push(0);
                buf.push(v[0]);
                buf.push(v[1]);
                buf.push(v[2]);
            }

            dsp.play(&buf)?;
        }

        println!("    draining...");
        dsp.sync()?;
        println!("    ok!");
    }

    Ok(())
}
