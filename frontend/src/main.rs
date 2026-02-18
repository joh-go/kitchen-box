use yew::prelude::*;
mod models;
mod api;
mod components;

use components::{recipe_form::RecipeForm, recipe_list::RecipeList};
use models::Recipe;

#[function_component(App)]
fn app() -> Html {
    let tab = use_state(|| "users".to_string());
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
                <button onclick={let tab = tab.clone(); Callback::from(move |_| tab.set("users".to_string()))} class="mr-2 px-3 py-1 bg-gray-200">{ "Users" }</button>
                <button onclick={let tab = tab.clone(); Callback::from(move |_| tab.set("recipes".to_string()))} class="px-3 py-1 bg-gray-200">{ "Recipes" }</button>
            </div>

            { if *tab == "users" { html!{ <h1>{ "User Management (existing)" }</h1> } } else { html!{
                <div>
                    <RecipeForm on_saved={on_saved.clone()} editing={(*editing).clone()} />
                    <RecipeList on_edit={on_edit.clone()} />
                </div>
            } } }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
