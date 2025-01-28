use std::{
    env,
    fmt::Display,
    fs::{self, FileType, Permissions},
    io::{self, BufRead, IsTerminal, Write},
    os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt},
    time::SystemTime,
};

use colored::Colorize;
use humansize::{format_size, DECIMAL};
use time::{
    format_description::{self},
    OffsetDateTime, UtcOffset,
};

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
    file_type: FileType,
    permissions: Permissions,
    last_modified: SystemTime,
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let mode = self.permissions.mode();
        let readonly = if self.permissions.readonly() {
            "READONLY".red().bold()
        } else {
            "-".white()
        };
        let permissions = format_permissions(mode);
        let size = format_size(self.size, DECIMAL);
        let dt: OffsetDateTime = self.last_modified.into();
        let local_dt =
            dt.to_offset(UtcOffset::current_local_offset().map_err(|_| std::fmt::Error)?);
        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]")
            .map_err(|_| std::fmt::Error)?;
        let accessed = local_dt
            .format(&format)
            .unwrap_or("[no date]".red().to_string());

        let file_type = match self.file_type {
            t if t.is_dir() => "directory".blue(),
            t if t.is_symlink() => "symlink".cyan(),
            t if t.is_socket() => "socket".white(),
            t if t.is_block_device() => "disk".normal(),
            t if t.is_file() => "file".yellow(),
            _ => "unknown".on_black(),
        };

        write!(
            f,
            "{} {:o} ({}) {} \"{}\" {} {}",
            file_type,
            mode,
            permissions,
            size.bold().blue(),
            accessed,
            readonly,
            name.bold(),
        )
    }
}

fn main() -> Result<(), io::Error> {
    let mut stdout = std::io::stdout().lock();

    let files: Vec<String> = if !io::stdin().is_terminal() {
        io::stdin().lock().lines().map(|f| f.unwrap()).collect()
    } else {
        env::args().skip(1).collect()
    };

    let _: Vec<File> = files
        .iter()
        .map(|f| -> Result<File, io::Error> {
            let meta = fs::metadata(f)?;

            let file = File {
                name: f.to_string(),
                size: meta.size(),
                file_type: meta.file_type(),
                permissions: meta.permissions(),
                last_modified: meta.modified()?,
            };
            writeln!(stdout, "{file}")?;
            Ok(file)
        })
        .collect::<Result<_, _>>()?;

    Ok(())
}

const CHARS: [char; 2] = ['-', 'r'];

fn format_permissions(mode: u32) -> String {
    let mut result = String::with_capacity(9);

    for i in (0..9).rev() {
        result.push(CHARS[((mode >> i) & 1) as usize]);
    }

    // set the executable mark
    if mode & 0o200 != 0 {
        result.replace_range(1..2, "w");
    }
    if mode & 0o100 != 0 {
        result.replace_range(2..3, "x");
    }
    if mode & 0o020 != 0 {
        result.replace_range(4..5, "w");
    }
    if mode & 0o010 != 0 {
        result.replace_range(5..6, "x");
    }
    if mode & 0o002 != 0 {
        result.replace_range(7..8, "w");
    }
    if mode & 0o001 != 0 {
        result.replace_range(8..9, "x");
    }

    result
}
