use yew::prelude::*;
use shared_types::Recipe;
use crate::components::recipe_list::RecipeList;

#[function_component(Home)]
pub fn home() -> Html {
    let on_edit = Callback::from(|_recipe: Recipe| {});
    let on_view = Callback::from(|_id: i32| {});
    let on_add = Callback::from(|_| {});
    let on_search = Callback::from(|_value: String| {});
    html! {
        <RecipeList on_edit={on_edit} on_view={on_view} on_add={on_add} refresh={0} search={String::new()} on_search={on_search} />
    }
}
