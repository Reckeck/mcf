mod generate_presets;

fn main() {
    println!("cargo::rerun-if-changed=build/main.rs");
    generate_presets::generate_presets().unwrap();
}
