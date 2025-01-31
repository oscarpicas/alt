use crate::def_file;
use crate::shim;
use std::env;
use std::path::*;
use std::process;

pub fn run(command: &str, version: &str, bin: &str) {
    let bin_path = PathBuf::from(bin);

    if !bin_path.exists() {
        println!("File not found: {}", bin);
        process::exit(1);
    }

    let mut defs = def_file::load();
    {
        let def = defs
            .entry(command.to_string())
            .or_insert_with(def_file::CommandVersions::new);
        def.insert(version.to_owned(), PathBuf::from(bin));
    }

    def_file::save(&defs).expect("failed to save defs file");

    shim::make_shim(command, env::current_exe().unwrap().as_path())
        .unwrap_or_else(|err| panic!("failed to create shim for {}: {}", command, err));
}
