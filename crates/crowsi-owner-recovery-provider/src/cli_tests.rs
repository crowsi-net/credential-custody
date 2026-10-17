use crate::{cli::run, input::number, output::write_descriptor};

#[test]
fn command_shape_never_accepts_a_secret_argument() {
    assert_eq!(
        run(&["approve-stdin".into(), "phrase".into()]),
        Err("usage-invalid")
    );
    assert_eq!(
        run(&["verify".into(), "secret".into()]),
        Err("usage-invalid")
    );
}

#[test]
fn public_number_parser_is_closed() {
    assert_eq!(number("42"), Ok(42));
    assert_eq!(number("-1"), Err("number-invalid"));
}

#[test]
fn descriptor_file_is_newline_terminated_and_never_overwritten() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let path = directory.path().join("descriptor.json");
    write_descriptor(
        path.to_str().expect("path"),
        &serde_json::json!({"revision": 1}),
    )
    .expect("first write");
    assert_eq!(
        std::fs::read_to_string(&path).expect("read"),
        "{\"revision\":1}\n"
    );
    assert_eq!(
        write_descriptor(
            path.to_str().expect("path"),
            &serde_json::json!({"revision": 2})
        ),
        Err("descriptor-create-failed")
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            std::fs::metadata(path)
                .expect("metadata")
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
}
