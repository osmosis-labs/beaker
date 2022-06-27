use derive_get_docs::GetDocs;
use get_docs::{GetDocs, StructDoc};
use pretty_assertions::assert_eq;

#[test]
fn test_enum() {
    #[derive(GetDocs)]
    enum Simple {
        /// First variant
        #[allow(dead_code)]
        First,

        /// Second variant
        /// So doc is longer
        #[allow(dead_code)]
        Second,
    }

    assert_eq!(
        Simple::get_struct_docs(),
        vec![
            StructDoc::new(
                "First".to_string(),
                "Simple::First".to_string(),
                vec!["First variant".to_string()],
                vec![]
            ),
            StructDoc::new(
                "Second".to_string(),
                "Simple::Second".to_string(),
                vec!["Second variant".to_string(), "So doc is longer".to_string(),],
                vec![]
            )
        ]
    );

    #[derive(GetDocs)]
    enum WithAttr {
        /// First variant
        #[allow(dead_code)]
        First {
            /// Some attr
            some_attr: String,

            /// Other attr
            other_attr: bool,
        },

        /// Second variant
        /// So doc is longer
        #[allow(dead_code)]
        Second,
    }

    assert_eq!(
        WithAttr::get_struct_docs(),
        vec![
            StructDoc::new(
                "First".to_string(),
                "WithAttr::First".to_string(),
                vec!["First variant".to_string()],
                vec![
                    StructDoc::new(
                        "some_attr".to_string(),
                        "String".to_string(),
                        vec!["Some attr".to_string()],
                        vec![]
                    ),
                    StructDoc::new(
                        "other_attr".to_string(),
                        "bool".to_string(),
                        vec!["Other attr".to_string()],
                        vec![]
                    ),
                ]
            ),
            StructDoc::new(
                "Second".to_string(),
                "WithAttr::Second".to_string(),
                vec!["Second variant".to_string(), "So doc is longer".to_string(),],
                vec![]
            )
        ]
    );
}
