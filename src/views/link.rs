use loco_rs::prelude::*;

use crate::models::_entities::links;

/// Render a list view of `links`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<links::Model>) -> Result<Response> {
    format::render().view(v, "link/list.html", data!({"items": items}))
}

/// Render a single `link` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &links::Model) -> Result<Response> {
    format::render().view(v, "link/show.html", data!({"item": item}))
}

/// Render a `link` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "link/create.html", data!({}))
}

/// Render a `link` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &links::Model) -> Result<Response> {
    format::render().view(v, "link/edit.html", data!({"item": item}))
}
