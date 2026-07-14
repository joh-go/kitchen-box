use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::api;
use crate::i18n::t;
use crate::language_provider::LanguageState;
use shared_types::User;

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let state = use_state(|| RegisterState::default());
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(crate::i18n::Language::English);

    let onsubmit = {
        let state = state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*state).username.clone();
            let password = (*state).password.clone();
            let confirm_password = (*state).confirm_password.clone();

            if username.trim().is_empty() || password.is_empty() {
                let mut s = (*state).clone();
                s.error = Some("Bitte füllen Sie alle Felder aus.".to_string());
                state.set(s);
                return;
            }

            if password != confirm_password {
                let mut s = (*state).clone();
                s.error = Some(t("passwords_do_not_match", lang));
                state.set(s);
                return;
            }

            let mut loading_state = (*state).clone();
            loading_state.loading = true;
            loading_state.error = None;
            state.set(loading_state);

            let s2 = state.clone();
            let u = username.clone();
            let p = password.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let user = User {
                    id: None,
                    name: u.clone(),
                    email: String::new(),
                    password: Some(p.clone()),
                };
                match api::create_user(&user).await {
                    Ok(_) => {
                        web_sys::window().unwrap().location().set_href("/login").unwrap();
                    }
                    Err(e) => {
                        let mut err_state = (*s2).clone();
                        err_state.loading = false;
                        err_state.error = Some(e);
                        s2.set(err_state);
                    }
                }
            });
        })
    };

    let oninput = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            let name = input.get_attribute("name").unwrap_or_default();
            let mut s = (*state).clone();
            match name.as_str() {
                "username" => s.username = value,
                "password" => s.password = value,
                "confirm_password" => s.confirm_password = value,
                _ => {}
            }
            state.set(s);
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
                    <h1>{t("app_name", lang)}</h1>
                    <p>{t("create_administrator_account", lang)}</p>
                </div>

                {if let Some(ref err) = (*state).error {
                    html! {
                        <div class="alert alert-error">
                            <span>{err}</span>
                        </div>
                    }
                } else { html! {} }}

                <form onsubmit={onsubmit}>
                    <div class="form-group">
                        <label class="form-label" for="reg-username">{"Benutzername"}</label>
                        <input id="reg-username" name="username" class="form-input" type="text"
                            placeholder={"Benutzername wählen"}
                            value={(*state).username.clone()}
                            oninput={oninput.clone()}
                        />
                    </div>
                    <div class="form-group">
                        <label class="form-label" for="reg-password">{t("password", lang)}</label>
                        <input id="reg-password" name="password" class="form-input" type="password"
                            placeholder={t("password", lang)}
                            value={(*state).password.clone()}
                            oninput={oninput.clone()}
                        />
                    </div>
                    <div class="form-group">
                        <label class="form-label" for="reg-confirm">{t("confirm_password", lang)}</label>
                        <input id="reg-confirm" name="confirm_password" class="form-input" type="password"
                            placeholder={t("confirm_password", lang)}
                            value={(*state).confirm_password.clone()}
                            oninput={oninput}
                        />
                    </div>
                    <button type="submit" class="btn btn-primary w-full" disabled={(*state).loading}>
                        {if (*state).loading { t("saving", lang) } else { t("register_button", lang) }}
                    </button>
                </form>

                <div class="auth-footer">
                    {t("have_account", lang) + " "}
                    <a href="/login">{t("login", lang)}</a>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, Debug, Default)]
struct RegisterState {
    username: String,
    password: String,
    confirm_password: String,
    loading: bool,
    error: Option<String>,
}
