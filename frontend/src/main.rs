use yew::prelude::*;
mod api;
mod components;
mod models;

use components::{recipe_form::RecipeForm, recipe_list::RecipeList};
use models::Recipe;

#[function_component(App)]
fn app() -> Html {
    let editing = use_state(|| None as Option<Recipe>);

    let on_edit = Callback::from({
        let editing = editing.clone();
        move |r: Recipe| {
            editing.set(Some(r));
        }
    });

    let on_saved = Callback::from({
        let editing = editing.clone();
        move |_| {
            editing.set(None);
        }
    });

    html! {
        <div class="container mx-auto p-4">
            <div class="mb-4">
                <h1 class="text-xl font-bold">{ "Recipes" }</h1>
            </div>

            <div>
                <RecipeForm on_saved={on_saved.clone()} editing={(*editing).clone()} />
                <RecipeList on_edit={on_edit.clone()} />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
