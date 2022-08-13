use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::process::{Command, ExitCode};

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
    add_entry(file, OffsetDateTime::now_local()?, Duration::ZERO)?;
    notify("Added start timestamp")
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
    add_entry(file, now, duration)?;
    notify("Added timestamp")
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
    println!(
        r#"Usage: timestampr [cmd]

Writes a tab-separated timestamp and duration to ~/Documents/timestamps.tsv
A notification is shown when an entry is added successfully via `notify-send`.

COMMANDS:

start      Add a new entry with the duration set to 00:00:00

[default]  If no command is supplied the default behaviour is to append a
           new timestamp to the file with the duration since the start
           entry. I.e. the last entry with duration 00:00:00

FILES:

        ~/Documents/timestamps.tsv      Where the records are written
"#
    )
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

fn notify(message: &str) -> Result<(), Box<dyn Error>> {
    let _status = Command::new("notify-send")
        .args(&[
            "-i",
            "/usr/share/icons/Adwaita/64x64/legacy/preferences-system-time-symbolic.symbolic.png",
            "Timestampr",
            message,
        ])
        .status()?;
    Ok(())
}
