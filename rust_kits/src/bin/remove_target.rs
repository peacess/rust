use std::{fs, path};
/// remove all target
fn main() {
    let p = {
        match std::env::current_exe() {
            Err(e) => {
                println!("{}", e);
                return;
            }
            Ok(t) => t.parent().expect("").to_path_buf(),
        }
    };

    if let Err(e) = remove_target(&p, "target") {
        println!("{}", e);
        return;
    }
}

fn remove_target(dir: &path::PathBuf, target: &str) -> Result<(), std::io::Error> {
    for it in walkdir::WalkDir::new(dir).into_iter().filter_map(|it| {
        if let Ok(o) = it {
            if target == o.path().file_name().expect("").to_str().expect("") {
                Some(o)
            } else {
                None
            }
        } else {
            None
        }
    }) {
        let path = it.path();
        println!("path: {:?}", path);
        fs::remove_dir_all(path)?;
    }
    Ok(())
}
