/// A crate for defining variables in build.rs and using them in your code.
///
/// # Example
/// ```no_run
/// use build_vars::define_vars;
///
/// define_vars!(
///     (MY_VAR, &str, "Hello, world!"),
///     (MY_VAR_2, usize, 42),
/// );
/// ```
///
/// will result in a `build_vars.rs`:
/// ```
/// pub const MY_VAR: &str = "Hello, world!";
/// pub const MY_VAR_2: usize = 42;
/// ```
///
/// which you can then use in your code:
/// ```ignore
/// include!(concat!(env!("OUT_DIR"), "/build_vars.rs"));
///
/// println!("{}", MY_VAR);
/// println!("{}", MY_VAR_2);
/// ```
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

static FILE: Mutex<Option<File>> = Mutex::new(None);

pub fn create_file(val: Vec<String>) {
    let mut file = FILE.lock().unwrap();
    if file.is_none() {
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("build_vars.rs");
        file.replace(File::create(dest_path).unwrap());
    }

    file.as_mut()
        .unwrap()
        .write_all(val.join("\n").as_bytes())
        .unwrap();
}

/// Define variables in build.rs and use them in your code.
///
/// # Example
/// ```no_run
/// use build_vars::define_vars;
///
/// define_vars!(
///     (MY_VAR, &str, "Hello, world!"),
///     (MY_VAR_2, usize, 42),
/// );
/// ```
#[macro_export]
macro_rules! define_vars {
    ($(($name: ident, $ty: ty, $value: expr)),+ $(,)?) => {
        let mut vars = Vec::new();
        $({
            let mut quotes = "";
            if stringify!($ty) == "&'static str" || stringify!($ty) == "& str" {
                quotes = "\"";
            }

            vars.push(format!(
                "pub const {}: {} = {quotes}{:?}{quotes};",
                stringify!($name),
                stringify!($ty),
                $value
            ));
        })*

        build_vars::create_file(vars);
    };
}
