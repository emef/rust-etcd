#[cfg(not(feature = "serde_macros"))]
mod inner {
    extern crate serde_codegen;
    extern crate syntex;

    use std::env;
    use std::path::Path;

    const MODULES: &'static[&'static str] = &[
        "error",
        "keys",
        "stats",
        "version",
    ];

    pub fn main() {
        let out_dir = env::var_os("OUT_DIR").unwrap();

        for module in MODULES.iter() {
            let src = format!("src/{}_gen.rs", module);
            let src_path = Path::new(&src);
            let dst = format!("{}.rs", module);
            let dst_path = Path::new(&out_dir).join(&dst);

            let mut registry = syntex::Registry::new();

            serde_codegen::register(&mut registry);
            registry.expand("", &src_path, &dst_path).unwrap();
        }
    }
}

#[cfg(feature = "serde_macros")]
mod inner {
    pub fn main() {}
}

fn main() {
    inner::main();
}
