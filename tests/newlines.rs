mod basics {
    use std::process::Command;

    #[test]
    fn no_args() {
        let output = Command::new("./target/debug/rdf")
            .arg("./tests/newlines.txt")
            .output()
            .unwrap();

        assert_eq!(String::from_utf8_lossy(&output.stderr), "\n\n\n");
    }
}
