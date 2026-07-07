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
                        Err(_) => {}
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
                    "name" => { state.set(SettingsState { name: value, ..state.deref().clone() }); }
                    "email" => { state.set(SettingsState { email: value, ..state.deref().clone() }); }
                    "current_password" => { state.set(SettingsState { current_password: value, ..state.deref().clone() }); }
                    "new_password" => { state.set(SettingsState { new_password: value, ..state.deref().clone() }); }
                    "confirm_password" => { state.set(SettingsState { confirm_password: value, ..state.deref().clone() }); }
                    _ => {}
                }
            }
        })
    };

    html! {
        <div class="settings-section">
            <div class="page-enter">
                <h1 class="page-title">{t("account_settings", lang)}</h1>
                <p class="text-muted">{t("manage_profile_password", lang)}</p>
            </div>

            <div class="settings-card page-enter">
                <div class="settings-card-body">
                    <form class="flex flex-col gap-6" onsubmit={onsubmit}>
                        {if let Some(ref error) = state.deref().error {
                            html! {
                                <div class="settings-message settings-message-error">{error}</div>
                            }
                        } else { html!{} }}

                        {if let Some(ref success) = state.deref().success {
                            html! {
                                <div class="settings-message settings-message-success">{success}</div>
                            }
                        } else { html!{} }}

                        <div>
                            <h3 class="section-title mb-4">{t("profile_information", lang)}</h3>
                            <div class="flex flex-col gap-4">
                                <div class="form-group">
                                    <label for="name" class="form-label">{t("display_name", lang)}</label>
                                    <input
                                        id="name" name="name" type="text"
                                        class="form-input"
                                        placeholder={t("enter_display_name", lang)}
                                        value={state.name.clone()}
                                        oninput={oninput.clone()}
                                    />
                                </div>
                                <div class="form-group">
                                    <label for="email" class="form-label">{t("email_address", lang)}</label>
                                    <input
                                        id="email" name="email" type="email"
                                        class="form-input"
                                        placeholder={t("enter_email", lang)}
                                        value={state.email.clone()}
                                        oninput={oninput.clone()}
                                    />
                                </div>
                            </div>
                        </div>

                        <div style="border-top: 1px solid var(--gray-200); padding-top: var(--space-6);">
                            <h3 class="section-title mb-4">{t("language", lang)}</h3>
                            <div class="form-group">
                                <label class="form-label">{t("language", lang)}</label>
                                <select class="form-select"
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

                        <div style="border-top: 1px solid var(--gray-200); padding-top: var(--space-6);">
                            <h3 class="section-title mb-4">{t("change_password", lang)}</h3>
                            <div class="flex flex-col gap-4">
                                <div class="form-group">
                                    <label for="current_password" class="form-label">{t("current_password_label", lang)}</label>
                                    <input id="current_password" name="current_password" type="password" class="form-input"
                                        placeholder={t("enter_current_password", lang)}
                                        value={state.current_password.clone()} oninput={oninput.clone()} />
                                </div>
                                <div class="form-group">
                                    <label for="new_password" class="form-label">{t("new_password_label", lang)}</label>
                                    <input id="new_password" name="new_password" type="password" class="form-input"
                                        placeholder={t("enter_new_password", lang)}
                                        value={state.new_password.clone()} oninput={oninput.clone()} />
                                </div>
                                <div class="form-group">
                                    <label for="confirm_password" class="form-label">{t("confirm_new_password", lang)}</label>
                                    <input id="confirm_password" name="confirm_password" type="password" class="form-input"
                                        placeholder={t("confirm_new_password", lang)}
                                        value={state.confirm_password.clone()} oninput={oninput} />
                                </div>
                            </div>
                        </div>

                        <div class="flex justify-end">
                            <button type="submit" disabled={state.loading} class="btn btn-primary">
                                {if state.loading {
                                    html! { <>{t("saving", lang)}</> }
                                } else {
                                    html! {t("save_changes", lang)}
                                }}
                            </button>
                        </div>
                    </form>
                </div>
            </div>

            {if api::is_current_user_admin() {
                html! {
                    <div class="settings-card page-enter">
                        <div class="settings-card-header">
                            <h3>{t("admin_panel", lang)}</h3>
                        </div>
                        <div class="settings-card-body">
                            <div class="settings-admin-grid">
                                <div class="card" style="padding: var(--space-4);">
                                    <h4 class="font-mono text-sm text-muted mb-2">{t("user_management_title", lang)}</h4>
                                    <p class="text-sm text-muted mb-3">{t("user_management", lang)}</p>
                                    <button onclick={Callback::from(|_| { if let Some(w) = web_sys::window() { let _ = w.location().set_href("/admin/users"); } })}
                                        class="btn btn-primary btn-sm w-full">{t("manage_users", lang)}</button>
                                </div>
                                <div class="card" style="padding: var(--space-4);">
                                    <h4 class="font-mono text-sm text-muted mb-2">{t("recipe_management", lang)}</h4>
                                    <p class="text-sm text-muted mb-3">{t("view_delete_recipes", lang)}</p>
                                    <button onclick={Callback::from(|_| { if let Some(w) = web_sys::window() { let _ = w.location().set_href("/admin/recipes"); } })}
                                        class="btn btn-primary btn-sm w-full">{t("manage_recipes", lang)}</button>
                                </div>
                                <div class="card" style="padding: var(--space-4);">
                                    <h4 class="font-mono text-sm text-muted mb-2">{t("category_management", lang)}</h4>
                                    <p class="text-sm text-muted mb-3">{t("manage_categories", lang)}</p>
                                    <button onclick={Callback::from(|_| { if let Some(w) = web_sys::window() { let _ = w.location().set_href("/admin/categories"); } })}
                                        class="btn btn-primary btn-sm w-full">{t("manage_categories_button", lang)}</button>
                                </div>
                                <div class="card" style="padding: var(--space-4);">
                                    <h4 class="font-mono text-sm text-muted mb-2">{t("system_statistics", lang)}</h4>
                                    <p class="text-sm text-muted mb-3">{t("view_system_stats", lang)}</p>
                                    <button class="btn btn-primary btn-sm w-full">{t("view_stats", lang)}</button>
                                </div>
                            </div>

                            <div class="alert alert-warning mt-4">
                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
                                </svg>
                                <div class="alert-content">
                                    <div class="alert-title">{t("admin_privileges", lang)}</div>
                                    <div>{t("admin_privileges_desc", lang)}</div>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            } else { html!{} }}
        </div>
    }
}
