use yew::prelude::*;
use shared_types::Recipe;
use crate::components::recipe_list::RecipeList;

#[function_component(Home)]
pub fn home() -> Html {
    let on_edit = Callback::from(|_r: Recipe| {});
    let on_view = Callback::from(|_id: i32| {});
    let on_search = Callback::from(|_value: String| {});
    html! {
        <div>
            <RecipeList on_edit={on_edit} on_view={on_view} refresh={0} search={String::new()} on_search={on_search} />
        </div>
    }
}
