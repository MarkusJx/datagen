#[cfg(feature = "sqlite")]
mod sqlite {
    use build_vars::define_vars;
    use regex::Regex;

    pub fn parse_flags() {
        let sqlite_flags = option_env!("LIBSQLITE3_FLAGS");
        let mut max_vars: u64 = 32_766;

        if let Some(flags) = sqlite_flags {
            if let Some(matches) = Regex::new("SQLITE_MAX_VARIABLE_NUMBER=(\\d+)")
                .unwrap()
                .captures(flags)
            {
                max_vars = matches.get(1).unwrap().as_str().parse().unwrap();
            }
        }

        define_vars!((SQLITE_MAX_VARIABLE_NUMBER, usize, max_vars));
    }
}

fn main() {
    #[cfg(feature = "sqlite")]
    sqlite::parse_flags();
}
