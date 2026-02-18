use yew::prelude::*;
use crate::components::recipe_form::RecipeForm;

#[function_component(AddRecipe)]
pub fn add_recipe() -> Html {
    let on_saved = Callback::from(|_: ()| {
        // navigation handled by parent or can use Navigator to go home
    });

    html!{
        <div>
            <RecipeForm on_saved={on_saved} editing={None} />
        </div>
    }
}
