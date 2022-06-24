#[derive(Default, Debug, PartialEq, Eq)]
pub struct StructDoc {
    pub ident: String,
    pub desc: Vec<String>,
    pub sub_docs: Vec<StructDoc>,
}

impl StructDoc {
    pub fn new(ident: String, desc: Vec<String>, sub_docs: Vec<StructDoc>) -> Self {
        Self {
            ident,
            desc,
            sub_docs,
        }
    }
}

pub trait GetDocs {
    fn get_struct_docs() -> Vec<StructDoc>;
}
