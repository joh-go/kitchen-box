use yew::prelude::*;
use crate::api;
use crate::i18n::{Language, t};
use crate::language_provider::LanguageState;

#[derive(Clone, Debug, PartialEq)]
pub struct AdminUser {
    pub id: i32,
    pub name: String,
    pub is_admin: bool,
    pub created_at: Option<String>,
}

impl AdminUser {
    pub fn from_value(user: &serde_json::Value) -> Option<Self> {
        Some(Self {
            id: user.get("id")?.as_i64()? as i32,
            name: user.get("name")?.as_str()?.to_string(),
            is_admin: user.get("is_admin")?.as_bool()?,
            created_at: user.get("created_at").and_then(|c| c.as_str()).map(|s| s.to_string()),
        })
    }
}

#[derive(Clone, Debug)]
enum UserAction {
    None,
    Create,
    Edit(i32),
    Delete(i32),
}

#[function_component(AdminUsersPage)]
pub fn admin_users_page() -> Html {
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

    let users = use_state(Vec::<AdminUser>::new);
    let loading = use_state(|| true);
    let action = use_state(|| UserAction::None);
    let form_name = use_state(|| String::new());
    let form_password = use_state(|| String::new());
    let form_is_admin = use_state(|| false);
    let form_error = use_state(|| None::<String>);
    let form_loading = use_state(|| false);

    // Load users
    {
        let users = users.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            let users = users.clone();
            let loading = loading.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::get_admin_users().await {
                    Ok(data) => {
                        if let Some(arr) = data.as_array() {
                            let list: Vec<AdminUser> = arr.iter().filter_map(AdminUser::from_value).collect();
                            users.set(list);
                        }
                        loading.set(false);
                    }
                    Err(_) => { loading.set(false); }
                }
            });
            || ()
        });
    }

    let refresh = {
        let users = users.clone();
        let loading = loading.clone();
        Callback::from(move |_| {
            let users = users.clone();
            let loading = loading.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                match api::get_admin_users().await {
                    Ok(data) => {
                        if let Some(arr) = data.as_array() {
                            let list: Vec<AdminUser> = arr.iter().filter_map(AdminUser::from_value).collect();
                            users.set(list);
                        }
                        loading.set(false);
                    }
                    Err(_) => { loading.set(false); }
                }
            });
        })
    };

    let open_create = {
        let action = action.clone();
        let form_name = form_name.clone();
        let form_password = form_password.clone();
        let form_is_admin = form_is_admin.clone();
        let form_error = form_error.clone();
        Callback::from(move |_| {
            form_name.set(String::new());
            form_password.set(String::new());
            form_is_admin.set(false);
            form_error.set(None);
            action.set(UserAction::Create);
        })
    };

    let open_edit = {
        let action = action.clone();
        let form_name = form_name.clone();
        let form_password = form_password.clone();
        let form_is_admin = form_is_admin.clone();
        let form_error = form_error.clone();
        let users2 = users.clone();
        Callback::from(move |user_id: i32| {
            if let Some(user) = (*users2).iter().find(|u| u.id == user_id) {
                form_name.set(user.name.clone());
                form_password.set(String::new());
                form_is_admin.set(user.is_admin);
                form_error.set(None);
                action.set(UserAction::Edit(user_id));
            }
        })
    };

    let submit_form = {
        let action = action.clone();
        let form_name = form_name.clone();
        let form_password = form_password.clone();
        let form_is_admin = form_is_admin.clone();
        let form_loading = form_loading.clone();
        let form_error = form_error.clone();
        let refresh = refresh.clone();
        Callback::from(move |_| {
            let name = (*form_name).clone();
            let password = (*form_password).clone();
            let is_admin = *form_is_admin;

            if name.trim().is_empty() {
                form_error.set(Some(t("name_required", lang).to_string()));
                return;
            }

            form_loading.set(true);
            form_error.set(None);

            let action = action.clone();
            let form_name = form_name.clone();
            let form_password = form_password.clone();
            let form_is_admin = form_is_admin.clone();
            let form_loading = form_loading.clone();
            let form_error2 = form_error.clone();
            let refresh = refresh.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match (*action).clone() {
                    UserAction::Create => {
                        let user_data = serde_json::json!({
                            "name": name,
                            "password": password,
                            "is_admin": is_admin,
                        });
                        match api::create_admin_user(user_data).await {
                            Ok(_) => {
                                form_name.set(String::new());
                                form_password.set(String::new());
                                form_is_admin.set(false);
                                form_loading.set(false);
                                action.set(UserAction::None);
                                refresh.emit(());
                            }
                            Err(e) => { form_error2.set(Some(e)); form_loading.set(false); }
                        }
                    }
                    UserAction::Edit(user_id) => {
                        let user_data = serde_json::json!({
                            "name": name,
                            "is_admin": is_admin,
                            "password": if password.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(password) },
                        });
                        match api::update_admin_user(user_id, user_data).await {
                            Ok(_) => {
                                form_name.set(String::new());
                                form_password.set(String::new());
                                form_is_admin.set(false);
                                form_loading.set(false);
                                action.set(UserAction::None);
                                refresh.emit(());
                            }
                            Err(e) => { form_error2.set(Some(e)); form_loading.set(false); }
                        }
                    }
                    _ => {}
                }
            });
        })
    };

    let confirm_delete = {
        let action = action.clone();
        let refresh = refresh.clone();
        Callback::from(move |user_id: i32| {
            let action = action.clone();
            let refresh = refresh.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = api::delete_admin_user(user_id).await;
                action.set(UserAction::None);
                refresh.emit(());
            });
        })
    };

    html! {
        <div class="page" style="padding: var(--space-6);">
            <div class="section-header">
                <h1 class="section-title">{t("user_management_title", lang)}</h1>
                <button class="btn btn-primary btn-sm" onclick={open_create}>{t("add_user", lang)}</button>
            </div>

            {if *loading {
                html! {
                    <div class="card" style="padding: var(--space-6);">
                        {for (0..3).map(|_| html! {
                            <div class="skeleton" style="height: 40px; margin-bottom: 8px;" />
                        })}
                    </div>
                }
            } else {
                html! {
                    <div class="table-container">
                        <table class="table-card-view">
                            <thead>
                                <tr>
                                    <th>{t("name", lang)}</th>
                                    <th>{t("role_column", lang)}</th>
                                    <th>{t("actions_column", lang)}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {for users.iter().map(|user| {
                                    let user_id = user.id;
                                    let on_edit = open_edit.clone();
                                    let on_delete = {
                                        let action = action.clone();
                                        Callback::from(move |_| action.set(UserAction::Delete(user_id)))
                                    };
                                    html! {
                                        <tr>
                                            <td data-label={t("name", lang)} style="font-weight: 500;">
                                                <span>{&user.name}</span>
                                            </td>
                                            <td data-label={t("role_column", lang)}>
                                                {if user.is_admin {
                                                    html! { <span class="badge badge-primary">{t("admin", lang)}</span> }
                                                } else {
                                                    html! { <span class="badge badge-default">{t("user", lang)}</span> }
                                                }}
                                            </td>
                                            <td data-label={t("actions_column", lang)}>
                                                <div style="display: flex; gap: 0.5rem;">
                                                    <button class="btn btn-sm btn-outline" onclick={let uid = user_id; move |_| on_edit.emit(uid)}>
                                                        {t("edit", lang)}
                                                    </button>
                                                    <button class="btn btn-sm btn-danger" onclick={on_delete}>
                                                        {t("delete", lang)}
                                                    </button>
                                                </div>
                                            </td>
                                        </tr>
                                    }
                                })}
                            </tbody>
                        </table>
                    </div>
                }
            }}

            // Create/Edit Modal
            {if matches!(*action, UserAction::Create | UserAction::Edit(_)) {
                let is_edit = matches!(*action, UserAction::Edit(_));
                html! {
                    <div class="modal-overlay" onclick={let a = action.clone(); Callback::from(move |_: MouseEvent| a.set(UserAction::None))}>
                        <div class="modal" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                            <div class="modal-header">
                                <h3 class="modal-title">
                                    {if is_edit { t("edit_user_title", lang) } else { t("create_user_title", lang) }}
                                </h3></div>
                            <div class="modal-body">
                                {if let Some(ref err) = &*form_error {
                                    html! {
                                        <div class="alert alert-error" style="margin-bottom: 1rem;">
                                            <span>{err}</span>
                                        </div>
                                    }
                                } else { html! {} }}

                                <div class="form-group">
                                    <label class="form-label">{t("name", lang)}</label>
                                    <input type="text" class="form-input" value={(*form_name).clone()}
                                        oninput={let s = form_name.clone(); Callback::from(move |e: InputEvent| {
                                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                            s.set(input.value());
                                        })}
                                    />
                                </div>
                                <div class="form-group">
                                    <label class="form-label">{t("password", lang)}</label>
                                    <input type="password" class="form-input" value={(*form_password).clone()}
                                        placeholder={if is_edit { t("leave_blank_keep_password", lang) } else { t("enter_password", lang) }}
                                        oninput={let s = form_password.clone(); Callback::from(move |e: InputEvent| {
                                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                            s.set(input.value());
                                        })}
                                    />
                                </div>
                                <div style="display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1rem;">
                                    <label class="toggle">
                                        <input type="checkbox" checked={*form_is_admin}
                                            onchange={let s = form_is_admin.clone(); Callback::from(move |e: Event| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                s.set(input.checked());
                                            })}
                                        />
                                        <span class="toggle-slider"></span>
                                        <span style="font-size: 0.875rem; font-weight: 500; color: var(--gray-700);">{t("administrator_label", lang)}</span>
                                    </label>
                                </div>
                                <div class="form-actions" style="margin-top: 0;">
                                    <button class="btn btn-ghost" onclick={let a = action.clone(); Callback::from(move |_: MouseEvent| a.set(UserAction::None))}>{t("cancel", lang)}</button>
                                    <button class="btn btn-primary" onclick={submit_form} disabled={*form_loading}>
                                        {if *form_loading { t("saving", lang) } else if is_edit { t("save", lang) } else { t("create_user_title", lang) }}
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            } else { html! {} }}

            // Delete Confirmation Modal
            {if let UserAction::Delete(user_id) = (*action).clone() {
                let user_name = users.iter().find(|u| u.id == user_id).map(|u| u.name.clone()).unwrap_or_default();
                html! {
                    <div class="modal-overlay" onclick={let a = action.clone(); Callback::from(move |_: MouseEvent| a.set(UserAction::None))}>
                        <div class="modal" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                            <div class="modal-body" style="text-align: center;">
                                <h3 class="modal-title" style="margin-bottom: 1rem;">{t("delete_user_title", lang)}</h3>
                                <p style="margin-bottom: 1.5rem; color: var(--gray-600); font-size: 0.9375rem;">
                                    {if user_name.is_empty() {
                                        html! { t("delete_user_confirmation", lang) }
                                    } else {
                                        html! { format!("{} {}", t("delete_user_confirmation", lang), user_name) }
                                    }}
                                </p>
                                <div class="form-actions" style="margin-top: 0; justify-content: center;">
                                    <button class="btn btn-ghost" onclick={let a = action.clone(); Callback::from(move |_: MouseEvent| a.set(UserAction::None))}>{t("cancel", lang)}</button>
                                    <button class="btn btn-danger" onclick={let id = user_id; move |_| confirm_delete.emit(id)}>{t("delete", lang)}</button>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            } else { html! {} }}
        </div>
    }
}
