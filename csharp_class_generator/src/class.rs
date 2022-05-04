use crate::types::*;

pub struct Class {
    pub id: Str,
    pub comment: Str,
    pub generics: Vec<Str>,
}
impl Class {
    pub fn compile(&self) -> String {
        let comment = self
            .comment
            .replace("\r\n", "\n")
            .split("\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| format!("\n// {}", s))
            .collect::<Vec<String>>()
            .join("");

        let id = self.id.trim();
        let generics = if self.generics.is_empty() {
            String::default()
        } else {
            format!(
                "<{}>",
                self.generics
                    .iter()
                    .map(|g| g.trim().to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            )
        };

        format!(
            "
{comment}
public class {id}{generics} {{

}}
		",
            comment = comment,
            id = id,
            generics = generics
        )
    }
}
