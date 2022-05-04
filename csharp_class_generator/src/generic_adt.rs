/// Generic ADT class generator.
pub struct GenericAdt {
    items: Vec<char>,
}
impl GenericAdt {
    pub fn generate_csharp() -> String {
        let mut strings = vec![];
        for i in 0..26 {
            let mut adt = Self { items: vec![] };
            for j in 0..i {
                let ascii = 'A' as u8 + j;
                let ch = ascii as char;
                adt.items.push(ch);
            }
            strings.push(adt.print());
        }

        strings.remove(0);
        strings.join("\n\n")
    }

    fn print(&self) -> String {
        let items: Vec<char> = self.items.iter().map(|c| c.to_ascii_uppercase()).collect();

        let template = "
public class Adt<__GENERICS__> {
\tprivate byte _idx;
\t__MEMBERS__
\t__CONSTRUCTORS__
        public void Match(__MATCHPROPS__) {
            switch (_idx){
                __SWITCH__
            }
        }
}
        ";

        let generics = items
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<String>>()
            .join(", ");

        let members = items
            .iter()
            .map(|c| format!("\n\tprivate {} _{};", c, c.to_lowercase()))
            .collect::<Vec<String>>()
            .join("");

        let constructors = items
            .iter()
            .enumerate()
            .map(|(i, c)| {
                format!(
                    "
        public Adt({ch} value) {{
            _{chLower} = value;
            _idx = {idx};
        }}
",
                    ch = c,
                    chLower = c.to_lowercase(),
                    idx = i
                )
            })
            .collect::<Vec<String>>()
            .join("");

        let match_props = items
            .iter()
            .enumerate()
            .map(|(i, c)| format!("Action<{chType}> {ch}", chType = c, ch = c.to_lowercase()))
            .collect::<Vec<String>>()
            .join(", ");

        let switches = items
            .iter()
            .enumerate()
            .map(|(i, c)| {
                format!(
                    "
                case ({idx}):
                    {ch}(_{ch});
                    break;
",
                    idx = i,
                    ch = c.to_lowercase()
                )
            })
            .collect::<Vec<String>>()
            .join("");

        template
            .replace("__GENERICS__", &generics)
            .replace("__MEMBERS__", &members)
            .replace("__CONSTRUCTORS__", &constructors)
            .replace("__MATCHPROPS__", &match_props)
            .replace("__SWITCH__", &switches)
    }
}
