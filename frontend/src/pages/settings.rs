use yew::prelude::*;
use yew::{function_component, html, use_state, use_effect_with};
use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;
use home_hub_shared::components::settings::{AppearanceSection, DangerZone, PasswordSection, ProfileSection};
use home_hub_shared::prefs::UserPrefs;
use home_hub_shared::components::Modal;
use home_hub_shared::icons::{Icon, IconComponent};
use crate::api;
use crate::i18n::{Language, t};
use crate::language_provider::LanguageState;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct SettingsState {
    pub name: String,
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
            current_password: String::new(),
            new_password: String::new(),
            confirm_password: String::new(),
            loading: false,
            error: None,
            success: None,
        }
    }
}

#[derive(Clone, PartialEq)]
enum AdminSection {
    Overview,
    Users,
    Recipes,
    Categories,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub theme_revision: u32,
    #[prop_or_default]
    pub on_theme_changed: Callback<()>,
}

#[function_component(SettingsPage)]
pub fn settings(props: &Props) -> Html {
    let state = use_state(SettingsState::default);
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);
    let selected_tab = use_state(|| "profile".to_string());
    let admin_section = use_state(|| AdminSection::Overview);
    let confirm_delete = use_state(|| false);
    let delete_loading = use_state(|| false);
    let user_prefs = use_state(|| UserPrefs::load());

    {
        let up = user_prefs.clone();
        let rev = props.theme_revision;
        use_effect_with(rev, move |_| {
            up.set(UserPrefs::load());
            || ()
        });
    }

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
                            state.set(SettingsState {
                                name,
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
                let current_password = state.deref().current_password.clone();
                let new_password = state.deref().new_password.clone();

                match api::update_profile(&name, &current_password, &new_password).await {
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
                    "current_password" => { state.set(SettingsState { current_password: value, ..state.deref().clone() }); }
                    "new_password" => { state.set(SettingsState { new_password: value, ..state.deref().clone() }); }
                    "confirm_password" => { state.set(SettingsState { confirm_password: value, ..state.deref().clone() }); }
                    _ => {}
                }
            }
        })
    };

    // Delete account handler
    let delete_account = {
        let delete_loading = delete_loading.clone();
        let confirm_delete = confirm_delete.clone();
        let add_toast = Callback::from(move |msg: String| {
            web_sys::console::log_1(&msg.into());
        });

        Callback::from(move |_| {
            let user_id = api::get_current_user_id().unwrap_or(0);
            if user_id == 0 {
                return;
            }

            delete_loading.set(true);

            let delete_loading = delete_loading.clone();
            let confirm_delete = confirm_delete.clone();

            spawn_local(async move {
                match api::delete_my_account(user_id).await {
                    Ok(_) => {
                        home_hub_shared::check_auth_error("401".to_string());
                    }
                    Err(e) => {
                        delete_loading.set(false);
                        confirm_delete.set(false);
                        web_sys::console::log_1(&format!("Fehler: {}", e).into());
                    }
                }
            });
        })
    };

    html! {
        <div class="page settings-page">
            <div class="page-header">
                <h1>{t("account_settings", lang)}</h1>
            </div>

            <div class="chart-tabs">
                <button class={if *selected_tab == "profile" { "active" } else { "" }}
                    onclick={let s = selected_tab.clone(); Callback::from(move |_| s.set("profile".to_string()))}>
                    {"Profil"}
                </button>
                <button class={if *selected_tab == "appearance" { "active" } else { "" }}
                    onclick={let s = selected_tab.clone(); Callback::from(move |_| s.set("appearance".to_string()))}>
                    {"Darstellung"}
                </button>
                <button class={if *selected_tab == "admin" { "active" } else { "" }}
                    onclick={let s = selected_tab.clone(); let a = admin_section.clone(); Callback::from(move |_| { s.set("admin".to_string()); a.set(AdminSection::Overview); })}>
                    {"Admin"}
                </button>
            </div>

            {if *selected_tab == "profile" {
                html! {
                    <>
                        <ProfileSection
                            current_name={(*state).name.clone()}
                            on_save={let st = state.clone(); Callback::from(move |new_name: String| {
                                if new_name.trim().is_empty() { return; }
                                let st = st.clone();
                                spawn_local(async move {
                                    match api::update_profile(&new_name, "", "").await {
                                        Ok(_) => {
                                            if let Some(window) = web_sys::window() {
                                                if let Ok(Some(storage)) = window.local_storage() {
                                                    let _ = storage.set_item("user_name", &new_name);
                                                }
                                            }
                                            st.set(SettingsState { success: Some(t("profile_updated", lang)), ..(*st).clone() });
                                        }
                                        Err(e) => { st.set(SettingsState { error: Some(e), ..(*st).clone() }); }
                                    }
                                });
                            })}
                            saving={false}
                        />

                        <div class="card-section">
                            <h2>{t("language", lang)}</h2>
                            <div class="form-group">
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

                        <PasswordSection
                            on_save={let st = state.clone(); Callback::from(move |(current, new_pw, confirm): (String, String, String)| {
                                if new_pw != confirm {
                                    st.set(SettingsState { error: Some(t("passwords_do_not_match", lang)), ..(*st).clone() });
                                    return;
                                }
                                if current.is_empty() {
                                    st.set(SettingsState { error: Some(t("current_password_required", lang)), ..(*st).clone() });
                                    return;
                                }
                                let name = (*st).name.clone();
                                let st = st.clone();
                                spawn_local(async move {
                                    match api::update_profile(&name, &current, &new_pw).await {
                                        Ok(_) => {
                                            st.set(SettingsState {
                                                loading: false,
                                                success: Some(t("profile_updated", lang)),
                                                current_password: String::new(),
                                                new_password: String::new(),
                                                confirm_password: String::new(),
                                                ..(*st).clone()
                                            });
                                        }
                                        Err(e) => { st.set(SettingsState { loading: false, error: Some(e), ..(*st).clone() }); }
                                    }
                                });
                            })}
                            saving={false}
                        />

                        <DangerZone
                            on_delete={
                                let dl = delete_loading.clone();
                                Callback::from(move |_: ()| {
                                    let user_id = api::get_current_user_id().unwrap_or(0);
                                    if user_id == 0 { return; }
                                    dl.set(true);
                                    let dl2 = dl.clone();
                                    spawn_local(async move {
                                        match api::delete_my_account(user_id).await {
                                            Ok(_) => { home_hub_shared::check_auth_error("401".to_string()); }
                                            Err(e) => { dl2.set(false); web_sys::console::log_1(&format!("Fehler: {}", e).into()); }
                                        }
                                    });
                                })
                            }
                            delete_label={"Konto löschen".to_string()}
                            confirm_title={"Konto löschen?".to_string()}
                            confirm_message={"Wenn Sie Ihr Konto löschen, werden alle Ihre Daten unwiderruflich entfernt.".to_string()}
                            deleting={*delete_loading}
                        />
                    </>
                }
            } else if *selected_tab == "appearance" {
                html! {
                    <AppearanceSection
                        prefs={(*user_prefs).clone()}
                        on_prefs_change={let up = user_prefs.clone(); let oc = props.on_theme_changed.clone(); Callback::from(move |p: UserPrefs| {
                            let old_theme = up.theme.clone();
                            let theme_changed = p.theme != old_theme;
                            up.set(p);
                            if theme_changed {
                                oc.emit(());
                            }
                        })}
                        lang_ctx={lang_ctx.clone()}
                    />
                }
            } else if *admin_section == AdminSection::Overview {
                html! {
                    <div class="card-section">
                        <h2>{"Admin Panel"}</h2>
                        <div class="settings-admin-grid" style="display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 1rem; margin-top: 1rem;">
                            <div class="card" style="padding: 1rem; cursor: pointer;" onclick={let a = admin_section.clone(); Callback::from(move |_: MouseEvent| a.set(AdminSection::Users))}>
                                <h4 style="font-size: 0.875rem; font-weight: 600; margin-bottom: 0.5rem;">{t("user_management_title", lang)}</h4>
                                <p class="text-sm text-muted" style="margin-bottom: 0.75rem;">{t("user_management", lang)}</p>
                                <span class="btn btn-primary btn-sm w-full" style="display: inline-block; text-align: center;">{t("manage_users", lang)}</span>
                            </div>
                            <div class="card" style="padding: 1rem; cursor: pointer;" onclick={let a = admin_section.clone(); Callback::from(move |_: MouseEvent| a.set(AdminSection::Recipes))}>
                                <h4 style="font-size: 0.875rem; font-weight: 600; margin-bottom: 0.5rem;">{t("recipe_management", lang)}</h4>
                                <p class="text-sm text-muted" style="margin-bottom: 0.75rem;">{"Rezepte verwalten"}</p>
                                <span class="btn btn-primary btn-sm w-full" style="display: inline-block; text-align: center;">{t("manage_recipes", lang)}</span>
                            </div>
                            <div class="card" style="padding: 1rem; cursor: pointer;" onclick={let a = admin_section.clone(); Callback::from(move |_: MouseEvent| a.set(AdminSection::Categories))}>
                                <h4 style="font-size: 0.875rem; font-weight: 600; margin-bottom: 0.5rem;">{t("category_management", lang)}</h4>
                                <p class="text-sm text-muted" style="margin-bottom: 0.75rem;">{"Kategorien verwalten"}</p>
                                <span class="btn btn-primary btn-sm w-full" style="display: inline-block; text-align: center;">{t("manage_categories_button", lang)}</span>
                            </div>
                        </div>
                    </div>
                }
            } else if *admin_section == AdminSection::Users {
                html! {
                    <div class="card-section">
                        <div style="display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1rem;">
                            <button class="btn btn-ghost btn-sm" onclick={let a = admin_section.clone(); Callback::from(move |_: MouseEvent| a.set(AdminSection::Overview))}>
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="19" y1="12" x2="5" y2="12"/><polyline points="12 19 5 12 12 5"/></svg>
                                {" Zurück"}
                            </button>
                            <h2 style="margin: 0;">{t("user_management_title", lang)}</h2>
                        </div>
                        <crate::pages::admin_users::AdminUsersPage />
                    </div>
                }
            } else if *admin_section == AdminSection::Recipes {
                html! {
                    <div class="card-section">
                        <div style="display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1rem;">
                            <button class="btn btn-ghost btn-sm" onclick={let a = admin_section.clone(); Callback::from(move |_: MouseEvent| a.set(AdminSection::Overview))}>
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="19" y1="12" x2="5" y2="12"/><polyline points="12 19 5 12 12 5"/></svg>
                                {" Zurück"}
                            </button>
                            <h2 style="margin: 0;">{t("recipe_management_title", lang)}</h2>
                        </div>
                        <crate::pages::admin_recipes::AdminRecipesPage />
                    </div>
                }
            } else if *admin_section == AdminSection::Categories {
                html! {
                    <div class="card-section">
                        <div style="display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1rem;">
                            <button class="btn btn-ghost btn-sm" onclick={let a = admin_section.clone(); Callback::from(move |_: MouseEvent| a.set(AdminSection::Overview))}>
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="19" y1="12" x2="5" y2="12"/><polyline points="12 19 5 12 12 5"/></svg>
                                {" Zurück"}
                            </button>
                            <h2 style="margin: 0;">{t("category_management_title", lang)}</h2>
                        </div>
                        <crate::pages::admin_categories::AdminCategoriesPage />
                    </div>
                }
            } else {
                html! {}
            }}

            <Modal
                title="Konto löschen?"
                show={*confirm_delete}
                on_close={let c = confirm_delete.clone(); Callback::from(move |_: ()| c.set(false))}
            >
                <p style="margin: 0 0 1.5rem; color: var(--gray-600); font-size: 0.9375rem; line-height: 1.5;">
                    {"Soll Ihr Konto wirklich gelöscht werden? Alle Ihre Daten werden unwiderruflich entfernt."}
                </p>
                <div class="form-actions" style="margin-top: 0;">
                    <button class="btn btn-ghost" onclick={let c = confirm_delete.clone(); Callback::from(move |_: MouseEvent| c.set(false))}>{"Abbrechen"}</button>
                    <button class="btn btn-danger" onclick={delete_account} disabled={*delete_loading}>
                        <IconComponent kind={Icon::Delete} size={18} color="#ffffff" />
                        {if *delete_loading { "Lösche..." } else { "Ja, Konto löschen" }}
                    </button>
                </div>
            </Modal>
        </div>
    }
}
