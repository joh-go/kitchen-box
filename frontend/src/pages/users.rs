use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use shared_types::User;

#[function_component(UsersPage)]
pub fn users_page() -> Html {
    let users = use_state(Vec::<User>::new);
    let error = use_state(|| None as Option<String>);

    {
        let users = users.clone();
        let error = error.clone();
        use_effect(move || {
            let users = users.clone();
            let error = error.clone();
            spawn_local(async move {
                match api::get_users().await {
                    Ok(list) => users.set(list),
                    Err(e) => error.set(Some(e)),
                }
            });
            || ()
        });
    }

    html!{
        <div>
            <h2 class="text-lg font-semibold mb-2">{ "Manage Users" }</h2>
            { if let Some(e) = &*error { html!{ <p class="text-red-500">{ e }</p> } } else { html!{} } }
            <ul>
                { for (*users).iter().map(|u| html!{ <li class="py-1">{ format!("{} <{}>", u.name, u.email) }</li> }) }
            </ul>
        </div>
    }
}
