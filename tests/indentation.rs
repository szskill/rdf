const SPACE_INDENTATION_OUTPUT: &str = "This
    file
        is
            indented
                each
                    line!
";

const TAB_INDENTATION_OUTPUT: &str = "This
\tfile
\t\tis
\t\t\tindented
\t\t\t\teach
\t\t\t\t\tline!
";

mod indentation {
    use std::process::Command;

    use crate::SPACE_INDENTATION_OUTPUT;
    use crate::TAB_INDENTATION_OUTPUT;

    #[test]
    fn space_indentation() {
        let output = Command::new("./target/debug/rdf")
            .arg("./tests/space_indentation.txt")
            .output()
            .unwrap();
        
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            SPACE_INDENTATION_OUTPUT
        );
    }

    #[test]
    fn tab_indentation() {
        let output = Command::new("./target/debug/rdf")
            .arg("./tests/tab_indentation.txt")
            .output()
            .unwrap();
        
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            TAB_INDENTATION_OUTPUT
        );
    }
}
