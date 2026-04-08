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
                        // Extract user info from the response
                        if let Some(user) = response.get("user").and_then(|u| u.as_object()) {
                            if let Some(id) = user.get("id").and_then(|i| i.as_u64()) {
                                if let Some(email) = user.get("email").and_then(|e| e.as_str()) {
                                    // Store user info in localStorage
                                    if let Some(window) = web_sys::window() {
                                        if let Ok(Some(storage)) = window.local_storage() {
                                            let _ = storage.set_item("user_id", &id.to_string());
                                            let _ = storage.set_item("user_email", email);
                                            // Store user name if available
                                            if let Some(name) = user.get("name").and_then(|n| n.as_str()) {
                                                let _ = storage.set_item("user_name", name);
                                            }
                                            // Store admin status if available
                                            if let Some(is_admin) = user.get("is_admin").and_then(|a| a.as_bool()) {
                                                let _ = storage.set_item("user_is_admin", &is_admin.to_string());
                                            }
                                        }
                                    }
                                    
                                    // Extract the actual JWT token from the response
                                    if let Some(token) = response.get("token").and_then(|t| t.as_str()) {
                                        // Store token in localStorage
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
        <div class="glass rounded-2xl p-6 shadow-lg border border-emerald-100 dark:border-slate-700 animate-fade-in">
            <div class="text-center">
                <h2 class="text-3xl font-bold text-slate-800 dark:text-slate-200 mb-2">
                    {t("app_name", lang)}
                </h2>
                <p class="text-slate-600 dark:text-slate-400">
                    {t("sign_in_account", lang)}
                </p>
            </div>

            <form class="mt-8 space-y-6" onsubmit={onsubmit}>
                {if let Some(ref error) = state.deref().error {
                    html! {
                        <div class="bg-red-50 border border-red-200 text-red-600 px-4 py-3 rounded-md mb-4">
                            {error}
                        </div>
                    }
                } else {
                    html! {}
                }}

                <div>
                    <label for="email" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                        {t("email_address", lang)}
                    </label>
                    <input
                        id="email"
                        name="email"
                        type="email"
                        required=true
                        class="appearance-none rounded-md relative block w-full px-3 py-2 border border-slate-300 dark:border-slate-600 placeholder-slate-500 dark:placeholder-slate-400 text-slate-900 dark:text-slate-100 bg-white dark:bg-slate-800 focus:outline-none focus:ring-emerald-500 focus:border-emerald-500 focus:z-10 sm:text-sm"
                        placeholder={t("enter_email", lang)}
                        value={state.email.clone()}
                        oninput={oninput.clone()}
                    />
                </div>

                <div>
                    <label for="password" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                        {t("password", lang)}
                    </label>
                    <input
                        id="password"
                        name="password"
                        type="password"
                        required=true
                        class="appearance-none rounded-md relative block w-full px-3 py-2 border border-slate-300 dark:border-slate-600 placeholder-slate-500 dark:placeholder-slate-400 text-slate-900 dark:text-slate-100 bg-white dark:bg-slate-800 focus:outline-none focus:ring-emerald-500 focus:border-emerald-500 focus:z-10 sm:text-sm"
                        placeholder={t("enter_password", lang)}
                        value={state.password.clone()}
                        oninput={oninput}
                    />
                </div>

                <div class="flex items-center justify-between">
                    <div class="flex items-center">
                        <input
                            id="remember-me"
                            name="remember-me"
                            type="checkbox"
                            class="h-4 w-4 text-emerald-600 focus:ring-emerald-500 border-slate-300 dark:border-slate-600 rounded"
                        />
                        <label for="remember-me" class="ml-2 block text-sm text-slate-900 dark:text-slate-100">
                            {t("remember_me", lang)}
                        </label>
                    </div>

                    <div class="text-sm">
                        <a href="#" class="font-medium text-emerald-600 hover:text-emerald-500">
                            {t("forgot_password", lang)}
                        </a>
                    </div>
                </div>

                <div>
                    <button
                        type="submit"
                        disabled={state.loading}
                        class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-emerald-600 hover:bg-emerald-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-emerald-500 disabled:opacity-50"
                    >
                        {if state.loading {
                            html! {
                                <>
                                    <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0h12c6.627 0 12 5.373 12v12c0 6.627-5.373 12-12h-4zm-1 1.465L9.465 15H15v-2h-4v-2h4v-2z"></path>
                                    </svg>
                                    {t("signing_in", lang)}
                                </>
                            }
                        } else {
                            html! {t("login_button", lang)}
                        }}
                    </button>
                </div>

                <div class="mt-6 text-center">
                    <div class="text-sm text-slate-600 dark:text-slate-400">
                        {t("dont_have_account", lang)}
                        <a href="#" class="font-medium text-emerald-600 hover:text-emerald-500">
                            {t("sign_up", lang)}
                        </a>
                    </div>
                </div>
            </form>
        </div>
    }
}
