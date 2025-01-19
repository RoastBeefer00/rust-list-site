use super::{
    index::IndexTemplate, list::List, list_group::ListGroup, list_item::ListItem,
    list_preview::ListPreview, ListItemEdit, ListItemText,
};
use askama_axum::Template;
use axum::response::{Html, IntoResponse, Response};

// Define a macro called 'impl_into_response_for_template'
macro_rules! impl_into_response_for_template {
    // Pattern matcher that accepts a comma-separated list of types
    // $t:ty means it matches any type
    // * means it can match zero or more repetitions
    ($($t:ty),*) => {
        // Generate code for each matched type
        // The $( )* is like a loop that generates code for each matched type
        $(
            // This is the actual implementation that will be generated for each type
            impl IntoResponse for $t {
                fn into_response(self) -> Response {
                    Html(self.render().unwrap()).into_response()
                }
            }
        )*
    };
}

// Using the macro
impl_into_response_for_template!(
    IndexTemplate,
    List,
    ListItem,
    ListGroup,
    ListPreview,
    ListItemEdit,
    ListItemText
);
