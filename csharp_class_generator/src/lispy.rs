pub struct Runtime {}
impl Runtime {
    pub fn compile() -> String {
        "
public class Env {

}
"
        .into()
    }

    /// Built in primitives.
    pub fn builtins() -> Vec<Method> {
        vec![
            Method { id: "quote" },
            Method { id: "atom" },
            Method { id: "eq" },
        ]
    }
}

pub struct Method {
    id: &'static str,
}
