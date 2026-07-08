use yew::prelude::*;
use yew::{function_component, html, use_state};
use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use crate::i18n::{Language, t};
use crate::language_provider::LanguageState;
use shared_types::User;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct RegisterState {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub loading: bool,
    pub error: Option<String>,
}

impl Default for RegisterState {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            loading: false,
            error: None,
        }
    }
}

#[function_component(RegisterPage)]
pub fn register() -> Html {
    let state = use_state(RegisterState::default);
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

    let onsubmit = {
        let state = state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let state = state.clone();

            if state.deref().password != state.deref().confirm_password {
                state.set(RegisterState {
                    error: Some(t("passwords_match", lang)),
                    loading: false,
                    ..state.deref().clone()
                });
                return;
            }

            spawn_local(async move {
                state.set(RegisterState {
                    loading: true,
                    error: None,
                    ..state.deref().clone()
                });

                let user = User {
                    id: None,
                    name: state.deref().name.clone(),
                    email: state.deref().email.clone(),
                    password: Some(state.deref().password.clone()),
                };

                match api::create_user(&user).await {
                    Ok(_) => {
                        web_sys::window().unwrap().location().set_href("/login").unwrap();
                    }
                    Err(e) => {
                        state.set(RegisterState {
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
                    "name" => {
                        state.set(RegisterState {
                            name: value,
                            ..state.deref().clone()
                        });
                    }
                    "email" => {
                        state.set(RegisterState {
                            email: value,
                            ..state.deref().clone()
                        });
                    }
                    "password" => {
                        state.set(RegisterState {
                            password: value,
                            ..state.deref().clone()
                        });
                    }
                    "confirm_password" => {
                        state.set(RegisterState {
                            confirm_password: value,
                            ..state.deref().clone()
                        });
                    }
                    _ => {}
                }
            }
        })
    };

    html! {
        <div class="auth-card page-enter">
            <div class="auth-logo">
                <div class="auth-logo-icon">
                    <svg fill="currentColor" viewBox="0 0 24 24">
                        <path d="M21 4.5C19.9 4.15 18.7 4 17.5 4c-1.95 0-4.05.4-5.5 1.5C10.55 4.4 8.45 4 6.5 4 5.3 4 4.1 4.15 3 4.5v14.65c0 .25.25.5.5.5.1 0 .15-.05.25-.05 1.1-.35 2.3-.5 3.5-.5 1.95 0 4.05.4 5.5 1.5 1.45-1.1 3.55-1.5 5.5-1.5 1.2 0 2.4.15 3.5.5.1.05.15.05.25.05.25 0 .5-.25.5-.5V4.5z"/>
                    </svg>
                </div>
                <h1>{t("create_account", lang)}</h1>
                <p>{t("join_community", lang)}</p>
            </div>

            <form onsubmit={onsubmit}>
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
                    <label for="name" class="form-label">{t("full_name", lang)}</label>
                    <input
                        id="name"
                        name="name"
                        type="text"
                        required=true
                        class="form-input"
                        placeholder={t("enter_full_name", lang)}
                        value={state.name.clone()}
                        oninput={oninput.clone()}
                    />
                </div>

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
                        oninput={oninput.clone()}
                    />
                </div>

                <div class="form-group">
                    <label for="confirm_password" class="form-label">{t("confirm_password_label", lang)}</label>
                    <input
                        id="confirm_password"
                        name="confirm_password"
                        type="password"
                        required=true
                        class="form-input"
                        placeholder={t("confirm_password_placeholder", lang)}
                        value={state.confirm_password.clone()}
                        oninput={oninput}
                    />
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
                                    {t("creating_account", lang)}
                                </>
                            }
                        } else {
                            html! {t("register_button", lang)}
                        }}
                    </button>
                </div>

                <div class="auth-footer">
                    <p>{t("already_have_account", lang)} <a href="/login" class="auth-footer-link">{t("login", lang)}</a></p>
                </div>
            </form>
        </div>
    }
}
