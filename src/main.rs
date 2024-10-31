pub mod parser;

use crate::parser::{_42Header, _42header};
use chrono::{DateTime, FixedOffset};
use clap::Parser;
use std::path::Path;
use std::{fs, io, time::UNIX_EPOCH};

#[derive(Debug, Parser)]
struct Args {
    #[arg(long)]
    name: String,

    #[arg(long)]
    email: String,

    #[arg(long)]
    path: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let content = fs::read_to_string(&args.path)?;
    match _42header(&content) {
        Ok((c, header)) => {
            fs::write(&args.path, _42_replace(header, args.name, Path::new(&args.path), args.path
                .split('/')
                .last()
                .expect("file name cannot be retrieved")) + c)?;
            Ok(())
        }
        Err(e) => match e {
            nom::Err::Incomplete(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Incomplete")),
            nom::Err::Error(nom::error::Error { input: c, .. }) | nom::Err::Failure(nom::error::Error { input: c, .. })=> {
                let header = _42(
                    args.name,
                    args.email,
                    args.path
                        .split('/')
                        .last()
                        .expect("file name cannot be retrieved"),
                    &args.path,
                );
                fs::write(&args.path, header + c)?;
                Ok(())
            }
        }
    }
}

fn _42(name: String, email: String, file: &str, path: &str) -> String {
    let mut res = String::new();
    res.push_str(
        "/* ************************************************************************** */\n",
    );
    res.push_str(
        "/*                                                                            */\n",
    );
    res.push_str(
        "/*                                                        :::      ::::::::   */\n",
    );
    res.push_str("/*   ");
    res.push_str(&file);
    assert!(51 >= file.len());
    res.push_str(" ".repeat(51 - file.len()).as_str());
    res.push_str(":+:      :+:    :+:   */\n");
    res.push_str(
        "/*                                                    +:+ +:+         +:+     */\n",
    );
    res.push_str("/*   By: ");
    res.push_str(&name);
    res.push_str(" <");
    res.push_str(&email);
    res.push_str(">");
    assert!(40 >= name.len() + email.len());
    res.push_str(" ".repeat(40 - name.len() - email.len()).as_str());
    res.push_str("+#+  +:+       +#+        */\n");
    res.push_str(
        "/*                                                +#+#+#+#+#+   +#+           */\n",
    );

    res.push_str("/*   Created: ");
    let meta = fs::metadata(path).expect("metadata on this path does not exist");
    let created = meta.created().expect("created time not available");
    let c_dur = created
        .duration_since(UNIX_EPOCH)
        .expect("conversion failed: created");
    let created = DateTime::from_timestamp_nanos(c_dur.as_nanos().try_into().unwrap())
        .with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
    res.push_str(&created.format("%Y/%m/%d %H:%M:%S").to_string());
    res.push_str(" by ");
    res.push_str(&name);
    assert!(18 >= name.len());
    res.push_str(" ".repeat(18 - name.len()).as_str());
    res.push_str("#+#    #+#             */\n");

    res.push_str("/*   Updated: ");
    let modified = meta.modified().expect("modified time not available");
    let m_dur = modified
        .duration_since(UNIX_EPOCH)
        .expect("conversion failed: modified");
    let modified = DateTime::from_timestamp_nanos(m_dur.as_nanos().try_into().unwrap())
        .with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
    res.push_str(&modified.format("%Y/%m/%d %H:%M:%S").to_string());
    res.push_str(" by ");
    res.push_str(&name);
    assert!(17 >= name.len());
    res.push_str(" ".repeat(17 - name.len()).as_str());
    res.push_str("###   ########.fr       */\n");
    res.push_str(
        "/*                                                                            */\n",
    );
    res.push_str(
        "/* ************************************************************************** */\n",
    );

    res
}

fn _42_replace(header: _42Header, updater_name: String, path: &Path, file_name: &str) -> String {
    let mut res = String::new();
    res.push_str(
        "/* ************************************************************************** */\n",
    );
    res.push_str(
        "/*                                                                            */\n",
    );
    res.push_str(
        "/*                                                        :::      ::::::::   */\n",
    );
    res.push_str("/*   ");
    res.push_str(file_name);
    assert!(51 >= file_name.len());
    res.push_str(" ".repeat(51 - file_name.len()).as_str());
    res.push_str(":+:      :+:    :+:   */\n");
    res.push_str(
        "/*                                                    +:+ +:+         +:+     */\n",
    );
    res.push_str("/*   By: ");
    res.push_str(&header.name);
    res.push_str(" <");
    res.push_str(&header.email);
    res.push_str(">");
    assert!(40 >= header.name.len() + header.email.len());
    res.push_str(
        " ".repeat(40 - header.name.len() - header.email.len())
            .as_str(),
    );
    res.push_str("+#+  +:+       +#+        */\n");
    res.push_str(
        "/*                                                +#+#+#+#+#+   +#+           */\n",
    );

    res.push_str("/*   Created: ");
    let meta = fs::metadata(path).expect("metadata on this path does not exist");
    res.push_str(&header.created.format("%Y/%m/%d %H:%M:%S").to_string());
    res.push_str(" by ");
    res.push_str(&header.name);
    assert!(18 >= header.name.len());
    res.push_str(" ".repeat(18 - header.name.len()).as_str());
    res.push_str("#+#    #+#             */\n");

    res.push_str("/*   Updated: ");
    let modified = meta.modified().expect("modified time not available");
    let m_dur = modified
        .duration_since(UNIX_EPOCH)
        .expect("conversion failed: modified");
    let modified = DateTime::from_timestamp_nanos(m_dur.as_nanos().try_into().unwrap())
        .with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
    res.push_str(&modified.format("%Y/%m/%d %H:%M:%S").to_string());
    res.push_str(" by ");
    res.push_str(&updater_name);
    assert!(17 >= updater_name.len());
    res.push_str(" ".repeat(17 - updater_name.len()).as_str());
    res.push_str("###   ########.fr       */\n");
    res.push_str(
        "/*                                                                            */\n",
    );
    res.push_str(
        "/* ************************************************************************** */\n",
    );

    res
}
