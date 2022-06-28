use data_doc::{DataDoc, GetDataDocs};
use data_doc_derive::GetDataDocs;
use pretty_assertions::assert_eq;
use std::collections::HashMap;

#[test]
fn test_simple_struct_with_map() {
    #[derive(GetDataDocs)]
    struct Simple {
        /// Name for simple example
        #[allow(dead_code)]
        name: String,

        /// Length of something I'm not so sure what it's for
        /// This doc string is so long
        /// Here is some code blocks `println!("hello");` you see?:
        #[allow(dead_code)]
        length: u64,
    }

    #[derive(GetDataDocs)]
    struct SimpleMap {
        /// Map for simple struct
        #[allow(dead_code)]
        simple: HashMap<String, Simple>,
    }

    assert_eq!(
        SimpleMap::get_data_docs(),
        vec![DataDoc::new(
            "simple".to_string(),
            "HashMap < String, Simple >".to_string(),
            vec!["Map for simple struct".to_string()],
            vec![
                DataDoc::new(
                    "name".to_string(),
                    "String".to_string(),
                    vec!["Name for simple example".to_string()],
                    vec![]
                ),
                DataDoc::new(
                    "length".to_string(),
                    "u64".to_string(),
                    vec![
                        "Length of something I'm not so sure what it's for".to_string(),
                        "This doc string is so long".to_string(),
                        "Here is some code blocks `println!(\"hello\");` you see?:".to_string()
                    ],
                    vec![]
                )
            ]
        ),]
    );
}
