use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn create_file(val: Vec<String>) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_vars.rs");
    let mut f = File::create(dest_path).unwrap();

    f.write_all(val.join("\n").as_bytes()).unwrap();
}

#[macro_export]
macro_rules! define_vars {
    ($(($name: ident, $ty: ty, $value: expr)),+ $(,)?) => {
        let mut vars = Vec::new();
        $(
            let mut quotes = "";
            if stringify!($ty) == "&'static str" {
                quotes = "\"";
            }

            vars.push(format!(
                "pub const {}: {} = {quotes}{}{quotes};",
                stringify!($name),
                stringify!($ty),
                $value
            ));
        )*

        build_vars::create_file(vars);
    };
}
