use clap::Parser;
use std::collections::HashSet;
use std::{fs, path};

/// remove all target
fn main() {
    let mut args = Args::parse();
    if args.removes.is_empty() {
        args.removes = ["target", "node_modules"].map(|it| it.to_string()).to_vec();
    }
    let p = {
        match std::env::current_exe() {
            Err(e) => {
                println!("{}", e);
                return;
            }
            Ok(t) => t.parent().expect("").to_path_buf(),
        }
    };

    if let Err(e) = remove_target(&p, &HashSet::from(["target", "node_modules"])) {
        println!("{}", e);
        return;
    }
}

fn remove_target(dir: &path::PathBuf, target: &HashSet<&str>) -> Result<(), std::io::Error> {
    for it in walkdir::WalkDir::new(dir).into_iter() {
        if let Ok(o) = it {
            let path = o.path();
            let it_path = path.file_name().expect("").to_str().expect("");
            if target.contains(it_path) {
                println!("removing : {:?}", path);
                fs::remove_dir_all(path)?;
                println!("removed : {:?}", path);
            }
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// remove paths / files
    /// default value: ["target","node_modules"]
    /// sample:
    /// target, same as */target/* and */target file
    /// */target/*, all target folder, not include target file
    /// */target, all target file, not include target folder
    #[arg(short, long)]
    removes: Vec<String>,

    /// search the path, default is exe path
    #[arg(short, long, default_value_t = String::default())]
    path: String,
}
