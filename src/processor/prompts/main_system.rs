use askama::Template;
use graph_flow::SerializableMessage;

#[derive(Template)] // this will generate the code...
#[template(path = "main_system_prompt.md")] // using the template in this path, relative
// to the `templates` dir in the crate root
pub struct MainSystemPromptTemplate {
    pub history: Vec<SerializableMessage>,
    pub user_input: String,
    pub context: String,
}
