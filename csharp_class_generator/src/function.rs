use crate::types::*;

pub struct Function {
    pub id: Str,
    pub return_type: Option<Str>,
}
impl Function {
    pub fn compile(&self) -> String {
        let id = self.id.trim();
        let return_type = match self.return_type {
            Some(t) => t,
            None => "void",
        };

        format!(
            "
public {return_type} {id}(){{

}}
		",
            return_type = return_type,
            id = id
        )
    }
}
