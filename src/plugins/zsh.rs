pub fn install() {
    println!("Add this to ~/.zshrc:");
    println!("_smart_complete() {{");
    println!("    compadd $(smart-shell-complete complete \"${{(Q)words[CURRENT]}}\")");
    println!("}}");
}
