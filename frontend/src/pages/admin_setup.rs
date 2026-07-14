use yew::prelude::*;
use crate::api;
use crate::i18n::{Language, t};
use crate::language_provider::LanguageState;

#[function_component(AdminSetupPage)]
pub fn admin_setup_page() -> Html {
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let confirm_password = use_state(|| String::new());

    let on_create_admin = {
        let loading = loading.clone();
        let error = error.clone();
        let username = username.clone();
        let password = password.clone();
        let confirm_password = confirm_password.clone();

        Callback::from(move |_| {
            let u = (*username).clone();
            let p = (*password).clone();
            let cp = (*confirm_password).clone();

            if u.trim().is_empty() || p.is_empty() {
                error.set(Some(t("fill_all_fields", lang)));
                return;
            }
            if p != cp {
                error.set(Some(t("passwords_do_not_match", lang)));
                return;
            }

            loading.set(true);
            error.set(None);

            let loading = loading.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match api::create_initial_admin(u, String::new(), p).await {
                    Ok(_) => {
                        if let Some(window) = web_sys::window() {
                            let _ = window.location().set_href("/login");
                        }
                    }
                    Err(e) => {
                        error.set(Some(e));
                        loading.set(false);
                    }
                }
            });
        })
    };

    html! {
        <div class="auth-page">
            <div class="auth-card">
                <div class="auth-logo">
                    <div class="auth-logo-icon">
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M21 4.5C19.9 4.15 18.7 4 17.5 4c-1.95 0-4.05.4-5.5 1.5C10.55 4.4 8.45 4 6.5 4 5.3 4 4.1 4.15 3 4.5v14.65c0 .25.25.5.5.5.1 0 .15-.05.25-.05 1.1-.35 2.3-.5 3.5-.5 1.95 0 4.05.4 5.5 1.5 1.45-1.1 3.55-1.5 5.5-1.5 1.2 0 2.4.15 3.5.5.1.05.15.05.25.05.25 0 .5-.25.5-.5V4.5z"/>
                        </svg>
                    </div>
                    <h1>{t("initial_setup", lang)}</h1>
                    <p>{t("create_admin_desc", lang)}</p>
                </div>

                {if let Some(ref err) = *error {
                    html! {
                        <div class="alert alert-error" style="margin-bottom: 1rem;">
                            <span>{err}</span>
                        </div>
                    }
                } else { html! {} }}

                <form>
                    <div class="form-group">
                        <label class="form-label" for="setup-username">{"Benutzername"}</label>
                        <input id="setup-username" class="form-input" type="text"
                            placeholder={"Benutzername"}
                            value={(*username).clone()}
                            oninput={let s = username.clone(); Callback::from(move |e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                s.set(input.value());
                            })}
                        />
                    </div>
                    <div class="form-group">
                        <label class="form-label" for="setup-password">{t("password", lang)}</label>
                        <input id="setup-password" class="form-input" type="password"
                            placeholder={t("create_strong_password", lang)}
                            value={(*password).clone()}
                            oninput={let s = password.clone(); Callback::from(move |e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                s.set(input.value());
                            })}
                        />
                    </div>
                    <div class="form-group">
                        <label class="form-label" for="setup-confirm">{t("confirm_password", lang)}</label>
                        <input id="setup-confirm" class="form-input" type="password"
                            placeholder={t("confirm_your_password", lang)}
                            value={(*confirm_password).clone()}
                            oninput={let s = confirm_password.clone(); Callback::from(move |e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                s.set(input.value());
                            })}
                        />
                    </div>
                    <button type="button" class="btn btn-primary w-full" onclick={on_create_admin} disabled={*loading}>
                        {if *loading { t("creating_admin", lang) } else { t("create_administrator", lang) }}
                    </button>
                </form>
            </div>
        </div>
    }
}
