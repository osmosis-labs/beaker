use data_doc::{DataDoc, GetDataDocs};
use data_doc_derive::GetDataDocs;
use pretty_assertions::assert_eq;

#[test]
fn test_nested_struct() {
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
    struct Nested {
        /// This looks simple, but we can make it more complicated
        /// You will see it soon
        #[allow(dead_code)]
        simple: Simple,

        #[allow(dead_code)]
        no_doc: String,

        /// Doesn't really matter that much, please ignore
        #[allow(dead_code)]
        with_doc: String,
    }

    assert_eq!(
        Nested::get_data_docs(),
        vec![
            DataDoc::new(
                "simple".to_string(),
                "Simple".to_string(),
                vec![
                    "This looks simple, but we can make it more complicated".to_string(),
                    "You will see it soon".to_string()
                ],
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
            ),
            DataDoc::new("no_doc".to_string(), "String".to_string(), vec![], vec![]),
            DataDoc::new(
                "with_doc".to_string(),
                "String".to_string(),
                vec!["Doesn't really matter that much, please ignore".to_string()],
                vec![]
            )
        ]
    );
}
