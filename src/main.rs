use atty::{isnt, Stream};
use clap::{command, Arg, ArgMatches};
use std::fmt::Result;
use std::fs::{self, File};
use std::io;
use std::io::LineWriter;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

fn main() -> Result {
    let matches: ArgMatches = command!()
        .author("Maurice-Jörg Nießen <info@mjniessen.com>")
        .arg(
            Arg::new("APPEND")
                .long("append")
                .alias("add")
                .short('a')
                .action(clap::ArgAction::SetTrue)
                .help("Append to stored content"),
        )
        .arg(
            Arg::new("CLEAR")
                .long("clear")
                .short('c')
                .alias("clean")
                .alias("remove")
                .short_alias('c')
                .action(clap::ArgAction::SetTrue)
                .help("Clear stored content"),
        )
        .arg(
            Arg::new("QUIET")
                .long("quiet")
                .short('q')
                .action(clap::ArgAction::SetTrue)
                .help("Suppress any output"),
        )
        .get_matches();

    let file = place();

    if isnt(Stream::Stdin) {
        let _ = pipe_in(&file);
    }

    if !matches.get_flag("QUIET") || isnt(Stream::Stdout) {
        let _ = pipe_out(&file);
    }

    if matches.get_flag("CLEAR") {
        let _ = pipe_del(&file);
    }

    // eprintln!("stdout? {}", is(Stream::Stdout));
    // eprintln!("stderr? {}", is(Stream::Stderr));
    // eprintln!("stdin? {}", is(Stream::Stdin));

    Ok(())
}

fn place() -> PathBuf {
    // start with HOME
    let mut path = match home::home_dir() {
        Some(path) => path,
        None => PathBuf::new(),
    };

    // check for .cache
    // if !std::path::Path::new(&path).exists() {
    // }

    path.push(".cache");
    path.push("ans");

    if !std::path::Path::new(&path).exists() {
        fs::create_dir_all(&path).unwrap();
    }

    path.push("temp");
    path
}

fn pipe_in(file: &PathBuf) -> io::Result<()> {
    let f = File::create(file)?;
    let mut writer = LineWriter::new(f);

    let reader = io::stdin().lock();
    for line in reader.lines() {
        writer.write_all(line?.as_bytes())?;
        writer.write_all(b"\n")?;
    }

    writer.flush()?;
    Ok(())
}

fn pipe_out(file: &PathBuf) -> io::Result<()> {
    let f = File::open(file)?;
    let reader = BufReader::new(f);
    let mut stdout = io::stdout().lock();

    for line in reader.lines() {
        writeln!(stdout, "{}", line?)?;
    }

    Ok(())
}

fn pipe_del(file: &PathBuf) -> io::Result<()> {
    fs::remove_file(file)?;

    Ok(())
}
