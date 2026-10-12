//! Bounded deterministic stdio fixture; no network, DPAPI or real credentials.

use std::io::{Read, Write};

fn read_frame(input: &mut impl Read) -> Vec<u8> {
    let mut prefix = [0; 4];
    input.read_exact(&mut prefix).expect("frame prefix");
    let size = usize::try_from(u32::from_be_bytes(prefix)).expect("frame size");
    assert!((1..=65536).contains(&size));
    let mut body = vec![0; size];
    input.read_exact(&mut body).expect("frame body");
    body
}

fn write_frame(output: &mut impl Write, body: &[u8]) {
    output.write_all(&u32::try_from(body.len()).expect("size").to_be_bytes()).expect("prefix");
    output.write_all(body).expect("body");
}

fn field<'a>(body: &'a str, key: &str) -> &'a str {
    let prefix = format!("\"{key}\":\"");
    let tail = body.split_once(&prefix).expect("fixture field").1;
    tail.split_once('"').expect("fixture string").0
}

fn main() {
    if std::env::current_exe().expect("helper path").file_name().is_some_and(|name| name == "blocked.exe") {
        std::thread::sleep(std::time::Duration::from_secs(5));
        return;
    }
    assert_eq!(std::env::vars_os().count(), 0, "helper environment must be empty");
    let mut input = std::io::stdin().lock();
    let mut output = std::io::stdout().lock();
    let body = read_frame(&mut input);
    let body = std::str::from_utf8(&body).expect("fixture JSON");
    assert!(!body.contains("credential-material"));
    let operation_only = body.contains("operation-request/v3");
    let operation = if operation_only { "sign" } else { field(body, "operation") };
    if operation == "put" {
        assert_eq!(read_frame(&mut input), b"credential-material");
    }
    assert_eq!(input.read(&mut [0]).expect("EOF"), 0);
    let request_id = field(body, "request_id");
    let response = if operation_only {
        format!(
            "{{\"schema\":\"crowsi://platform-custody/operation-response/v3\",\"request_id\":{request_id:?},\"credential_id\":{:?},\"revision\":{:?},\"result\":{{\"kind\":\"signature\",\"algorithm\":\"ed25519\",\"value_hex\":{:?}}},\"contains_secret_values\":false,\"secret_follows\":false}}",
            field(body, "credential_id"), field(body, "expected_revision"), "ab".repeat(64)
        )
    } else {
        let metadata = if operation == "doctor" {
            "null".into()
        } else {
            format!("{{\"credential_id\":\"github.app.pem\",\"revision\":\"rev1:{}\",\"provider_kind\":\"windows-dpapi-user\"}}", "a".repeat(64))
        };
        format!("{{\"schema\":\"crowsi://platform-custody/response/v1\",\"protocol\":\"crowsi-windows-custody-v1\",\"request_id\":{request_id:?},\"result\":{{\"state\":\"ready\",\"operation\":{operation:?},\"metadata\":{metadata},\"secret_follows\":{}}},\"contains_secret_values\":false}}", operation == "get")
    };
    write_frame(&mut output, response.as_bytes());
    if operation == "get" {
        write_frame(&mut output, b"credential-material");
    }
}
