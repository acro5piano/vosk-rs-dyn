fn main() {
    // provide cargo with the path to the vosk libraries.
    println!("cargo:rustc-link-search=./lib/vosk");
}
