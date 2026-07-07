use yew::prelude::*;
use yew::{function_component, html, use_state};
use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use crate::i18n::{Language, t};
use crate::language_provider::LanguageState;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct LoginState {
    pub email: String,
    pub password: String,
    pub loading: bool,
    pub error: Option<String>,
}

impl Default for LoginState {
    fn default() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            loading: false,
            error: None,
        }
    }
}

#[function_component(LoginPage)]
pub fn login() -> Html {
    let state = use_state(LoginState::default);
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

    let onsubmit = {
        let state = state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let state = state.clone();

            spawn_local(async move {
                state.set(LoginState {
                    loading: true,
                    error: None,
                    ..state.deref().clone()
                });

                let email = state.deref().email.clone();
                let password = state.deref().password.clone();

                match api::login(&email, &password).await {
                    Ok(response) => {
                        if let Some(user) = response.get("user").and_then(|u| u.as_object()) {
                            if let Some(id) = user.get("id").and_then(|i| i.as_u64()) {
                                if let Some(email) = user.get("email").and_then(|e| e.as_str()) {
                                    if let Some(window) = web_sys::window() {
                                        if let Ok(Some(storage)) = window.local_storage() {
                                            let _ = storage.set_item("user_id", &id.to_string());
                                            let _ = storage.set_item("user_email", email);
                                            if let Some(name) = user.get("name").and_then(|n| n.as_str()) {
                                                let _ = storage.set_item("user_name", name);
                                            }
                                            if let Some(is_admin) = user.get("is_admin").and_then(|a| a.as_bool()) {
                                                let _ = storage.set_item("user_is_admin", &is_admin.to_string());
                                            }
                                        }
                                    }

                                    if let Some(token) = response.get("token").and_then(|t| t.as_str()) {
                                        if let Some(window) = web_sys::window() {
                                            if let Ok(Some(storage)) = window.local_storage() {
                                                let _ = storage.set_item("auth_token", token);
                                            }
                                        }
                                        web_sys::window().unwrap().location().set_href("/").unwrap();
                                    } else {
                                        state.set(LoginState {
                                            loading: false,
                                            error: Some(t("failed_auth_token", lang)),
                                            ..state.deref().clone()
                                        });
                                    }
                                } else {
                                    state.set(LoginState {
                                        loading: false,
                                        error: Some(t("failed_user_info", lang)),
                                        ..state.deref().clone()
                                    });
                                }
                            } else {
                                state.set(LoginState {
                                    loading: false,
                                    error: Some(t("failed_user_id", lang)),
                                    ..state.deref().clone()
                                });
                            }
                        } else {
                            state.set(LoginState {
                                loading: false,
                                error: Some(t("failed_user_info", lang)),
                                ..state.deref().clone()
                            });
                        }
                    }
                    Err(e) => {
                        state.set(LoginState {
                            loading: false,
                            error: Some(e),
                            ..state.deref().clone()
                        });
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

            if let Some(name) = input.get_attribute("name") {
                match name.as_str() {
                    "email" => {
                        state.set(LoginState {
                            email: value,
                            ..state.deref().clone()
                        });
                    }
                    "password" => {
                        state.set(LoginState {
                            password: value,
                            ..state.deref().clone()
                        });
                    }
                    _ => {}
                }
            }
        })
    };

    html! {
        <div class="card page-enter">
            <div class="text-center">
                <h2 class="page-title">
                    {t("app_name", lang)}
                </h2>
                <p class="text-muted">
                    {t("sign_in_account", lang)}
                </p>
            </div>

            <form class="flex flex-col gap-6 mt-8" onsubmit={onsubmit}>
                {if let Some(ref error) = state.deref().error {
                    html! {
                        <div class="auth-error">
                            <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                            {error}
                        </div>
                    }
                } else {
                    html! {}
                }}

                <div class="form-group">
                    <label for="email" class="form-label">{t("email_address", lang)}</label>
                    <input
                        id="email"
                        name="email"
                        type="email"
                        required=true
                        class="form-input"
                        placeholder={t("enter_email", lang)}
                        value={state.email.clone()}
                        oninput={oninput.clone()}
                    />
                </div>

                <div class="form-group">
                    <label for="password" class="form-label">{t("password", lang)}</label>
                    <input
                        id="password"
                        name="password"
                        type="password"
                        required=true
                        class="form-input"
                        placeholder={t("enter_password", lang)}
                        value={state.password.clone()}
                        oninput={oninput}
                    />
                </div>

                <div class="auth-checkbox-group">
                    <label class="auth-checkbox">
                        <input
                            type="checkbox"
                            id="remember-me"
                        />
                        {t("remember_me", lang)}
                    </label>
                    <button type="button" class="auth-checkbox-link">{t("forgot_password", lang)}</button>
                </div>

                <div>
                    <button
                        type="submit"
                        disabled={state.loading}
                        class="btn btn-primary w-full"
                    >
                        {if state.loading {
                            html! {
                                <>
                                    <div class="spinner spinner-sm"><div class="spinner-circle"></div></div>
                                    {t("signing_in", lang)}
                                </>
                            }
                        } else {
                            html! {t("login_button", lang)}
                        }}
                    </button>
                </div>

                <div class="auth-footer">
                    <p>{t("dont_have_account", lang)} <button type="button" class="auth-footer-link">{t("sign_up", lang)}</button></p>
                </div>
            </form>
        </div>
    }
}
