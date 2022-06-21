use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::vec;

use beaker::Cli;
use clap::CommandFactory;
use clap::{App, ArgSettings};
use pulldown_cmark::{Event, HeadingLevel, LinkType, Tag};
use pulldown_cmark_to_cmark::{cmark_with_options, Options};

struct Document<'a>(Vec<Event<'a>>);

impl<'a> Document<'a> {
    fn header(&mut self, text: String, level: HeadingLevel) {
        self.0.push(Event::Start(Tag::Heading(level, None, vec![])));
        self.0.push(Event::Text(text.into()));
        self.0.push(Event::End(Tag::Heading(level, None, vec![])));
    }

    fn header_code(&mut self, text: String, level: HeadingLevel) {
        self.0.push(Event::Start(Tag::Heading(level, None, vec![])));
        self.0.push(Event::Code(text.into()));
        self.0.push(Event::End(Tag::Heading(level, None, vec![])));
    }

    fn paragraph(&mut self, text: String) {
        self.0.push(Event::Start(Tag::Paragraph));
        self.0.push(Event::Text(text.into()));
        self.0.push(Event::End(Tag::Paragraph));
    }

    fn link(&mut self, text: String, link: String) {
        self.0.push(Event::Start(Tag::Link(
            LinkType::Inline,
            link.clone().into(),
            "".into(),
        )));

        self.0.push(Event::Text(text.into()));

        self.0.push(Event::End(Tag::Link(
            LinkType::Inline,
            link.into(),
            "".into(),
        )));
    }
}

fn build_page(doc: &mut Document, app: &App, level: HeadingLevel, prefix: Vec<String>) {
    build_command(doc, &prefix, level, app, true);

    if app.get_subcommands().next().is_some() {
        let lv_plus_one = increase_level(&level);
        let lv_plus_two = increase_level(&lv_plus_one);
        doc.header("Subcommands".into(), lv_plus_one);

        for cmd in app.get_subcommands() {
            let is_first = app.get_subcommands().next() == Some(cmd);
            if !is_first {
                doc.0.push(Event::Rule);
            }

            let mut prefix = prefix.clone();
            prefix.push(app.get_name().to_owned());
            build_command(doc, &prefix, lv_plus_two, cmd, false);
        }
    }
}

fn build_command(
    doc: &mut Document,
    prefix: &[String],
    level: HeadingLevel,
    app: &App,
    is_main: bool,
) {
    let cmd_path = [&prefix[..(prefix.len() - 1)], &[app.get_name().to_owned()]].concat();
    let full_cmd_name = cmd_path.join(" ");
    doc.header_code(full_cmd_name.clone(), level);

    if let Some(about) = app.get_about() {
        doc.paragraph(about.into());
    }

    if app.get_subcommands().next().is_some() && !is_main {
        doc.0.push(Event::Start(Tag::Paragraph));
        doc.link(
            format!("> `{full_cmd_name}`'s subcommands"),
            format!("./{}.md", cmd_path.join("_")),
        );
        doc.0.push(Event::End(Tag::Paragraph));
    }

    if let Some(author) = app.get_author() {
        doc.paragraph(format!("Author: {}", author));
    }
    if let Some(version) = app.get_version() {
        let long_version = if let Some(msg) = app.get_long_version() {
            format!(" ({})", msg)
        } else {
            "".into()
        };
        doc.paragraph(format!("Version: {}{}", version, long_version));
    }
    if app.get_arguments().next().is_some() {
        doc.paragraph("Arguments:".into());
        doc.0.push(Event::Start(Tag::List(None)));

        for arg in app.get_arguments() {
            doc.0.push(Event::Start(Tag::Item));
            doc.0.push(Event::Start(Tag::Paragraph));

            let mut def = String::new();
            if let Some(short) = arg.get_short() {
                def.push('-');
                def.push(short);
            }
            if let Some(long) = arg.get_long() {
                if arg.get_short().is_some() {
                    def.push('/');
                }
                def.push_str("--");
                def.push_str(long);
            }

            if arg.is_set(ArgSettings::TakesValue) {
                def.push_str(" <");
                def.push_str(arg.get_name());
                def.push('>');
            }

            doc.0.push(Event::Code(def.into()));

            let mut text = String::new();
            if let Some(help) = arg.get_help() {
                if arg.get_short().is_some() || arg.get_long().is_some() {
                    text.push_str(": ");
                }
                text.push_str(help);
            }
            doc.0.push(Event::Text(text.into()));

            doc.0.push(Event::End(Tag::Paragraph));
            doc.0.push(Event::End(Tag::Item));
        }

        doc.0.push(Event::End(Tag::List(None)));
    }
}

fn increase_level(level: &HeadingLevel) -> HeadingLevel {
    match level {
        HeadingLevel::H1 => HeadingLevel::H2,
        HeadingLevel::H2 => HeadingLevel::H3,
        HeadingLevel::H3 => HeadingLevel::H4,
        HeadingLevel::H4 => HeadingLevel::H5,
        HeadingLevel::H5 => HeadingLevel::H6,
        HeadingLevel::H6 => HeadingLevel::H6,
    }
}

/// Convert a clap App to markdown documentation
///
/// # Parameters
///
/// - `app`: A reference to a clap application definition
/// - `level`: The level for first markdown headline. If you for example want to
///     render this beneath a `## Usage` headline in your readme, you'd want to
///     set `level` to `2`.
pub fn app_to_md(
    app: &App<'_>,
    prefix: &[String],
    level: HeadingLevel,
) -> Result<String, anyhow::Error> {
    let mut document = Document(Vec::new());
    build_page(&mut document, app, level, prefix.to_vec());
    let mut result = String::new();

    cmark_with_options(
        document.0.iter(),
        &mut result,
        Options {
            newlines_after_headline: 2,
            newlines_after_paragraph: 2,
            newlines_after_codeblock: 2,
            newlines_after_table: 2,
            newlines_after_rule: 2,
            newlines_after_list: 2,
            newlines_after_blockquote: 2,
            newlines_after_rest: 1,
            code_block_token_count: 3,
            code_block_token: '`',
            list_token: '*',
            emphasis_token: '*',
            strong_token: "**",
        },
    )?;
    Ok(result)
}

fn recur_gen(cmd: &App<'_>, path: PathBuf, prefix: &[String]) -> Result<(), anyhow::Error> {
    std::fs::create_dir_all(path.as_path())?;

    let mut prefix = prefix.to_vec();
    prefix.push(cmd.get_name().into());

    let md_string = app_to_md(cmd, prefix.as_slice(), HeadingLevel::H1)?;
    let mut file = File::create(path.join(format!("{}.md", prefix.join("_"))))?;
    file.write_all(md_string.as_bytes())?;

    for sub in cmd.get_subcommands() {
        if sub.get_subcommands().next().is_some() {
            recur_gen(sub, path.clone(), prefix.as_slice())?;
        }
    }

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let app = Cli::command();

    let docs_path = Path::new("docs");
    if docs_path.exists() {
        std::fs::remove_dir_all(docs_path)?;
    }
    recur_gen(&app, docs_path.to_path_buf(), &[])?;

    std::fs::rename(
        docs_path.join(format!("{}.md", app.get_name())),
        docs_path.join("README.md"),
    )?;

    Ok(())
}
