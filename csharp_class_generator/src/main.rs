mod class;
mod function;
mod generic_adt;
mod lispy;
mod types;

fn main() {
    let c = class::Class {
        comment: "testy mctest
        AND SHT
        STUFF
        
        ",
        id: "Foo",
        generics: vec!["A", "B", "C"],
    };

    println!("{}", c.compile());

    let f = function::Function {
        id: "Baz",
        return_type: Some("Int32"),
    };
    println!("{}", f.compile());
}
