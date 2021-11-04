const NO_ARGS_OUTPUT: &str = "rdf - read a file
usage: rdf <filename>
";

const NORMAL_OUTPUT: &str = "I am a normal text file.\n";

mod basics {
    use std::process::Command;

    use crate::NO_ARGS_OUTPUT;
    use crate::NORMAL_OUTPUT;

    #[test]
    fn no_args() {
        let output = Command::new("./target/debug/rdf").output().unwrap();

        assert_eq!(String::from_utf8_lossy(&output.stderr), NO_ARGS_OUTPUT);
    }

    #[test]
    fn normal() {
        let output = Command::new("./target/debug/rdf")
            .arg("./tests/normal.txt")
            .output()
            .unwrap();

        assert_eq!(String::from_utf8_lossy(&output.stdout), NORMAL_OUTPUT);
    }
}
