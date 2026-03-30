use yew::prelude::*;
use yew::{function_component, html, use_state};
use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;
use crate::api;
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

    let onsubmit = {
        let state = state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let state = state.clone();
            
            // Validate passwords match
            if state.deref().password != state.deref().confirm_password {
                state.set(RegisterState {
                    error: Some("Passwords do not match".to_string()),
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
                        // Navigate to login page would go here
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
        <div class="glass rounded-2xl p-6 shadow-lg border border-emerald-100 dark:border-slate-700 animate-fade-in">
            <div class="text-center">
                <h2 class="text-3xl font-bold text-slate-800 dark:text-slate-200 mb-2">
                    {"Create Account"}
                </h2>
                <p class="text-slate-600 dark:text-slate-400">
                    {"Join our recipe community"}
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
                    <label for="name" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                        {"Full Name"}
                    </label>
                    <input
                        id="name"
                        name="name"
                        type="text"
                        required=true
                        class="appearance-none rounded-md relative block w-full px-3 py-2 border border-slate-300 dark:border-slate-600 placeholder-slate-500 dark:placeholder-slate-400 text-slate-900 dark:text-slate-100 bg-white dark:bg-slate-800 focus:outline-none focus:ring-emerald-500 focus:border-emerald-500 focus:z-10 sm:text-sm"
                        placeholder="Enter your full name"
                        value={state.name.clone()}
                        oninput={oninput.clone()}
                    />
                </div>

                <div>
                    <label for="email" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                        {"Email address"}
                    </label>
                    <input
                        id="email"
                        name="email"
                        type="email"
                        required=true
                        class="appearance-none rounded-md relative block w-full px-3 py-2 border border-slate-300 dark:border-slate-600 placeholder-slate-500 dark:placeholder-slate-400 text-slate-900 dark:text-slate-100 bg-white dark:bg-slate-800 focus:outline-none focus:ring-emerald-500 focus:border-emerald-500 focus:z-10 sm:text-sm"
                        placeholder="Enter your email"
                        value={state.email.clone()}
                        oninput={oninput.clone()}
                    />
                </div>

                <div>
                    <label for="password" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                        {"Password"}
                    </label>
                    <input
                        id="password"
                        name="password"
                        type="password"
                        required=true
                        class="appearance-none rounded-md relative block w-full px-3 py-2 border border-slate-300 dark:border-slate-600 placeholder-slate-500 dark:placeholder-slate-400 text-slate-900 dark:text-slate-100 bg-white dark:bg-slate-800 focus:outline-none focus:ring-emerald-500 focus:border-emerald-500 focus:z-10 sm:text-sm"
                        placeholder="Enter your password"
                        value={state.password.clone()}
                        oninput={oninput.clone()}
                    />
                </div>

                <div>
                    <label for="confirm_password" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                        {"Confirm Password"}
                    </label>
                    <input
                        id="confirm_password"
                        name="confirm_password"
                        type="password"
                        required=true
                        class="appearance-none rounded-md relative block w-full px-3 py-2 border border-slate-300 dark:border-slate-600 placeholder-slate-500 dark:placeholder-slate-400 text-slate-900 dark:text-slate-100 bg-white dark:bg-slate-800 focus:outline-none focus:ring-emerald-500 focus:border-emerald-500 focus:z-10 sm:text-sm"
                        placeholder="Confirm your password"
                        value={state.confirm_password.clone()}
                        oninput={oninput}
                    />
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
                                    {"Creating account..."}
                                </>
                            }
                        } else {
                            html! {"Create Account"}
                        }}
                    </button>
                </div>

                <div class="mt-6 text-center">
                    <div class="text-sm text-slate-600 dark:text-slate-400">
                        {"Already have an account? "}
                        <a href="/login" class="font-medium text-emerald-600 hover:text-emerald-500">
                            {"Sign in"}
                        </a>
                    </div>
                </div>
            </form>
        </div>
    }
}
