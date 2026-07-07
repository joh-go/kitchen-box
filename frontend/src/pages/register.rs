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
        <div class="card page-enter">
            <div class="text-center">
                <h2 class="page-title">{t("create_account", lang)}</h2>
                <p class="text-muted">{t("join_community", lang)}</p>
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
