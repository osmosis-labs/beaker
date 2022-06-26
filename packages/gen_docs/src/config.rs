use crate::document::Document;
use get_docs::{GetDocs, StructDoc};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Tag};
use serde::Serialize;

fn recur_config(
    document: &mut Document,
    struct_docs: Vec<StructDoc>,
) -> Result<(), Box<dyn std::error::Error>> {
    document.0.push(Event::Start(Tag::List(None)));

    for d in struct_docs {
        document.0.push(Event::Start(Tag::Item));
        document.0.push(Event::Code(d.ident.into()));
        document.0.push(Event::HardBreak);

        document.0.push(Event::Start(Tag::Paragraph));
        for ds in d.desc {
            document.0.push(Event::Text(ds.into()));
            document.0.push(Event::HardBreak);
        }

        recur_config(document, d.sub_docs)?;

        document.0.push(Event::Start(Tag::Paragraph));

        document.0.push(Event::End(Tag::Item));
    }
    document.0.push(Event::End(Tag::List(None)));

    Ok(())
}

pub fn config_to_md<C: GetDocs + Default + Serialize>(
    module_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut document = Document(Vec::new());
    let mut result = String::new();

    document.header(module_name.to_string(), HeadingLevel::H1);
    recur_config(&mut document, C::get_struct_docs())?;

    document.0.push(Event::Rule);
    document.header("Default Config".into(), HeadingLevel::H2);

    let s = toml::to_string_pretty(&C::default()).unwrap();

    document
        .0
        .push(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(
            "toml".into(),
        ))));

    document.0.push(Event::Text(s.into()));

    document
        .0
        .push(Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(
            "toml".into(),
        ))));

    crate::cmark::custom_cmark(document.0.iter(), &mut result)?;
    Ok(result)
}

#[macro_export]
macro_rules! generate_config_doc {
    ($path:expr, { #[no_wrap] $prefix:ident : $t:ty }) => {{
        let config_md = gen_docs::config::config_to_md::<$t>(stringify!($prefix))?;
        let mut file = std::fs::File::create($path.join(format!("{}.md", stringify!($prefix))))?;
        file.write_all(config_md.as_bytes())?;
    }};
    ($path:expr, { $prefix:ident : $t:ty }) => {{
        #[allow(non_camel_case_types)]
        #[derive(Serialize, Default, GetDocs)]
        struct $prefix {
            $prefix: $t,
        }
        let config_md = gen_docs::config::config_to_md::<$prefix>(stringify!($prefix))?;
        let mut file = std::fs::File::create($path.join(format!("{}.md", stringify!($prefix))))?;
        file.write_all(config_md.as_bytes())?;
    }};

    ($path:expr, { $( $(#[$attr:tt])? $prefix:ident : $t:ty,)+ }) => {{
        std::fs::create_dir_all($path.clone())?;
        $(
            gen_docs::generate_config_doc!($path, { $(#[$attr])? $prefix: $t });
        )+

        let default_config = vec![$( gen_docs::default_config!($(#[$attr])? $prefix : $t), ) +].join("\n\n");

        let readme = vec![
        "# Beaker Configuration",
        "The following list is the configuration references for beaker which can be used in `Beaker.toml`.",
        "",
        $( format!("- [{}](./{}.md)", stringify!($prefix), stringify!($prefix)).as_str(), ) +
        "---",
        "",
        "# Default Config",
        "```toml",
        default_config.as_str(),
        "```"
        ].join("\n");

        let mut file = std::fs::File::create($path.join("README.md"))?;
        file.write_all(readme.as_bytes())?;
    }};
}

#[macro_export]
macro_rules! default_config {
    (#[$attr:tt] $prefix:ident : $t:ty) => {
        toml::to_string_pretty(&<$t>::default())?
    };
    ($prefix:ident : $t:ty) => {{
        {
            #[allow(non_camel_case_types)]
            #[derive(Serialize, Default, GetDocs)]
            struct $prefix {
                $prefix: $t,
            }
            vec![
                format!("# {}", stringify!($prefix)),
                String::new(),
                toml::to_string_pretty(&<$prefix>::default())?,
            ]
            .join("\n")
        }
    }};
}
