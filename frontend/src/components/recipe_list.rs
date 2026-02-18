use crate::api;
use crate::models::Recipe;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_edit: Callback<Recipe>,
}

#[function_component(RecipeList)]
pub fn recipe_list(props: &Props) -> Html {
    let recipes = use_state(|| Vec::<Recipe>::new());
    let error = use_state(|| None::<String>);

    {
        let recipes = recipes.clone();
        let error = error.clone();
        let loaded = use_state(|| false);

        use_effect(move || {
            let recipes = recipes.clone();
            let error = error.clone();
            let loaded = loaded.clone();
            if !*loaded {
                spawn_local(async move {
                    match api::get_recipes().await {
                        Ok(list) => recipes.set(list),
                        Err(e) => error.set(Some(e)),
                    }
                    loaded.set(true);
                });
            }
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
            <ul>
                { for (*recipes).iter().map(|r| {
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
