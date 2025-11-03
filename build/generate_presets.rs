use std::{
    fs, io,
    path::{Path, PathBuf},
};

use serde_json::{Map, Value};

static PRESETS_PATH: &'static str = "presets";
static PRESETS_OUT: &'static str = "./src/generate/presets.rs";

pub fn generate_presets() -> io::Result<()> {
    println!("cargo::rerun-if-changed={:?}", Path::new(PRESETS_OUT));

    let presets_out = Path::new(PRESETS_OUT).parent().unwrap();
    fs::create_dir_all(presets_out).unwrap();

    let mut generated = String::from(
        r#"/// THIS FILE IS GENERATED IN `build/generate_presets.rs.
/// DON't MODIFIED THIS FILE AND DON'T PUSH IN GIT.
use std::collections::HashMap;

pub fn get_profiles() -> HashMap<String, Profile> {
    let mut map: HashMap<String, Profile> = HashMap::new();

"#,
    );

    let root_dir = PathBuf::from(PRESETS_PATH);
    let mut dirs_stack: Vec<PathBuf> = vec![root_dir.clone()];

    while dirs_stack.len() > 0 {
        let last_dir = dirs_stack.last().unwrap().clone();
        dirs_stack.pop();

        for entry in fs::read_dir(&last_dir).unwrap() {
            let entry = entry.unwrap();

            if entry.metadata().unwrap().is_dir() {
                dirs_stack.push(entry.path().to_path_buf());
                continue;
            }

            let entry_name = last_dir.file_name().unwrap().to_str().unwrap().to_string();
            let profile_str = fs::read_to_string(entry.path()).unwrap();

            let json: Map<String, Value> = serde_json::from_str(&profile_str).unwrap();

            let description = json.get("description").unwrap().as_str().unwrap();
            let frame = json.get("frame").unwrap().as_object().unwrap();
            let frame_rate = json.get("frame_rate").unwrap().as_object().unwrap();
            let sample_aspect = json.get("sample_aspect").unwrap().as_object().unwrap();
            let display_aspect = json.get("display_aspect").unwrap().as_object().unwrap();
            let explicit_type = json.get("explicit_type").unwrap().as_str().unwrap();
            let render_type = json.get("render_type").unwrap().as_str().unwrap();
            let color_space = json.get("color_space").unwrap().as_str().unwrap();

            generated.push_str("    {\n");
            generated.push_str("        let profile = ProfileBuilder::new()\n");

            generated.push_str(&format!(
                r#"            .set_frame(geometry::Frame {{
                width: {} as f32,
                height: {} as f32,
            }})"#,
                frame.get("width").unwrap().as_i64().unwrap(),
                frame.get("height").unwrap().as_i64().unwrap(),
            ));

            generated.push_str(&format!(
                r#"
            .set_frame_rate(Ratio {{
                number: {} as f32,
                denominator: {} as f32,
            }})"#,
                frame_rate.get("number").unwrap().as_i64().unwrap(),
                frame_rate.get("denominator").unwrap().as_i64().unwrap(),
            ));

            generated.push_str(&format!(
                r#"
            .set_sample_aspect(Ratio {{
                number: {} as f32,
                denominator: {} as f32,
            }})"#,
                sample_aspect.get("number").unwrap().as_i64().unwrap(),
                sample_aspect.get("denominator").unwrap().as_i64().unwrap(),
            ));

            generated.push_str(&format!(
                r#"
            .set_display_aspect(Ratio {{
                number: {} as f32,
                denominator: {} as f32,
            }})"#,
                display_aspect.get("number").unwrap().as_i64().unwrap(),
                display_aspect.get("denominator").unwrap().as_i64().unwrap(),
            ));

            generated.push_str(&format!(
                r#"
            .set_color_space(ColorSpace::{})"#,
                color_space,
            ));

            generated.push_str(&format!(
                r#"
            .set_explicit_type(ExplicitType::{})"#,
                explicit_type,
            ));

            generated.push_str(&format!(
                r#"
            .set_render_type(RenderType::{})"#,
                render_type,
            ));

            generated.push_str("\n            .build();");
            generated.push_str(&format!(
                "\n        map.insert(\"{entry_name} - {description}\".to_string(), profile);\n"
            ));
            generated.push_str("    }\n\n");
        }
    }

    generated.push_str("    map\n}");

    fs::write(PRESETS_OUT, generated).unwrap();

    Ok(())
}
