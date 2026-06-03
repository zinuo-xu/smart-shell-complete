pub fn install() {
    println!("Add this to ~/.config/fish/config.fish:");
    println!("function _smart_complete");
    println!("    smart-shell-complete complete (commandline -ct)");
    println!("end");
}
