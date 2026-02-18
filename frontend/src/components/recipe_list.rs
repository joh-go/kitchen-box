use crate::api;
use shared_types::Recipe;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_edit: Callback<Recipe>,
    pub refresh: i32,
}

#[function_component(RecipeList)]
pub fn recipe_list(props: &Props) -> Html {
    let recipes = use_state(|| Vec::<Recipe>::new());
    let error = use_state(|| None::<String>);
    let search = use_state(|| String::new());

    // accept refresh as external prop if passed in (backwards compatible)
    // We'll try to read a field named `refresh` via `js_sys` props are typed, so update Props accordingly in parent.

    {
        let recipes = recipes.clone();
        let error = error.clone();
        let refresh_dep = props.refresh;
        use_effect_with(
            refresh_dep,
            move |_refresh| {
                let recipes = recipes.clone();
                let error = error.clone();
                spawn_local(async move {
                    match api::get_recipes().await {
                        Ok(list) => recipes.set(list),
                        Err(e) => error.set(Some(e)),
                    }
                });
                || ()
            },
        );
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

    let search_input = search.clone();

    html! {
        <div>
            <h2 class="text-2xl font-bold mb-2">{ "Recipes" }</h2>
            {
                if let Some(e) = &*error {
                    html!{ <p class="text-red-500">{ e }</p> }
                } else { html!{} }
            }
            <div class="mb-3">
                <input
                    placeholder="Search recipes"
                    value={(*search).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        search_input.set(input.value());
                    })}
                    class="border px-2 py-1 w-full mb-2"
                />
            </div>

            <ul>
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
                        <li class="mb-2">
                            <div class="font-semibold">{ &r.title }</div>
                            <div class="text-sm text-gray-600">{ r.short_description.clone().unwrap_or_default() }</div>
                            <button class="mr-2 bg-yellow-400 px-2 py-1" onclick={props.on_edit.reform(move |_| r_clone.clone())}>{"Edit"}</button>
                            <button class="bg-red-500 text-white px-2 py-1" onclick={on_delete.reform(move |_| id)}>{"Delete"}</button>
                        </li>
                    }
                }) }
            </ul>
        </div>
    }
}
