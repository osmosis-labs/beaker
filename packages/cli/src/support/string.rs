pub fn trim_indent(s: &str) -> String {
    let it = s.split('\n');
    let first = it.clone().find(|s| !s.is_empty());
    if let Some(f) = first {
        let space_count = f.chars().take_while(|c| c == &' ').count();
        let pat = " ".repeat(space_count);

        it.map(|s| s.trim_start_matches(pat.as_str()))
            .skip_while(|s| s.is_empty())
            .collect::<Vec<&str>>()
            .join("\n")
    } else {
        s.to_string()
    }
}
