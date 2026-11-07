use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
};

struct Sample(PathBuf);

impl Sample {
    fn new(label: &str, extension: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "inject-42h-{label}-{}.{}",
            std::process::id(),
            extension
        ));
        let body = if extension == "h" {
            "\n#ifndef SAMPLE_H\n# define SAMPLE_H\n\n#endif\n"
        } else {
            "\nint\tmain(void)\n{\n\treturn (0);\n}\n"
        };
        fs::write(&path, body).unwrap();
        Self(path)
    }

    fn inject(&self, email: &str) -> Output {
        Command::new(env!("CARGO_BIN_EXE_inject_42h"))
            .args(["--name", "testuser", "--email", email, "--path"])
            .arg(&self.0)
            .args(["--offset", "1"])
            .output()
            .unwrap()
    }
}

impl Drop for Sample {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

#[test]
fn long_email_remains_one_valid_header_after_repeated_saves() {
    for extension in ["c", "h"] {
        // Include the exact capacity boundary as well as a longer school email.
        for email in [
            "student.with.long.name@learner.42.tech",
            "abcdefghijklmnopqrstuvwxyz@42.fr",
            "student@42.fr",
        ] {
            let sample = Sample::new("roundtrip", extension);
            let body = fs::read_to_string(&sample.0).unwrap();
            let mut created = None;
            for save in 1..=5 {
                let output = sample.inject(email);
                assert!(output.status.success(), "{:?}", output);
                let content = fs::read_to_string(&sample.0).unwrap();
                let lines: Vec<_> = content.split_inclusive('\n').collect();
                assert_eq!(lines[11..].concat(), body, "body changed on save {save}");
                assert!(
                    lines[5].contains("> "),
                    "By separator missing on save {save}"
                );
                assert!(lines[..11]
                    .iter()
                    .all(|line| line.trim_end_matches('\n').len() == 80));
                match &created {
                    None => created = Some(lines[7].to_string()),
                    Some(expected) => assert_eq!(lines[7], expected),
                }
            }
        }
    }
}

#[test]
fn malformed_existing_header_is_rejected_without_changing_any_bytes() {
    let sample = Sample::new("malformed", "c");
    assert!(sample.inject("test@42.fr").status.success());
    let malformed = fs::read_to_string(&sample.0)
        .unwrap()
        .replacen("+#+  +:+", "+##  +:+", 1);
    fs::write(&sample.0, &malformed).unwrap();
    let output = sample.inject("test@42.fr");
    assert!(
        !output.status.success(),
        "malformed header must not be rewritten"
    );
    assert_eq!(fs::read_to_string(&sample.0).unwrap(), malformed);
}
