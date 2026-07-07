use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use crate::i18n::{Language, t};
use crate::language_provider::LanguageState;
use shared_types::User;

#[function_component(UsersPage)]
pub fn users_page() -> Html {
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

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
        <div class="card page-enter">
            <div class="card-body">
                <h2 class="section-title mb-4">{ t("manage_users", lang) }</h2>
                { if let Some(e) = &*error { html!{ <div class="alert alert-error"><div class="alert-content">{ e }</div></div> } } else { html!{} } }
                <div class="flex flex-col gap-2">
                    { for (*users).iter().map(|u| html!{ <div class="flex items-center gap-3 text-sm text-muted">{ format!("{} <{}>", u.name, u.email) }</div> }) }
                </div>
            </div>
        </div>
    }
}
