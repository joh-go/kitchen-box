use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use crate::i18n::{Language, t};
use crate::language_provider::LanguageState;

#[derive(Clone, Debug, PartialEq)]
pub struct AdminUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub created_at: String,
}

#[derive(Clone, Debug)]
pub enum UserAction {
    None,
    Create,
    Edit(i32),
    Delete(i32),
}

#[function_component(AdminUsersPage)]
pub fn admin_users_page() -> Html {
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

    let users = use_state(|| Vec::<AdminUser>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let action = use_state(|| UserAction::None);
    let show_create_form = use_state(|| false);
    let selected_user = use_state(|| None::<AdminUser>);

    let form_name = use_state(|| String::new());
    let form_email = use_state(|| String::new());
    let form_password = use_state(|| String::new());
    let form_is_admin = use_state(|| false);
    let form_loading = use_state(|| false);
    let form_error = use_state(|| None::<String>);

    {
        let users = users.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                match api::get_admin_users().await {
                    Ok(response) => {
                        if let Some(users_data) = response.get("users").and_then(|u| u.as_array()) {
                            let parsed_users: Vec<AdminUser> = users_data.iter().filter_map(|user| {
                                Some(AdminUser {
                                    id: user.get("id")?.as_i64()? as i32,
                                    name: user.get("name")?.as_str()?.to_string(),
                                    email: user.get("email")?.as_str()?.to_string(),
                                    is_admin: user.get("is_admin")?.as_bool().unwrap_or(false),
                                    created_at: user.get("created_at")?.as_str()?.to_string(),
                                })
                            }).collect();
                            users.set(parsed_users);
                        } else {
                            error.set(Some(t("failed_parse_users", lang).to_string()));
                        }
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    let on_create_user = {
        let show_create_form = show_create_form.clone();
        let action = action.clone();
        Callback::from(move |_| {
            show_create_form.set(true);
            action.set(UserAction::Create);
        })
    };

    let on_user_click = {
        let selected_user = selected_user.clone();
        Callback::from(move |user: AdminUser| {
            if let Some(ref selected) = *selected_user {
                if selected.id == user.id {
                    selected_user.set(None);
                } else {
                    selected_user.set(Some(user.clone()));
                }
            } else {
                selected_user.set(Some(user.clone()));
            }
        })
    };

    let on_edit_user = {
        let action = action.clone();
        let show_create_form = show_create_form.clone();
        let users = users.clone();
        let form_name = form_name.clone();
        let form_email = form_email.clone();
        let form_password = form_password.clone();
        let form_is_admin = form_is_admin.clone();
        let form_error = form_error.clone();
        Callback::from(move |user_id: i32| {
            if let Some(user) = (*users).iter().find(|u| u.id == user_id) {
                form_name.set(user.name.clone());
                form_email.set(user.email.clone());
                form_password.set(String::new());
                form_is_admin.set(user.is_admin);
                form_error.set(None);
                action.set(UserAction::Edit(user_id));
                show_create_form.set(true);
            }
        })
    };

    let on_delete_user = {
        let action = action.clone();
        Callback::from(move |user_id: i32| {
            action.set(UserAction::Delete(user_id));
        })
    };

    let on_cancel_action = {
        let action = action.clone();
        let show_create_form = show_create_form.clone();
        let form_name = form_name.clone();
        let form_email = form_email.clone();
        let form_password = form_password.clone();
        let form_is_admin = form_is_admin.clone();
        let form_error = form_error.clone();

        Callback::from(move |_| {
            action.set(UserAction::None);
            show_create_form.set(false);
            form_name.set(String::new());
            form_email.set(String::new());
            form_password.set(String::new());
            form_is_admin.set(false);
            form_error.set(None);
        })
    };

    let on_form_submit = {
        let form_name = form_name.clone();
        let form_email = form_email.clone();
        let form_password = form_password.clone();
        let form_is_admin = form_is_admin.clone();
        let form_loading = form_loading.clone();
        let form_error = form_error.clone();
        let users = users.clone();
        let action = action.clone();
        let show_create_form = show_create_form.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let name = (*form_name).clone();
            let email = (*form_email).clone();
            let password = (*form_password).clone();
            let is_admin = *form_is_admin;
            let current_action = (*action).clone();

            if name.trim().is_empty() {
                form_error.set(Some(t("name_required", lang).to_string()));
                return;
            }
            if email.trim().is_empty() {
                form_error.set(Some(t("email_required", lang).to_string()));
                return;
            }
            if password.trim().is_empty() && matches!(current_action, UserAction::Create) {
                form_error.set(Some(t("password_required_new_users", lang).to_string()));
                return;
            }

            form_loading.set(true);
            form_error.set(None);

            let users_clone = users.clone();
            let action_clone = action.clone();
            let show_create_form_clone = show_create_form.clone();
            let form_loading_clone = form_loading.clone();
            let form_name_clone = form_name.clone();
            let form_email_clone = form_email.clone();
            let form_password_clone = form_password.clone();
            let form_is_admin_clone = form_is_admin.clone();
            let form_error_clone = form_error.clone();

            spawn_local(async move {
                let result = match current_action {
                    UserAction::Create => {
                        let user_data = serde_json::json!({
                            "name": name,
                            "email": email,
                            "password": password,
                            "is_admin": is_admin
                        });
                        api::create_admin_user(user_data).await
                    }
                    UserAction::Edit(user_id) => {
                        if password.is_empty() {
                            let user_data = serde_json::json!({
                                "name": Some(name),
                                "email": Some(email),
                                "password": None::<String>,
                                "is_admin": Some(is_admin)
                            });
                            api::update_admin_user(user_id, user_data).await
                        } else {
                            let user_data = serde_json::json!({
                                "name": Some(name),
                                "email": Some(email),
                                "password": Some(password),
                                "is_admin": Some(is_admin)
                            });
                            api::update_admin_user(user_id, user_data).await
                        }
                    }
                    _ => Err(t("invalid_action", lang).to_string()),
                };

                match result {
                    Ok(_) => {
                        match api::get_admin_users().await {
                            Ok(response) => {
                                if let Some(users_data) = response.get("users").and_then(|u| u.as_array()) {
                                    let parsed_users: Vec<AdminUser> = users_data.iter().filter_map(|user| {
                                        Some(AdminUser {
                                            id: user.get("id")?.as_i64()? as i32,
                                            name: user.get("name")?.as_str()?.to_string(),
                                            email: user.get("email")?.as_str()?.to_string(),
                                            is_admin: user.get("is_admin")?.as_bool().unwrap_or(false),
                                            created_at: user.get("created_at")?.as_str()?.to_string(),
                                        })
                                    }).collect();
                                    users_clone.set(parsed_users);
                                }
                            }
                            Err(_) => {}
                        }

                        action_clone.set(UserAction::None);
                        show_create_form_clone.set(false);
                        form_name_clone.set(String::new());
                        form_email_clone.set(String::new());
                        form_password_clone.set(String::new());
                        form_is_admin_clone.set(false);
                        form_loading_clone.set(false);
                    }
                    Err(e) => {
                        form_error_clone.set(Some(e));
                        form_loading_clone.set(false);
                    }
                }
            });
        })
    };

    let on_confirm_delete = {
        let users = users.clone();
        let action = action.clone();
        let user_id = if let UserAction::Delete(id) = *action { id } else { 0 };

        Callback::from(move |_| {
            let users = users.clone();
            let action = action.clone();
            let user_id = user_id;

            spawn_local(async move {
                match api::delete_admin_user(user_id).await {
                    Ok(_) => {
                        let updated: Vec<AdminUser> = (*users).clone().into_iter().filter(|u| u.id != user_id).collect();
                        users.set(updated);
                        action.set(UserAction::None);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Failed to delete user: {}", e).into());
                    }
                }
            });
        })
    };

    html! {
        <div class="page-enter">
            <div class="flex items-center justify-between mb-6">
                <div>
                    <h1 class="section-title">{t("user_management_title", lang)}</h1>
                    <p class="text-muted">{t("manage_user_accounts_permissions", lang)}</p>
                </div>
                <button onclick={on_create_user} class="btn btn-primary">{t("add_user", lang)}</button>
            </div>

            {if *loading {
                html! { <div class="spinner"><div class="spinner-circle"></div></div> }
            } else if let Some(ref error_msg) = *error {
                html! { <div class="alert alert-error"><div class="alert-content">{error_msg}</div></div> }
            } else {
                html! {
                    <>
                        <div class="table-container card">
                            <table>
                                <thead>
                                    <tr>
                                        <th>{t("user_column", lang)}</th>
                                        <th class="hide-mobile">{t("role_column", lang)}</th>
                                        <th class="text-right">{t("actions_column", lang)}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {for (*users).iter().map(|user| {
                                        let on_edit = on_edit_user.clone();
                                        let on_delete = on_delete_user.clone();
                                        let on_click = on_user_click.clone();
                                        let user_clone = user.clone();
                                        let user_id = user.id;
                                        let is_selected = selected_user.as_ref().map(|u| u.id).unwrap_or(0) == user_id;

                                        html! {
                                            <>
                                                <tr class={if is_selected { "row-selected" } else { "" }}>
                                                    <td onclick={Callback::from(move |_| on_click.emit(user_clone.clone()))}>
                                                        <div class="flex items-center gap-3">
                                                            <div class="avatar avatar-primary">
                                                                {&user.name.chars().next().unwrap_or('U').to_uppercase().to_string()}
                                                            </div>
                                                            <div>
                                                                <div class="text-sm font-medium">{&user.name}</div>
                                                                <div class="text-xs text-muted hide-desktop">{if user.is_admin { t("admin", lang) } else { t("user", lang) }}</div>
                                                            </div>
                                                        </div>
                                                    </td>
                                                    <td class="hide-mobile">
                                                        {if user.is_admin {
                                                            html! { <span class="badge badge-success">{t("admin", lang)}</span> }
                                                        } else {
                                                            html! { <span class="badge">{t("user", lang)}</span> }
                                                        }}
                                                    </td>
                                                    <td class="text-right">
                                                        <button onclick={Callback::from(move |_| on_edit.emit(user_id))} class="btn-icon btn-sm mr-2">
                                                            <svg width="16" height="16" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-11h-1z"></path>
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 11l3 3L22 9l-3-3"></path>
                                                            </svg>
                                                        </button>
                                                        <button onclick={Callback::from(move |_| on_delete.emit(user_id))} class="btn-icon btn-sm">
                                                            <svg width="16" height="16" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6M4 7h16"></path>
                                                            </svg>
                                                        </button>
                                                    </td>
                                                </tr>
                                                {if is_selected {
                                                    html! {
                                                        <tr class="table-expanded-row"><td colspan="3">
                                                            <div class="table-expanded-content">
                                                                <div class="table-expanded-label">{"Email: "}<span class="table-expanded-value">{&user.email}</span></div>
                                                                <div class="table-expanded-label">{"Created: "}<span class="table-expanded-value">{&user.created_at}</span></div>
                                                            </div>
                                                        </td></tr>
                                                    }
                                                } else {
                                                    html! {}
                                                }}
                                            </>
                                        }
                                    })}
                                    {if (*users).is_empty() {
                                        html! { <tr><td colspan="3" class="table-empty">{t("no_users", lang)}</td></tr> }
                                    } else {
                                        html! {}
                                    }}
                                </tbody>
                            </table>
                        </div>

                        {if *show_create_form {
                            html! {
                                <div class="modal-overlay">
                                    <div class="modal modal-sm">
                                        <div class="modal-header">
                                            <h3 class="modal-title">{if matches!(*action, UserAction::Edit(_)) { t("edit_user_title", lang) } else { t("create_user_title", lang) }}</h3>
                                            <button onclick={on_cancel_action.clone()} class="modal-close">
                                                <svg width="20" height="20" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                                </svg>
                                            </button>
                                        </div>
                                        <div class="modal-body">
                                            {if let Some(ref error_msg) = *form_error {
                                                html! { <div class="alert alert-error mb-4"><div class="alert-content">{error_msg}</div></div> }
                                            } else {
                                                html! {}
                                            }}
                                            <form onsubmit={on_form_submit.clone()}>
                                                <div class="form-group">
                                                    <label class="form-label">{t("name", lang)}</label>
                                                    <input type="text" value={(*form_name).clone()} oninput={Callback::from(move |e: yew::InputEvent| { let input = e.target_unchecked_into::<web_sys::HtmlInputElement>(); form_name.set(input.value()); })} class="form-input" required=true />
                                                </div>
                                                <div class="form-group">
                                                    <label class="form-label">{t("email", lang)}</label>
                                                    <input type="email" value={(*form_email).clone()} oninput={Callback::from(move |e: yew::InputEvent| { let input = e.target_unchecked_into::<web_sys::HtmlInputElement>(); form_email.set(input.value()); })} class="form-input" required=true />
                                                </div>
                                                <div class="form-group">
                                                    <label class="form-label">{t("password", lang)}</label>
                                                    <input type="password" value={(*form_password).clone()} oninput={Callback::from(move |e: yew::InputEvent| { let input = e.target_unchecked_into::<web_sys::HtmlInputElement>(); form_password.set(input.value()); })} class="form-input" required={matches!(*action, UserAction::Create)} />
                                                </div>
                                                <div class="form-group flex-row">
                                                    <input type="checkbox" id="is_admin" checked={*form_is_admin} onchange={Callback::from(move |e: yew::Event| { let input = e.target_unchecked_into::<web_sys::HtmlInputElement>(); form_is_admin.set(input.checked()); })} />
                                                    <label for="is_admin" class="form-label">{t("administrator_label", lang)}</label>
                                                </div>
                                                <div class="modal-footer">
                                                    <button type="button" onclick={on_cancel_action.clone()} class="btn btn-ghost">{t("cancel", lang)}</button>
                                                    <button type="submit" disabled={*form_loading} class="btn btn-primary">
                                                        {if *form_loading { t("saving", lang) } else { if matches!(*action, UserAction::Edit(_)) { t("update", lang) } else { t("create", lang) } }}
                                                    </button>
                                                </div>
                                            </form>
                                        </div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}

                        {if let UserAction::Delete(_user_id) = *action {
                            html! {
                                <div class="modal-overlay">
                                    <div class="modal modal-sm">
                                        <div class="modal-body text-center">
                                            <h3 class="section-title mb-4">{t("delete_user_title", lang)}</h3>
                                            <p class="text-muted mb-6">{t("delete_user_confirmation", lang)}</p>
                                            <div class="flex justify-center gap-3">
                                                <button onclick={on_cancel_action.clone()} class="btn btn-ghost">{t("cancel", lang)}</button>
                                                <button onclick={on_confirm_delete} class="btn btn-danger">{t("delete", lang)}</button>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                    </>
                }
            }}
        </div>
    }
}
