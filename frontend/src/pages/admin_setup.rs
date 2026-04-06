use yew::prelude::*;
use crate::api;
use web_sys::window;

#[derive(Clone, PartialEq)]
pub enum SetupStep {
    Welcome,
    CreateAdmin,
    Success,
}

#[function_component(AdminSetupPage)]
pub fn admin_setup_page() -> Html {
    let current_step = use_state(|| SetupStep::Welcome);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    
    let name = use_state(|| String::new());
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let confirm_password = use_state(|| String::new());

    let on_next = {
        let current_step = current_step.clone();
        Callback::from(move |_| {
            let step = (*current_step).clone();
            match step {
                SetupStep::Welcome => current_step.set(SetupStep::CreateAdmin),
                SetupStep::CreateAdmin => {}, // Handled by form submission
                SetupStep::Success => {}, // Should redirect to login
            }
        })
    };

    let on_create_admin = {
        let name = name.clone();
        let email = email.clone();
        let password = password.clone();
        let confirm_password = confirm_password.clone();
        let loading = loading.clone();
        let error = error.clone();
        let current_step = current_step.clone();

        Callback::from(move |_| {
            let name = (*name).clone();
            let email = (*email).clone();
            let password = (*password).clone();
            let confirm_password = (*confirm_password).clone();
            let loading = loading.clone();
            let error = error.clone();
            let current_step = current_step.clone();

            // Validation
            if name.trim().is_empty() {
                error.set(Some("Please enter your name".to_string()));
                return;
            }
            if email.trim().is_empty() {
                error.set(Some("Please enter your email".to_string()));
                return;
            }
            if password.len() < 6 {
                error.set(Some("Password must be at least 6 characters".to_string()));
                return;
            }
            if password != confirm_password {
                error.set(Some("Passwords do not match".to_string()));
                return;
            }

            loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                match api::create_initial_admin(name, email, password).await {
                    Ok(_) => {
                        loading.set(false);
                        current_step.set(SetupStep::Success);
                    }
                    Err(e) => {
                        loading.set(false);
                        error.set(Some(format!("Failed to create admin: {}", e)));
                    }
                }
            });
        })
    };

    let on_name_change = {
        let name = name.clone();
        Callback::from(move |e: yew::InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            name.set(input.value());
        })
    };

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: yew::InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            email.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: yew::InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            password.set(input.value());
        })
    };

    let on_confirm_password_change = {
        let confirm_password = confirm_password.clone();
        Callback::from(move |e: yew::InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            confirm_password.set(input.value());
        })
    };

    let step = (*current_step).clone();

    html! {
        <div class="min-h-screen bg-gradient-to-br from-emerald-50 via-white to-orange-50 dark:from-slate-900 dark:via-slate-800 dark:to-slate-900 flex items-center justify-center p-4">
            <div class="w-full max-w-md">
                // Header
                <div class="text-center mb-8">
                    <div class="w-16 h-16 bg-gradient-to-br from-emerald-400 to-emerald-600 rounded-2xl flex items-center justify-center shadow-lg mx-auto mb-4">
                        <span class="text-white text-2xl">{"🍳"}</span>
                    </div>
                    <h1 class="text-3xl font-bold bg-gradient-to-r from-emerald-600 to-emerald-800 dark:from-emerald-400 dark:to-emerald-300 bg-clip-text text-transparent mb-2">
                        {"Kitchenbox Setup"}
                    </h1>
                    <p class="text-slate-600 dark:text-slate-400">
                        {"Let's get your recipe manager configured"}
                    </p>
                </div>

                // Progress Indicator
                <div class="mb-8">
                    <div class="flex items-center justify-between">
                        <div class={if matches!(step, SetupStep::Welcome) { "w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium transition-colors bg-emerald-600 text-white" } else { "w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium transition-colors bg-emerald-200 text-emerald-600 dark:bg-emerald-800 dark:text-emerald-300" }}>
                            {"1"}
                        </div>
                        <div class={if matches!(step, SetupStep::CreateAdmin) || matches!(step, SetupStep::Success) { "flex-1 h-1 mx-2 transition-colors bg-emerald-600" } else { "flex-1 h-1 mx-2 transition-colors bg-emerald-200 dark:bg-emerald-800" }}></div>
                        <div class={if matches!(step, SetupStep::CreateAdmin) { "w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium transition-colors bg-emerald-600 text-white" } else { "w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium transition-colors bg-emerald-200 text-emerald-600 dark:bg-emerald-800 dark:text-emerald-300" }}>
                            {"2"}
                        </div>
                        <div class={if matches!(step, SetupStep::Success) { "flex-1 h-1 mx-2 transition-colors bg-emerald-600" } else { "flex-1 h-1 mx-2 transition-colors bg-emerald-200 dark:bg-emerald-800" }}></div>
                        <div class={if matches!(step, SetupStep::Success) { "w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium transition-colors bg-emerald-600 text-white" } else { "w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium transition-colors bg-emerald-200 text-emerald-600 dark:bg-emerald-800 dark:text-emerald-300" }}>
                            {"✓"}
                        </div>
                    </div>
                </div>

                // Content Card
                <div class="glass rounded-2xl p-8 shadow-xl">
                    {match step {
                        SetupStep::Welcome => html! {
                            <div class="text-center">
                                <div class="w-16 h-16 bg-emerald-100 dark:bg-emerald-900 rounded-full flex items-center justify-center mx-auto mb-4">
                                    <svg class="w-8 h-8 text-emerald-600 dark:text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"></path>
                                    </svg>
                                </div>
                                <h2 class="text-2xl font-semibold text-slate-800 dark:text-slate-200 mb-4">
                                    {"Welcome to Kitchenbox!"}
                                </h2>
                                <p class="text-slate-600 dark:text-slate-400 mb-6">
                                    {"This appears to be your first time running Kitchenbox. Let's create an administrator account to get you started."}
                                </p>
                                <div class="space-y-3 text-left bg-slate-50 dark:bg-slate-800 rounded-lg p-4 mb-6">
                                    <div class="flex items-center space-x-3">
                                        <svg class="w-5 h-5 text-emerald-600 dark:text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                        </svg>
                                        <span class="text-sm text-slate-700 dark:text-slate-300">{"Manage all users and recipes"}</span>
                                    </div>
                                    <div class="flex items-center space-x-3">
                                        <svg class="w-5 h-5 text-emerald-600 dark:text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                        </svg>
                                        <span class="text-sm text-slate-700 dark:text-slate-300">{"Configure system settings"}</span>
                                    </div>
                                    <div class="flex items-center space-x-3">
                                        <svg class="w-5 h-5 text-emerald-600 dark:text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                        </svg>
                                        <span class="text-sm text-slate-700 dark:text-slate-300">{"Full access to all features"}</span>
                                    </div>
                                </div>
                                <button 
                                    onclick={on_next}
                                    class="w-full bg-gradient-to-r from-emerald-500 to-emerald-600 hover:from-emerald-600 hover:to-emerald-700 text-white font-medium py-3 px-6 rounded-lg transition-all transform hover:scale-105 shadow-lg"
                                >
                                    {"Get Started"}
                                </button>
                            </div>
                        },
                        SetupStep::CreateAdmin => html! {
                            <div>
                                <h2 class="text-2xl font-semibold text-slate-800 dark:text-slate-200 mb-6 text-center">
                                    {"Create Administrator Account"}
                                </h2>
                                
                                {if let Some(ref error_msg) = *error {
                                    html! {
                                        <div class="mb-4 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
                                            <p class="text-red-700 dark:text-red-300 text-sm">{error_msg}</p>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }}

                                <form class="space-y-4">
                                    <div>
                                        <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                                            {"Name"}
                                        </label>
                                        <input
                                            type="text"
                                            value={(*name).clone()}
                                            oninput={on_name_change}
                                            placeholder="Enter your name"
                                            class="w-full px-4 py-3 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                            disabled={*loading}
                                        />
                                    </div>

                                    <div>
                                        <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                                            {"Email"}
                                        </label>
                                        <input
                                            type="email"
                                            value={(*email).clone()}
                                            oninput={on_email_change}
                                            placeholder="admin@example.com"
                                            class="w-full px-4 py-3 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                            disabled={*loading}
                                        />
                                    </div>

                                    <div>
                                        <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                                            {"Password"}
                                        </label>
                                        <input
                                            type="password"
                                            value={(*password).clone()}
                                            oninput={on_password_change}
                                            placeholder="Create a strong password"
                                            class="w-full px-4 py-3 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                            disabled={*loading}
                                        />
                                    </div>

                                    <div>
                                        <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                                            {"Confirm Password"}
                                        </label>
                                        <input
                                            type="password"
                                            value={(*confirm_password).clone()}
                                            oninput={on_confirm_password_change}
                                            placeholder="Confirm your password"
                                            class="w-full px-4 py-3 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                            disabled={*loading}
                                        />
                                    </div>

                                    <button
                                        type="button"
                                        onclick={on_create_admin}
                                        disabled={*loading}
                                        class="w-full bg-gradient-to-r from-emerald-500 to-emerald-600 hover:from-emerald-600 hover:to-emerald-700 disabled:from-slate-400 disabled:to-slate-500 text-white font-medium py-3 px-6 rounded-lg transition-all transform hover:scale-105 disabled:scale-100 shadow-lg disabled:shadow-none flex items-center justify-center"
                                    >
                                        {if *loading {
                                            html! {
                                                <div class="flex items-center justify-center">
                                                    <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                                    </svg>
                                                    <span>{"Creating Admin..."}</span>
                                                </div>
                                            }
                                        } else {
                                            html! { <span>{"Create Administrator"}</span> }
                                        }}
                                    </button>
                                </form>
                            </div>
                        },
                        SetupStep::Success => html! {
                            <div class="text-center">
                                <div class="w-16 h-16 bg-green-100 dark:bg-green-900 rounded-full flex items-center justify-center mx-auto mb-4">
                                    <svg class="w-8 h-8 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                    </svg>
                                </div>
                                <h2 class="text-2xl font-semibold text-slate-800 dark:text-slate-200 mb-4">
                                    {"Setup Complete!"}
                                </h2>
                                <p class="text-slate-600 dark:text-slate-400 mb-6">
                                    {"Your administrator account has been created successfully. You can now log in and start using Kitchenbox."}
                                </p>
                                <button 
                                    onclick={Callback::from(|_| {
                                        // Redirect to login page
                                        if let Some(window) = window() {
                                            let _ = window.location().reload();
                                        }
                                    })}
                                    class="w-full bg-gradient-to-r from-emerald-500 to-emerald-600 hover:from-emerald-600 hover:to-emerald-700 text-white font-medium py-3 px-6 rounded-lg transition-all transform hover:scale-105 shadow-lg"
                                >
                                    {"Go to Login"}
                                </button>
                            </div>
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
