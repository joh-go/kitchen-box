use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use shared_types::Recipe;
use crate::components::recipe_form::RecipeForm;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
    pub on_saved: Callback<i32>,
}

#[function_component(EditRecipe)]
pub fn edit_recipe(props: &Props) -> Html {
    let recipe = use_state(|| None as Option<Recipe>);
    let error = use_state(|| None as Option<String>);
    let refresh = use_state(|| 0u32);
    let id = props.id;

    {
        let recipe = recipe.clone();
        let error = error.clone();
        let refresh_val = *refresh;
        use_effect_with((id, refresh_val), move |(id, _)| {
            let recipe = recipe.clone();
            let error = error.clone();
            let id = *id;
            spawn_local(async move {
                match api::get_recipe(id).await {
                    Ok(r) => recipe.set(Some(r)),
                    Err(e) => error.set(Some(e)),
                }
            });
            || ()
        });
    }

    let on_saved = {
        let on_saved = props.on_saved.clone();
        let id = props.id;
        Callback::from(move |_| {
            on_saved.emit(id);
        })
    };

    let on_refresh = {
        let refresh = refresh.clone();
        Callback::from(move |_| {
            refresh.set(*refresh + 1);
        })
    };

    html!{
        <div>
            { if let Some(r) = &*recipe {
                html!{ <RecipeForm on_saved={on_saved} editing={Some(r.clone())} on_refresh={on_refresh} /> }
            } else if let Some(e) = &*error {
                html!{ <p class="text-red-500">{ e }</p> }
            } else {
                html!{ <p>{ "Loading..." }</p> }
            }}
        </div>
    }
}
