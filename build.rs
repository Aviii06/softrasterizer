fn main() {
    // For macos: download sdl2 dmg from github releases and then copy the framework to /Library/Frameworks
    println!("cargo:rustc-link-search=framework=/Library/Frameworks")
}