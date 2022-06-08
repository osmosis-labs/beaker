pub trait OpResponseDisplay {
    fn headline() -> &'static str;
    fn attrs(&self) -> Vec<String>;
    fn display_format(&self) -> String {
        [
            vec!["", format!("  {}", Self::headline()).as_str(), "    +"],
            self.attrs().iter().map(|s| s.as_str()).collect(),
            vec![""],
        ]
        .concat()
        .join("\n")
    }
    fn log(&self) {
        println!("{}", self.display_format())
    }
}

#[macro_export]
macro_rules! attrs_format {
    ($container:ident | $attr:ident) => {
        vec![format!("    └── {}: {}", stringify!($attr), $container.$attr)]
    };
    ($container:ident | $fst:ident, $($attr:ident),+) => {
        vec![
           vec![format!("    ├── {}: {}", stringify!($fst), $container.$fst)],
           attrs_format!($container | $($attr),+ )
        ].concat()
    };
}

#[macro_export]
macro_rules! vars_format {
    ($attr:ident) => {
        vec![format!("    └── {}: {}", stringify!($attr), $attr), "".to_string()]
    };
    ($fst:ident, $($attr:ident),+) => {
        vec![
           vec![format!("    ├── {}: {}", stringify!($fst), $fst)],
           vars_format!($($attr),+ )
        ].concat()
    };
    ($headline:expr, $($attr:ident),+) => {
        vec![
           vec!["".to_string(), format!("  {}", $headline), "    +".to_string()],
           vars_format!($($attr),+ )
        ].concat()
    };
}
