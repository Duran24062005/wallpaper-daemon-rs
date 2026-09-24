#![allow(dead_code, unused)]
use std::process::Command;


pub fn review_entire() -> std::process::Output {
        let output2 = Command::new("gsettings")
                .arg("get")
                .arg("org.gnome.desktop.background")
                .arg("picture-uri")
                .output()
                .expect("Failed to execute command");

        output2
}

// Review function
pub fn review() -> String {
    let output = Command::new("gsettings")
        .arg("get")
        .arg("org.gnome.desktop.background")
        .arg("picture-uri")
        .output()
        .expect("Failed to execute command");

    String::from_utf8(output.stdout)
        .expect("Failed to convert output to String")
}

// facade main
pub fn main_fn() {
        
        // let review = code_templates::review::review();

        let review = review();
        let review_entire = review_entire();

        println!("{:?}", review);
        println!("{}", String::from_utf8(review_entire.stdout).expect("Failed to convert output to String"));
}