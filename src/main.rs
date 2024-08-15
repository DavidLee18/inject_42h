use chrono::DateTime;
use clap::Parser;
use std::{fs, time::UNIX_EPOCH};

#[derive(Debug, Parser)]
struct Args {
    #[arg(long)]
    name: String,

    #[arg(long)]
    email: String,

    #[arg(long)]
    path: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let content = fs::read_to_string(&args.path)?;
    let header = _42(
        args.name,
        args.email,
        args.path
            .split('/')
            .last()
            .expect("file name cannot be retreived"),
        &args.path,
    );
    fs::write(&args.path, header + &content)?;
    Ok(())
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
    let created = DateTime::from_timestamp_nanos(c_dur.as_nanos().try_into().unwrap());
    res.push_str(&created.format("%Y/%m/%d %H:%M:%S").to_string());
    res.push_str(" by ");
    res.push_str(&name);
    res.push_str(" ".repeat(18 - name.len()).as_str());
    res.push_str("#+#    #+#             */\n");

    res.push_str("/*   Updated: ");
    let modified = meta.modified().expect("modified time not available");
    let m_dur = modified
        .duration_since(UNIX_EPOCH)
        .expect("conversion failed: modified");
    let modified = DateTime::from_timestamp_nanos(m_dur.as_nanos().try_into().unwrap());
    res.push_str(&modified.format("%Y/%m/%d %H:%M:%S").to_string());
    res.push_str(" by ");
    res.push_str(&name);
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
