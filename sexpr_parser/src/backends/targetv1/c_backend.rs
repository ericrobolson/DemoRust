#[cfg(test)]
mod tests {
    use crate::{
        backends::file::File,
        intermediate_representation::{self, *},
    };

    use super::super::*;

    fn target() -> TargetV1 {
        let json = std::fs::read_to_string("backends/backend_c.json").unwrap();

        TargetV1::deserialize(&json).unwrap()
    }

    #[test]
    fn simple_hello_world() {
        let input = Artifact {
            artifact_type: ArtifactType::Executable(Executable {
                file_name: "hello_world".to_string(),
                main_module: Module {
                    file_name: "main".into(),
                },
            }),
        };

        let main_c = r#"
#include <stdio.h>
#include "main.h"

int main()
{
    printf("hi \n\n");
    return 0;
}
"#;
        let main_h = r#""#;

        // todo: remove these
        let main_c = "";
        let main_h = "";
        // end todo

        let expected = vec![
            File {
                contents: main_c.trim().to_string(),
                file_name: "main.c".into(),
            },
            // File {
            //     contents: main_h.trim().to_string(),
            //     file_name: "main.h".into(),
            // },
        ];

        let actual = target().compile(input);

        assert_eq!(expected, actual)
    }
}
