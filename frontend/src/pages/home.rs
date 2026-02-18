use yew::prelude::*;
use shared_types::Recipe;
use crate::components::recipe_list::RecipeList;

#[function_component(Home)]
pub fn home() -> Html {
    let on_edit = Callback::from(|_r: Recipe| {});
    html! {
        <div>
            <RecipeList on_edit={on_edit} refresh={0} search={String::new()} />
        </div>
    }
}
