use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::api;
use crate::i18n::t;
use crate::language_provider::LanguageState;

#[derive(Clone)]
struct LoginState {
    username: String,
    password: String,
    loading: bool,
    error: Option<String>,
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let state = use_state(|| LoginState {
        username: String::new(),
        password: String::new(),
        loading: false,
        error: None,
    });
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(crate::i18n::Language::English);

    let onsubmit = {
        let state = state.clone();
        let lang = lang.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let current = (*state).clone();
            let username = current.username;
            let password = current.password;

            if username.trim().is_empty() || password.is_empty() {
                let mut err_state = (*state).clone();
                err_state.error = Some("Please fill in all fields.".to_string());
                state.set(err_state);
                return;
            }

            state.set(LoginState {
                username: username.clone(),
                password: password.clone(),
                loading: true,
                error: None,
            });

            let s2 = state.clone();
            let l2 = lang.clone();
            let username2 = username.clone();
            let password2 = password.clone();

            wasm_bindgen_futures::spawn_local(async move {
                on_login_response(&username2, &password2, s2, l2).await;
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
                    <p>{t("app_tagline", lang)}</p>
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
                        <label class="form-label" for="login-username">{"Benutzername"}</label>
                        <input id="login-username" name="username" class="form-input" type="text"
                            placeholder={"Benutzername"}
                            value={(*state).username.clone()}
                            oninput={oninput.clone()}
                        />
                    </div>
                    <div class="form-group">
                        <label class="form-label" for="login-password">{t("password", lang)}</label>
                        <input id="login-password" name="password" class="form-input" type="password"
                            placeholder={t("password", lang)}
                            value={(*state).password.clone()}
                            oninput={oninput}
                        />
                    </div>
                    <button type="submit" class="btn btn-primary w-full" disabled={(*state).loading}>
                        {if (*state).loading { t("signing_in", lang) } else { t("login_button", lang) }}
                    </button>
                </form>

                <div class="auth-footer">
                    {t("dont_have_account", lang) + " "}
                    <a href="/register">{t("sign_up", lang)}</a>
                </div>
            </div>
        </div>
    }
}

async fn on_login_response(
    username: &str,
    password: &str,
    state: yew::UseStateHandle<LoginState>,
    lang: crate::i18n::Language,
) {
    match api::login(username, password).await {
        Ok(response) => {
            let user_obj = response.get("user").and_then(|u| u.as_object());
            let user_id = user_obj.and_then(|u| u.get("id")).and_then(|i| i.as_u64()).map(|i| i.to_string());
            let display_name = user_obj.and_then(|u| u.get("name")).and_then(|n| n.as_str()).map(|s| s.to_string());
            let is_admin = user_obj.and_then(|u| u.get("is_admin")).and_then(|a| a.as_bool()).unwrap_or(false);
            let token = response.get("token").and_then(|t| t.as_str()).map(|s| s.to_string());
            let has_token = token.is_some();

            if let (Some(uid), Some(name), Some(tok)) = (user_id, display_name, token) {
                let mut auth = home_hub_shared::Auth::new();
                auth.login(name.clone(), uid, tok.clone(), is_admin);
                web_sys::window().unwrap().location().set_href("/").unwrap();
            } else if !has_token {
                let mut s = (*state).clone();
                s.loading = false;
                s.error = Some(t("failed_auth_token", lang));
                state.set(s);
            } else {
                let mut s = (*state).clone();
                s.loading = false;
                s.error = Some(t("failed_user_info", lang));
                state.set(s);
            }
        }
        Err(e) => {
            let mut s = (*state).clone();
            s.loading = false;
            s.error = Some(e);
            state.set(s);
        }
    }
}
