use crate::api;
use shared_types::Recipe;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_edit: Callback<Recipe>,
    pub refresh: i32,
    pub search: String,
}

#[function_component(RecipeList)]
pub fn recipe_list(props: &Props) -> Html {
    let recipes = use_state(|| Vec::<Recipe>::new());
    let error = use_state(|| None::<String>);
    // use search passed from parent for centralized header search
    let search = props.search.clone();

    // accept refresh as external prop if passed in (backwards compatible)
    // We'll try to read a field named `refresh` via `js_sys` props are typed, so update Props accordingly in parent.

    {
        let recipes = recipes.clone();
        let error = error.clone();
        let refresh_dep = props.refresh;
        use_effect_with(refresh_dep, move |_refresh| {
            let recipes = recipes.clone();
            let error = error.clone();
            spawn_local(async move {
                match api::get_recipes().await {
                    Ok(list) => recipes.set(list),
                    Err(e) => error.set(Some(e)),
                }
            });
            || ()
        });
    }

    let on_delete = {
        let recipes = recipes.clone();
        Callback::from(move |id: i32| {
            let recipes = recipes.clone();
            spawn_local(async move {
                if api::delete_recipe(id).await.is_ok() {
                    recipes.set(
                        recipes
                            .iter()
                            .cloned()
                            .filter(|r| r.id != Some(id))
                            .collect(),
                    );
                }
            });
        })
    };

    html! {
        <div>
            <h2 class="text-2xl font-bold mb-2">{ "Recipes" }</h2>
            {
                if let Some(e) = &*error {
                    html!{ <p class="text-red-500">{ e }</p> }
                } else { html!{} }
            }
            // search is rendered in header; keep a small hint here for mobile
            <div class="mb-3 block lg:hidden">
                <input
                    placeholder="Search recipes"
                    value={search.clone()}
                    readonly=true
                    class="border px-2 py-1 w-full mb-2 bg-gray-100"
                />
            </div>

            <ul class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                { for (*recipes).iter().filter(|r| {
                    if search.is_empty() { true }
                    else {
                        let q = search.to_lowercase();
                        r.title.to_lowercase().contains(&q) || r.short_description.clone().unwrap_or_default().to_lowercase().contains(&q)
                    }
                }).map(|r| {
                    let id = r.id.unwrap_or_default();
                    let r_clone = r.clone();
                    html!{
                        <li class="bg-white dark:bg-gray-800 shadow rounded p-4 transform hover:-translate-y-1 hover:shadow-lg transition">
                            <div class="flex justify-between items-start">
                                <div>
                                    <div class="font-semibold text-lg text-gray-900 dark:text-gray-100">{ &r.title }</div>
                                    <div class="text-sm text-gray-600 dark:text-gray-300">{ r.short_description.clone().unwrap_or_default() }</div>
                                </div>
                                <div class="text-sm text-gray-500 flex items-center gap-2">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-gray-400" viewBox="0 0 20 20" fill="currentColor"><path d="M10 2a8 8 0 100 16 8 8 0 000-16zM9 7h2v5H9V7zM9 13h2v2H9v-2z"/></svg>
                                    { r.prep_minutes.map(|m| format!("{}m", m)).unwrap_or_default() }
                                </div>
                            </div>

                            <div class="mt-3 flex gap-2">
                                <button class="flex-1 bg-yellow-400 hover:bg-yellow-500 text-black px-3 py-2 rounded flex items-center justify-center gap-2" onclick={props.on_edit.reform(move |_| r_clone.clone())}>
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor"><path d="M17.414 2.586a2 2 0 010 2.828l-9.193 9.193-3.536.707.707-3.536L14.586 2.586a2 2 0 012.828 0z"/></svg>
                                    {"Edit"}
                                </button>
                                <button class="flex-1 bg-red-600 hover:bg-red-700 text-white px-3 py-2 rounded flex items-center justify-center gap-2" onclick={on_delete.reform(move |_| id)}>
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M6 2a1 1 0 00-1 1v1H3a1 1 0 000 2h14a1 1 0 100-2h-2V3a1 1 0 00-1-1H6zm2 6a1 1 0 012 0v6a1 1 0 11-2 0V8zm4 0a1 1 0 10-2 0v6a1 1 0 102 0V8z" clip-rule="evenodd"/></svg>
                                    {"Delete"}
                                </button>
                            </div>
                        </li>
                    }
                }) }
            </ul>
        </div>
    }
}
