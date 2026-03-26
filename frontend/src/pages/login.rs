use yew::prelude::*;
use yew::{function_component, html, use_state};
use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;
use crate::api;
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
                                            error: Some("Failed to get authentication token".to_string()),
                                            ..state.deref().clone()
                                        });
                                    }
                                } else {
                                    state.set(LoginState {
                                        loading: false,
                                        error: Some("Failed to get user info".to_string()),
                                        ..state.deref().clone()
                                    });
                                }
                            } else {
                                state.set(LoginState {
                                    loading: false,
                                    error: Some("Failed to get user ID".to_string()),
                                    ..state.deref().clone()
                                });
                            }
                        } else {
                            state.set(LoginState {
                                loading: false,
                                error: Some("Failed to get user info".to_string()),
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
        <div class="min-h-screen bg-gradient-to-br from-emerald-50 via-white to-teal-50 flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                <div class="bg-white rounded-2xl shadow-xl overflow-hidden">
                    <div class="p-8">
                        <div class="text-center">
                            <h2 class="text-3xl font-bold text-gray-900 mb-2">
                                {"Recipe App"}
                            </h2>
                            <p class="text-gray-600">
                                {"Sign in to your account"}
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
                                <label for="email" class="block text-sm font-medium text-gray-700 mb-2">
                                    {"Email address"}
                                </label>
                                <input
                                    id="email"
                                    name="email"
                                    type="email"
                                    required=true
                                    class="appearance-none rounded-md relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-emerald-500 focus:border-emerald-500 focus:z-10 sm:text-sm"
                                    placeholder="Enter your email"
                                    value={state.email.clone()}
                                    oninput={oninput.clone()}
                                />
                            </div>

                            <div>
                                <label for="password" class="block text-sm font-medium text-gray-700 mb-2">
                                    {"Password"}
                                </label>
                                <input
                                    id="password"
                                    name="password"
                                    type="password"
                                    required=true
                                    class="appearance-none rounded-md relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-emerald-500 focus:border-emerald-500 focus:z-10 sm:text-sm"
                                    placeholder="Enter your password"
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
                                        class="h-4 w-4 text-emerald-600 focus:ring-emerald-500 border-gray-300 rounded"
                                    />
                                    <label for="remember-me" class="ml-2 block text-sm text-gray-900">
                                        {"Remember me"}
                                    </label>
                                </div>

                                <div class="text-sm">
                                    <a href="#" class="font-medium text-emerald-600 hover:text-emerald-500">
                                        {"Forgot your password?"}
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
                                                {"Signing in..."}
                                            </>
                                        }
                                    } else {
                                        html! {"Sign in"}
                                    }}
                                </button>
                            </div>

                            <div class="mt-6 text-center">
                                <div class="text-sm">
                                    {"Don't have an account? "}
                                    <a href="#" class="font-medium text-emerald-600 hover:text-emerald-500">
                                        {"Sign up"}
                                    </a>
                                </div>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
