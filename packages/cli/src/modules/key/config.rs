use data_doc_derive::GetDataDocs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, GetDataDocs, Default)]
pub struct KeyConfig {}

pub const SERVICE: &str = "beaker";
