pub mod fish;
pub mod zsh;

pub fn install(shell: &str) {
    match shell {
        "fish" => fish::install(),
        "zsh" => zsh::install(),
        _ => eprintln!("Unknown shell: {}. Supported: fish, zsh", shell),
    }
}
