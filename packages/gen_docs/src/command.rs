use crate::{cmark::custom_cmark, document::Document};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::{builder::Str, Command};

use pulldown_cmark::{Event, HeadingLevel, Tag};

fn build_page(doc: &mut Document, cmd: &Command, level: HeadingLevel, prefix: Vec<String>) {
    build_command(doc, &prefix, level, cmd, true);

    if cmd.get_subcommands().next().is_some() {
        let lv_plus_one = increase_level(&level);
        let lv_plus_two = increase_level(&lv_plus_one);
        doc.header("Subcommands".into(), lv_plus_one);

        for cmd in cmd.get_subcommands() {
            let is_first =
                cmd.get_subcommands().next().map(|s| s.get_name()) == Some(cmd.get_name());
            if !is_first {
                doc.0.push(Event::Rule);
            }

            let mut prefix = prefix.clone();
            prefix.push(cmd.get_name().to_owned());
            build_command(doc, &prefix, lv_plus_two, cmd, false);
        }
    }
}

fn build_command(
    doc: &mut Document,
    prefix: &[String],
    level: HeadingLevel,
    cmd: &Command,
    is_main: bool,
) {
    let cmd_path = [&prefix[..(prefix.len() - 1)], &[cmd.get_name().to_owned()]].concat();
    let full_cmd_name = cmd_path.join(" ");
    doc.header_code(full_cmd_name.clone(), level);

    if let Some(about) = cmd.get_about() {
        doc.paragraph(about.to_string());
    }

    if cmd.get_subcommands().next().is_some() && !is_main {
        doc.0.push(Event::Start(Tag::Paragraph));
        doc.link(
            format!("> `{full_cmd_name}`'s subcommands"),
            format!("./{}.md", cmd_path.join("_")),
        );
        doc.0.push(Event::End(Tag::Paragraph));
    }

    if let Some(author) = cmd.get_author() {
        doc.paragraph(format!("Author: {}", author));
    }
    if let Some(version) = cmd.get_version() {
        let long_version = if let Some(msg) = cmd.get_long_version() {
            format!(" ({})", msg)
        } else {
            "".into()
        };
        doc.paragraph(format!("Version: {}{}", version, long_version));
    }
    if cmd.get_arguments().next().is_some() {
        doc.paragraph("Arguments:".into());
        doc.0.push(Event::Start(Tag::List(None)));

        for arg in cmd.get_arguments() {
            doc.0.push(Event::Start(Tag::Item));
            doc.0.push(Event::Start(Tag::Paragraph));

            let mut def = String::new();
            if let Some(short) = arg.get_short() {
                def.push('-');
                def.push(short);
                def.push(' ');
            }
            if let Some(long) = arg.get_long() {
                if arg.get_short().is_some() {
                    def.push('/');
                    def.push(' ');
                }
                def.push_str("--");
                def.push_str(long);
                def.push(' ');
            }

            let formmated_value_names = arg
                .get_value_names()
                .unwrap_or(&vec![] as &Vec<Str>)
                .iter()
                .map(|value_name| format!("<{}>", value_name))
                .collect::<Vec<String>>()
                .join(" ");

            def.push_str(&formmated_value_names);

            doc.0.push(Event::Code(def.into()));

            let mut text = String::new();
            if let Some(help) = arg.get_help() {
                if arg.get_short().is_some() || arg.get_long().is_some() {
                    text.push(':');
                }
                text.push(' ');
                text.push_str(help.to_string().as_str());
            }
            if !arg.get_default_values().is_empty() {
                text.push_str(
                    format!(
                        " (default: `{}`)",
                        arg.get_default_values()
                            .iter()
                            .map(|s| s.to_str().unwrap().into())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                    .as_str(),
                );
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

fn recur_gen(
    cmd: &Command,
    path: PathBuf,
    prefix: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
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

fn app_to_md(
    cmd: &Command,
    prefix: &[String],
    level: HeadingLevel,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut document = Document(Vec::new());
    build_page(&mut document, cmd, level, prefix.to_vec());
    let mut result = String::new();

    custom_cmark(document.0.iter(), &mut result)?;
    Ok(result)
}

pub fn generate_command_doc(
    cmd: &Command,
    path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    recur_gen(cmd, path, &[])
}
