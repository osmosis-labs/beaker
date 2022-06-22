use derive_get_docs::GetDocs;
use get_docs::GetDocs;

#[derive(GetDocs)]
struct Simple {
    /// Name for simple example
    #[allow(dead_code)]
    name: String,

    /// Length of something I'm not so sure what it's for
    /// This doc string is so long
    #[allow(dead_code)]
    length: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        Simple::get_docs(),
        vec![
            (
                "name".to_string(),
                vec!["Name for simple example".to_string()]
            ),
            (
                "length".to_string(),
                vec![
                    "Length of something I'm not so sure what it's for".to_string(),
                    "This doc string is so long".to_string()
                ]
            )
        ]
    );

    Ok(())
}
