use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use shared_types::Recipe;
use crate::components::recipe_form::RecipeForm;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component(EditRecipe)]
pub fn edit_recipe(props: &Props) -> Html {
    let recipe = use_state(|| None as Option<Recipe>);
    let error = use_state(|| None as Option<String>);
    let id = props.id;

    {
        let recipe = recipe.clone();
        let error = error.clone();
        use_effect_with(id, move |&id| {
            let recipe = recipe.clone();
            let error = error.clone();
            spawn_local(async move {
                match api::get_recipe(id).await {
                    Ok(r) => recipe.set(Some(r)),
                    Err(e) => error.set(Some(e)),
                }
            });
            || ()
        });
    }

    html!{
        <div>
            { if let Some(r) = &*recipe {
                html!{ <RecipeForm on_saved={Callback::from(|_|{})} editing={Some(r.clone())} /> }
            } else if let Some(e) = &*error {
                html!{ <p class="text-red-500">{ e }</p> }
            } else {
                html!{ <p>{ "Loading..." }</p> }
            }}
        </div>
    }
}
