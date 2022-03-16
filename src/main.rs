use std::{error::Error, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut in_log_stacktrace = false;
    let mut iter = stdin.lines().enumerate();
    while let Some((num, line)) = iter.next() {
        let line = line?;

        if in_log_stacktrace {
            if line.starts_with("(Filename") {
                in_log_stacktrace = false;
                // Skip empty line following the file name
                iter.next();
            }
            continue;
        }

        if line.starts_with("UnityEngine.StackTraceUtility:ExtractStackTrace") {
            in_log_stacktrace = true;
            continue;
        }

        println!("[{num}] {line}");
    }

    Ok(())
}
