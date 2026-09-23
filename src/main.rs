mod wallpapers;

fn main() {

        // let output2 = Command::new("gsettings")
        //     .arg("get")
        //     .arg("org.gnome.desktop.background")
        //     .arg("picture-uri")
        //     .output()
        //     .expect("Failed to execute command");

        let wallpaper = wallpapers::get_current_wallpaper().expect("Failed to get current path");



        println!("{wallpaper}");
}