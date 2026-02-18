use wasm_bindgen::JsCast;
use yew::prelude::*;
mod api;
mod components;

use components::{recipe_form::RecipeForm, recipe_list::RecipeList};
use shared_types::Recipe;

#[function_component(App)]
fn app() -> Html {
    let editing = use_state(|| None as Option<Recipe>);
    let refresh = use_state(|| 0i32);
    let search = use_state(|| String::new());
    let dark = use_state(|| false);

    let on_edit = Callback::from({
        let editing = editing.clone();
        move |r: Recipe| {
            editing.set(Some(r));
        }
    });

    let on_saved = Callback::from({
        let editing = editing.clone();
        let refresh = refresh.clone();
        move |_| {
            editing.set(None);
            refresh.set(*refresh + 1);
        }
    });

    let on_search = {
        let search = search.clone();
        Callback::from(move |q: String| {
            search.set(q);
        })
    };

    let toggle_dark = {
        let dark = dark.clone();
        Callback::from(move |_| {
            let new = !*dark;
            dark.set(new);
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if let Some(el) = doc.document_element() {
                    // modify class attribute to add/remove `dark` because class_list helpers
                    // may not be available in all web-sys bindings.
                    if let Some(mut classes) = el.get_attribute("class") {
                        if new {
                            if !classes.split_whitespace().any(|s| s == "dark") {
                                classes.push_str(" dark");
                                let _ = el.set_attribute("class", &classes);
                            }
                        } else {
                            let filtered = classes
                                .split_whitespace()
                                .filter(|s| *s != "dark")
                                .collect::<Vec<_>>()
                                .join(" ");
                            let _ = el.set_attribute("class", &filtered);
                        }
                    } else if new {
                        let _ = el.set_attribute("class", "dark");
                    }
                }
            }
        })
    };

    html! {
        <div class="min-h-screen bg-gray-50 dark:bg-gray-900">
            <header class="bg-white dark:bg-gray-800 shadow">
                <div class="container mx-auto px-4 py-4 flex items-center justify-between">
                    <h1 class="text-2xl font-bold text-gray-900 dark:text-white">{ "My Recipes" }</h1>
                    <nav class="text-sm text-gray-600 dark:text-gray-300">{ "Personal cookbook" }</nav>
                </div>
            </header>

            <main class="container mx-auto p-4">
                <div class="mb-4 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
                    <div class="flex items-center gap-4">
                        <h2 class="text-xl font-semibold text-gray-800 dark:text-gray-100">{ "Recipes" }</h2>
                        <div class="hidden sm:block">
                            <input
                                placeholder="Search recipes"
                                value={(*search).clone()}
                                oninput={Callback::from({
                                    let on_search = on_search.clone();
                                    move |e: InputEvent| {
                                        let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                        on_search.emit(input.value());
                                    }
                                })}
                                class="border rounded px-3 py-2"
                            />
                        </div>
                    </div>
                    <div class="flex items-center gap-3">
                        <button onclick={toggle_dark.clone()} class="bg-gray-200 dark:bg-gray-700 px-3 py-2 rounded">{ if *dark { "Light" } else { "Dark" } }</button>
                    </div>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                    <div class="lg:col-span-1">
                        <RecipeForm on_saved={on_saved.clone()} editing={(*editing).clone()} />
                    </div>
                    <div class="lg:col-span-2">
                        <RecipeList on_edit={on_edit.clone()} refresh={*refresh} search={(*search).clone()} />
                    </div>
                </div>
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
