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
        }
        thread::sleep(Duration::from_secs(10));
    }
}
