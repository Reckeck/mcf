mod generate_profiles;

fn main() {
    println!("cargo::rerun-if-changed=build/main.rs");
    generate_profiles::generate_presets().unwrap();
}
