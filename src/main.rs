use argh::FromArgs;
use std::{fs, path::Path, process::Command};

#[derive(FromArgs)]
/// Run a test range to detect future bugs
struct Arguments {
    /// bpp executable path
    #[argh(option, short = 'p')]
    path: String,
}

fn compare(only_folder: &str, export_path: String) {
    let path = Path::new(&export_path).join("index.html");
    let export = fs::read_to_string(path).unwrap();

    let correct_path = format!("corrects/{}", only_folder);
    let path = Path::new(&correct_path).join("index.html");
    let correct = fs::read_to_string(path).unwrap();

    assert_eq!(
        export, correct,
        "The result of the {only_folder}.bpp file is not the same as the template file."
    )
}

fn visit_dirs(exe: String, dir: &Path) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(exe.clone(), &path);
            } else {
                let with_extension = path.with_extension("");
                let only_folder = with_extension.file_name().unwrap().to_str().unwrap();
                let export_path = format!("exports/{}", only_folder);
                Command::new(&exe)
                    .arg("export")
                    .arg(path)
                    .arg(&export_path)
                    .output()
                    .unwrap();

                compare(only_folder, export_path);
            }
        }
    }
}

fn main() {
    let exports = Path::new("exports");
    if exports.exists() {
        fs::remove_dir_all(exports).unwrap();
    }

    fs::create_dir(exports).unwrap();

    let arguments: Arguments = argh::from_env();
    visit_dirs(arguments.path, Path::new("examples"));
}
