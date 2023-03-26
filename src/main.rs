use handlebars::Handlebars;
use inquire::Text;
use rust_embed::RustEmbed;
use serde::{Serialize, Deserialize};

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

#[derive(Serialize, Deserialize)]
struct Data {
    project_title: String,
    project_description: String,
    github_username: String,
    repo_name: String,
    license: String,
}

fn make_questions() -> Data {
    Data {
        project_title: Text::new("Project title:").prompt().unwrap(),
        project_description: Text::new("Project description:").prompt().unwrap(),
        github_username: Text::new("Github username:").prompt().unwrap(),
        repo_name: Text::new("Repo name:").prompt().unwrap(),
        license: Text::new("License:").prompt().unwrap(),
    }
}

fn main() {
    let data = make_questions();

    let mut  hbs = Handlebars::new();
    hbs.register_embed_templates::<Assets>().unwrap();

    let mut output_file = std::fs::File::create("README.md").unwrap();
    hbs.render_to_write("blueprint.md", &data, &mut output_file).unwrap();
}
