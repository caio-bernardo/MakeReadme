use handlebars::Handlebars;
use rust_embed::RustEmbed;

mod repodata;

const BLUEPRINT_FILE: &str = "blueprint.md";
const OUTPUT_FILE: &str = "README.md";

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;


pub fn run() {
    let repo_info = repodata::RepoData::ask_questions();

    let mut hbs = Handlebars::new();

    hbs.register_embed_templates::<Assets>().expect(&format!("Failed to register {} to templates.", BLUEPRINT_FILE));

    let mut output_file = std::fs::File::create(OUTPUT_FILE).expect(&format!("Failed to create {}", OUTPUT_FILE));
    hbs.render_to_write(BLUEPRINT_FILE, &repo_info, &mut output_file).unwrap();
}
