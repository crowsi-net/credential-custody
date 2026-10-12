//! Synthetic native child for bounded-input and whole-process-group tests.
use std::{fs, io::{Read, Write}, process::{Command, Stdio}, time::Duration};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).is_some_and(|value| value == "--linger") { linger(); return; }
    let index = args.iter().position(|value| value == "--config").expect("config argument");
    let config = fs::read_to_string(&args[index + 1]).expect("pinned fixture config");
    let (scenario, pidfile) = config.split_once('\n').expect("fixture scenario");
    if scenario == "blocked-input" {
        fs::write(pidfile, std::process::id().to_string()).expect("provider pid");
        linger(); return;
    }
    // Successful providers consume input before returning, avoiding a broken-pipe race.
    std::io::stdin().read_to_end(&mut Vec::new()).expect("request");
    if scenario.starts_with("grandchild-") {
        let child = Command::new(std::env::current_exe().expect("fixture image"))
            .arg("--linger").stdin(Stdio::null()).spawn().expect("grandchild");
        fs::write(pidfile, child.id().to_string()).expect("grandchild pid");
        if scenario == "grandchild-timeout" { linger(); return; }
    }
    let response = if scenario == "finite" { br#"{"ok":true}"#.as_slice() } else { b"{}" };
    std::io::stdout().write_all(response).expect("response");
}
fn linger() { std::thread::sleep(Duration::from_secs(30)); }
