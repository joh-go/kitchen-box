use yew::prelude::*;
use yew::{function_component, html, use_state, use_effect_with};
use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use crate::i18n::{Language, t};
use crate::language_provider::LanguageState;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct SettingsState {
    pub name: String,
    pub email: String,
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
    pub loading: bool,
    pub error: Option<String>,
    pub success: Option<String>,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            name: api::get_current_user_name().unwrap_or_default(),
            email: String::new(),
            current_password: String::new(),
            new_password: String::new(),
            confirm_password: String::new(),
            loading: false,
            error: None,
            success: None,
        }
    }
}

#[function_component(SettingsPage)]
pub fn settings() -> Html {
    let state = use_state(SettingsState::default);
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);
    
    let set_lang = {
        let lang_ctx = lang_ctx.clone();
        Callback::from(move |new_lang: Language| {
            if let Some(ref ctx) = lang_ctx {
                ctx.dispatch(crate::language_provider::LanguageAction::SetLanguage(new_lang));
            }
        })
    };

    // Fetch current user data on page load
    {
        let state = state.clone();
        use_effect_with((), move |_| {
            if api::is_logged_in() {
                spawn_local(async move {
                    match api::get_current_user().await {
                        Ok(user) => {
                            let name = user.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();
                            let email = user.get("email").and_then(|e| e.as_str()).unwrap_or("").to_string();
                            state.set(SettingsState {
                                name,
                                email,
                                ..state.deref().clone()
                            });
                        }
                        Err(_) => {
                            // Silently fail - user can still enter data manually
                        }
                    }
                });
            }
            || ()
        });
    }

    let onsubmit = {
        let state = state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let state = state.clone();
            
            // Validate passwords match if changing password
            if !state.deref().new_password.is_empty() {
                if state.deref().new_password != state.deref().confirm_password {
                    state.set(SettingsState {
                        error: Some(t("passwords_do_not_match", lang)),
                        success: None,
                        ..state.deref().clone()
                    });
                    return;
                }
                if state.deref().current_password.is_empty() {
                    state.set(SettingsState {
                        error: Some(t("current_password_required", lang)),
                        success: None,
                        ..state.deref().clone()
                    });
                    return;
                }
            }

            spawn_local(async move {
                state.set(SettingsState {
                    loading: true,
                    error: None,
                    success: None,
                    ..state.deref().clone()
                });

                let name = state.deref().name.clone();
                let email = state.deref().email.clone();
                let current_password = state.deref().current_password.clone();
                let new_password = state.deref().new_password.clone();

                match api::update_profile(&name, &email, &current_password, &new_password).await {
                    Ok(_) => {
                        // Update localStorage with new name
                        if let Some(window) = web_sys::window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                let _ = storage.set_item("user_name", &name);
                            }
                        }
                        
                        state.set(SettingsState {
                            loading: false,
                            error: None,
                            success: Some(t("profile_updated", lang)),
                            current_password: String::new(),
                            new_password: String::new(),
                            confirm_password: String::new(),
                            ..state.deref().clone()
                        });
                    }
                    Err(e) => {
                        state.set(SettingsState {
                            loading: false,
                            error: Some(e),
                            success: None,
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
                        state.set(SettingsState {
                            name: value,
                            ..state.deref().clone()
                        });
                    }
                    "email" => {
                        state.set(SettingsState {
                            email: value,
                            ..state.deref().clone()
                        });
                    }
                    "current_password" => {
                        state.set(SettingsState {
                            current_password: value,
                            ..state.deref().clone()
                        });
                    }
                    "new_password" => {
                        state.set(SettingsState {
                            new_password: value,
                            ..state.deref().clone()
                        });
                    }
                    "confirm_password" => {
                        state.set(SettingsState {
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
        <div class="space-y-6">
            <div class="animate-fade-in">
                <h1 class="text-2xl sm:text-3xl font-bold text-slate-800 dark:text-slate-200">
                    {t("account_settings", lang)}
                </h1>
                <p class="text-slate-500 dark:text-slate-400 mt-1">
                    {t("manage_profile_password", lang)}
                </p>
            </div>

            <div class="glass rounded-2xl p-6 shadow-lg border border-emerald-100 dark:border-slate-700 animate-fade-in">
                <form class="space-y-6" onsubmit={onsubmit}>
                    {if let Some(ref error) = state.deref().error {
                        html! {
                            <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-600 dark:text-red-400 px-4 py-3 rounded-lg">
                                {error}
                            </div>
                        }
                    } else {
                        html! {}
                    }}

                    {if let Some(ref success) = state.deref().success {
                        html! {
                            <div class="bg-emerald-50 dark:bg-emerald-900/20 border border-emerald-200 dark:border-emerald-800 text-emerald-600 dark:text-emerald-400 px-4 py-3 rounded-lg">
                                {success}
                            </div>
                        }
                    } else {
                        html! {}
                    }}

                    <div>
                        <h2 class="text-lg font-semibold text-slate-800 dark:text-slate-200 mb-4">
                            {t("profile_information", lang)}
                        </h2>
                        
                        <div class="space-y-4">
                            <div>
                                <label for="name" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                                    {t("display_name", lang)}
                                </label>
                                <input
                                    id="name"
                                    name="name"
                                    type="text"
                                    class="w-full px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                    placeholder={t("enter_display_name", lang)}
                                    value={state.name.clone()}
                                    oninput={oninput.clone()}
                                />
                            </div>

                            <div>
                                <label for="email" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                                    {t("email_address", lang)}
                                </label>
                                <input
                                    id="email"
                                    name="email"
                                    type="email"
                                    class="w-full px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                    placeholder={t("enter_email", lang)}
                                    value={state.email.clone()}
                                    oninput={oninput.clone()}
                                />
                            </div>
                        </div>
                    </div>

                    <div class="border-t border-slate-200 dark:border-slate-700 pt-6">
                        <h2 class="text-lg font-semibold text-slate-800 dark:text-slate-200 mb-4">
                            {t("language", lang)}
                        </h2>
                        
                        <div class="space-y-4">
                            <div>
                                <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                                    {t("language", lang)}
                                </label>
                                <select
                                    class="w-full px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                    onchange={{
                                        let set_lang = set_lang.clone();
                                        Callback::from(move |e: Event| {
                                            let target = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                                            let value = target.value();
                                            match value.as_str() {
                                                "de" => set_lang.emit(Language::German),
                                                _ => set_lang.emit(Language::English),
                                            }
                                        })
                                    }}
                                >
                                    <option value="en" selected={lang == Language::English}>{"English"}</option>
                                    <option value="de" selected={lang == Language::German}>{"Deutsch"}</option>
                                </select>
                            </div>
                        </div>
                    </div>

                    <div class="border-t border-slate-200 dark:border-slate-700 pt-6">
                        <h2 class="text-lg font-semibold text-slate-800 dark:text-slate-200 mb-4">
                            {t("change_password", lang)}
                        </h2>
                        
                        <div class="space-y-4">
                            <div>
                                <label for="current_password" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                                    {t("current_password_label", lang)}
                                </label>
                                <input
                                    id="current_password"
                                    name="current_password"
                                    type="password"
                                    class="w-full px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                    placeholder={t("enter_current_password", lang)}
                                    value={state.current_password.clone()}
                                    oninput={oninput.clone()}
                                />
                            </div>

                            <div>
                                <label for="new_password" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                                    {t("new_password_label", lang)}
                                </label>
                                <input
                                    id="new_password"
                                    name="new_password"
                                    type="password"
                                    class="w-full px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                    placeholder={t("enter_new_password", lang)}
                                    value={state.new_password.clone()}
                                    oninput={oninput.clone()}
                                />
                            </div>

                            <div>
                                <label for="confirm_password" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                                    {t("confirm_new_password", lang)}
                                </label>
                                <input
                                    id="confirm_password"
                                    name="confirm_password"
                                    type="password"
                                    class="w-full px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                    placeholder={t("confirm_new_password", lang)}
                                    value={state.confirm_password.clone()}
                                    oninput={oninput}
                                />
                            </div>
                        </div>
                    </div>

                    <div class="flex justify-end">
                        <button
                            type="submit"
                            disabled={state.loading}
                            class="touch-target btn-primary text-white px-6 py-2.5 rounded-lg font-medium flex items-center justify-center gap-2 transition-all duration-200 disabled:opacity-50"
                        >
                            {if state.loading {
                                html! {
                                    <>
                                        <svg class="animate-spin -ml-1 mr-2 h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0h12c6.627 0 12 5.373 12v12c0 6.627-5.373 12-12h-4zm-1 1.465L9.465 15H15v-2h-4v-2h4v-2z"></path>
                                        </svg>
                                        {t("saving", lang)}
                                    </>
                                }
                            } else {
                                html! {t("save_changes", lang)}
                            }}
                        </button>
                    </div>
                </form>
            </div>

            // Admin Section (only show if user is admin)
            {if api::is_current_user_admin() {
                html! {
                    <div class="glass rounded-2xl p-6 shadow-lg border border-emerald-100 dark:border-slate-700 animate-fade-in">
                        <div class="flex items-center space-x-3 mb-6">
                            <div class="w-10 h-10 bg-gradient-to-br from-emerald-400 to-emerald-600 rounded-xl flex items-center justify-center shadow-lg">
                                <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"></path>
                                </svg>
                            </div>
                            <div>
                                <h2 class="text-xl font-semibold text-slate-800 dark:text-slate-200">
                                    {t("admin_panel", lang)}
                                </h2>
                                <p class="text-sm text-slate-500 dark:text-slate-400">
                                    {t("manage_admin_settings", lang)}
                                </p>
                            </div>
                        </div>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            // User Management
                            <div class="bg-slate-50 dark:bg-slate-800 rounded-lg p-4 border border-slate-200 dark:border-slate-700">
                                <div class="flex items-center space-x-3 mb-3">
                                    <svg class="w-6 h-6 text-emerald-600 dark:text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5 0a4 4 0 11-8 0 4 4 0 018 0z"></path>
                                    </svg>
                                    <h3 class="font-medium text-slate-800 dark:text-slate-200">{t("user_management_title", lang)}</h3>
                                </div>
                                <p class="text-sm text-slate-600 dark:text-slate-400 mb-3">
                                    {t("user_management", lang)}
                                </p>
                                <button 
                                    onclick={Callback::from(|_| {
                                        // Navigate to admin users page
                                        if let Some(window) = web_sys::window() {
                                            let _ = window.location().set_href("/admin/users");
                                        }
                                    })}
                                    class="w-full bg-emerald-500 hover:bg-emerald-600 text-white font-medium py-2 px-4 rounded-lg transition-colors"
                                >
                                    {t("manage_users", lang)}
                                </button>
                            </div>

                            // Recipe Management
                            <div class="bg-slate-50 dark:bg-slate-800 rounded-lg p-4 border border-slate-200 dark:border-slate-700">
                                <div class="flex items-center space-x-3 mb-3">
                                    <svg class="w-6 h-6 text-emerald-600 dark:text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path>
                                    </svg>
                                    <h3 class="font-medium text-slate-800 dark:text-slate-200">{t("recipe_management", lang)}</h3>
                                </div>
                                <p class="text-sm text-slate-600 dark:text-slate-400 mb-3">
                                    {t("view_delete_recipes", lang)}
                                </p>
                                <button 
                                    onclick={Callback::from(|_| {
                                        if let Some(window) = web_sys::window() {
                                            let _ = window.location().set_href("/admin/recipes");
                                        }
                                    })}
                                    class="w-full bg-emerald-500 hover:bg-emerald-600 text-white font-medium py-2 px-4 rounded-lg transition-colors"
                                >
                                    {t("manage_recipes", lang)}
                                </button>
                            </div>

                            // Category Management
                            <div class="bg-slate-50 dark:bg-slate-800 rounded-lg p-4 border border-slate-200 dark:border-slate-700">
                                <div class="flex items-center space-x-3 mb-3">
                                    <svg class="w-6 h-6 text-emerald-600 dark:text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"></path>
                                    </svg>
                                    <h3 class="font-medium text-slate-800 dark:text-slate-200">{t("category_management", lang)}</h3>
                                </div>
                                <p class="text-sm text-slate-600 dark:text-slate-400 mb-3">
                                    {t("manage_categories", lang)}
                                </p>
                                <button 
                                    onclick={Callback::from(|_| {
                                        if let Some(window) = web_sys::window() {
                                            let _ = window.location().set_href("/admin/categories");
                                        }
                                    })}
                                    class="w-full bg-emerald-500 hover:bg-emerald-600 text-white font-medium py-2 px-4 rounded-lg transition-colors"
                                >
                                    {t("manage_categories_button", lang)}
                                </button>
                            </div>

                            // System Stats
                            <div class="bg-slate-50 dark:bg-slate-800 rounded-lg p-4 border border-slate-200 dark:border-slate-700">
                                <div class="flex items-center space-x-3 mb-3">
                                    <svg class="w-6 h-6 text-emerald-600 dark:text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
                                    </svg>
                                    <h3 class="font-medium text-slate-800 dark:text-slate-200">{t("system_statistics", lang)}</h3>
                                </div>
                                <p class="text-sm text-slate-600 dark:text-slate-400 mb-3">
                                    {t("view_system_stats", lang)}
                                </p>
                                <button 
                                    onclick={Callback::from(|_| {
                                        if let Some(window) = web_sys::window() {
                                            let _ = window.location().set_href("/admin/stats");
                                        }
                                    })}
                                    class="w-full bg-emerald-500 hover:bg-emerald-600 text-white font-medium py-2 px-4 rounded-lg transition-colors"
                                >
                                    {t("view_stats", lang)}
                                </button>
                            </div>
                        </div>

                        <div class="mt-6 p-4 bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800 rounded-lg">
                            <div class="flex items-start space-x-3">
                                <svg class="w-5 h-5 text-amber-600 dark:text-amber-400 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
                                </svg>
                                <div>
                                    <h4 class="font-medium text-amber-800 dark:text-amber-200">{t("admin_privileges", lang)}</h4>
                                    <p class="text-sm text-amber-700 dark:text-amber-300 mt-1">
                                        {t("admin_privileges_desc", lang)}
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
