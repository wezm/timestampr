use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::process::ExitCode;

use time::format_description::well_known::Rfc2822;
use time::{Duration, OffsetDateTime};

fn main() -> ExitCode {
    let cmd = env::args().nth(1);
    let res = match cmd.as_deref() {
        Some("start") => start(),
        Some(name) => {
            eprintln!("Unknown command: {}", name);
            usage();
            return ExitCode::from(2);
        }
        None => timestamp(),
    };

    match res {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {}", err);
            ExitCode::FAILURE
        }
    }
}

fn start() -> Result<(), Box<dyn Error>> {
    let mut file = open_file()?;
    file.seek(SeekFrom::End(0))?;
    add_entry(file, OffsetDateTime::now_local()?, Duration::ZERO)
}

fn timestamp() -> Result<(), Box<dyn Error>> {
    let now = OffsetDateTime::now_local()?;
    let mut duration = Duration::ZERO;

    let mut file = open_file()?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    for line in buf.lines().rev() {
        match line.split_once('\t') {
            Some((timestamp, line_duration)) if line_duration == "00:00:00" => {
                let timestamp = OffsetDateTime::parse(timestamp, &Rfc2822)?;
                duration = now - timestamp;
                break;
            }
            _ => {}
        }
    }

    file.seek(SeekFrom::End(0))?;
    add_entry(file, now, duration)
}

fn add_entry(
    mut file: File,
    timestamp: OffsetDateTime,
    duration: Duration,
) -> Result<(), Box<dyn Error>> {
    writeln!(
        &mut file,
        "{}\t{:02}:{:02}:{:02}",
        timestamp.format(&Rfc2822)?,
        duration.whole_hours(),
        duration.whole_minutes(),
        duration.whole_seconds()
    )?;
    Ok(())
}

fn usage() {
    todo!()
}

fn open_file() -> Result<File, Box<dyn Error>> {
    let mut path = home::home_dir().ok_or("unable to determine home directory")?;
    path.push("Documents");
    if !path.is_dir() {
        return Err("Documents directory does not exist".into());
    }
    path.push("timestamps.tsv");

    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)?;
    Ok(file)
}
