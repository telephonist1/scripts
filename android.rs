use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

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

fn detect() -> String {
    let output = Command::new("simple-mtpfs")
        .arg("-l")
        .output()
        .expect("Could not list devices!");
    let output = String::from_utf8_lossy(&output.stdout);
    output.to_string()
}

fn mount() {
    Command::new("simple-mtpfs")
        .args(&["--device", "1", "/home/user/mobile"])
        .spawn()
        .expect("mounting failed");
}

fn unmount() {
    Command::new("fusermount")
        .args(&["-u", "/home/user/mobile"])
        .spawn()
        .expect("unmounting failed");
}

fn get_process(program: &str) -> String {
    let output = Command::new("ps")
        .arg("-fC")
        .arg(program)
        .output()
        .expect("failed to get process");
    let output = String::from_utf8_lossy(&output.stdout);
    output.to_string()
}

fn synchronize(dir: &str, dest_dir: &str) {
    for file in listdir(dir.to_string()) {
        if listdir(dest_dir.to_string()).contains(&file) {
            continue;
        } else {
            let mut path = "/home/user/Music/".to_owned();
            path.push_str(&file);
            let directory = Path::new(&path);
            if directory.is_dir() && listdir(format!("/home/user/Music/{}", file)).len() < 1 {
                continue;
            } else {
                Command::new("cp")
                    .arg("-r")
                    .arg(format!("{}/{}", &dir, &file))
                    .arg(&dest_dir)
                    .spawn()
                    .expect("Failed to copy file");
            }
        }
    }
}

fn main() {
    loop {
        if detect().contains("1")
            && listdir("/home/user/mobile".to_string()).contains(&"Card".to_string()) == false
        {
            mount();
        } else if detect().contains("1") == false
            && listdir("/home/user/mobile".to_string()).contains(&"Card".to_string())
        {
            unmount();
        } else if get_process("youtube-dl").contains("youtube-dl") == false
            && listdir("/home/user/mobile".to_string()).contains(&"Card".to_string())
            && detect().contains("1")
        {
            synchronize("/home/user/Music/", "/home/user/mobile/Card/Music/");
        }
        thread::sleep(Duration::from_secs(10));
    }
}
