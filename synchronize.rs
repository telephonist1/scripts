use std::process::Command;

fn listdir(dir: String) -> Vec<String> {
    let output = Command::new("ls")
        .arg(dir)
        .output()
        .expect("Failed to list directory");
    let output = String::from_utf8_lossy(&output.stdout);
    let mut output: Vec<String> = output.split("\n").map(str::to_string).collect();
    output.pop();
    output
}

fn synchronize(dir: &str, dest_dir: &str) {
    for file in listdir(dir.to_string()) {
        if listdir(dest_dir.to_string()).contains(&file) {
            continue;
        } else {
            println!("Copying {}", &file);
            Command::new("cp")
                .arg(format!("{}/{}", &dir, &file))
                .arg(&dest_dir)
                .spawn()
                .expect("Failed to copy file");
        }
    }
}

fn main() {
    synchronize("/home/shamil/Music", "/home/shamil/mobile/Card/Music");
}
