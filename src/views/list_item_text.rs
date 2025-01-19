use askama_axum::Template;

#[derive(Template)]
#[template(path = "list_item_text.html")]
pub struct ListItemText {
    pub complete: bool,
    pub text: String,
}
