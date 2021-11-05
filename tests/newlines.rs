mod newlines {
    use std::process::Command;

    #[test]
    fn newlines() {
        let output = Command::new("./target/debug/rdf")
            .arg("./tests/newlines.txt")
            .output()
            .unwrap();

        assert_eq!(String::from_utf8_lossy(&output.stdout), "a\nb\nc\nd\n");
    }
}
