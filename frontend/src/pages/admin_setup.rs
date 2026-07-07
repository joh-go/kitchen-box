use yew::prelude::*;
use crate::api;
use crate::i18n::{Language, t};
use crate::language_provider::LanguageState;
use web_sys::window;

#[derive(Clone, PartialEq)]
pub enum SetupStep {
    Welcome,
    CreateAdmin,
    Success,
}

#[function_component(AdminSetupPage)]
pub fn admin_setup_page() -> Html {
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

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
                SetupStep::CreateAdmin => {},
                SetupStep::Success => {},
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

            if name.trim().is_empty() {
                error.set(Some(t("please_enter_name", lang)));
                return;
            }
            if email.trim().is_empty() {
                error.set(Some(t("please_enter_email", lang)));
                return;
            }
            if password.len() < 6 {
                error.set(Some(t("password_min_chars", lang)));
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
                        error.set(Some(format!("{}{}", t("failed_to_create_admin", lang), e)));
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
        <div class="page-center">
            <div class="setup-wizard">
                <div class="text-center mb-8">
                    <div class="setup-wizard-icon">
                        <span class="text-2xl">{"🍳"}</span>
                    </div>
                    <h1 class="section-title">{t("kitchenbox_setup", lang)}</h1>
                    <p class="text-muted">{t("setup_description", lang)}</p>
                </div>

                <div class="setup-steps mb-6">
                    <div class="flex items-center justify-between">
                        <div class={if matches!(step, SetupStep::Welcome) { "step-circle step-circle-active" } else { "step-circle step-circle-done" }}>
                            {"1"}
                        </div>
                        <div class={if matches!(step, SetupStep::CreateAdmin) || matches!(step, SetupStep::Success) { "step-line step-line-active" } else { "step-line" }}></div>
                        <div class={if matches!(step, SetupStep::CreateAdmin) { "step-circle step-circle-active" } else if matches!(step, SetupStep::Success) { "step-circle step-circle-done" } else { "step-circle" }}>
                            {"2"}
                        </div>
                        <div class={if matches!(step, SetupStep::Success) { "step-line step-line-active" } else { "step-line" }}></div>
                        <div class={if matches!(step, SetupStep::Success) { "step-circle step-circle-done" } else { "step-circle" }}>
                            {"✓"}
                        </div>
                    </div>
                </div>

                <div class="card">
                    <div class="card-body">
                        {match step {
                            SetupStep::Welcome => html! {
                                <div class="text-center">
                                    <h2 class="section-title mb-4">{t("welcome_kitchenbox", lang)}</h2>
                                    <p class="text-muted mb-6">{t("setup_welcome_message", lang)}</p>
                                    <div class="setup-features mb-6">
                                        <div class="flex items-center gap-3 text-sm text-muted mb-3">
                                            <span class="text-success">{"✓"}</span>
                                            <span>{t("manage_users_recipes", lang)}</span>
                                        </div>
                                        <div class="flex items-center gap-3 text-sm text-muted mb-3">
                                            <span class="text-success">{"✓"}</span>
                                            <span>{t("configure_system", lang)}</span>
                                        </div>
                                        <div class="flex items-center gap-3 text-sm text-muted">
                                            <span class="text-success">{"✓"}</span>
                                            <span>{t("full_access_features", lang)}</span>
                                        </div>
                                    </div>
                                    <button onclick={on_next} class="btn-primary">{t("get_started", lang)}</button>
                                </div>
                            },
                            SetupStep::CreateAdmin => html! {
                                <div>
                                    <h2 class="section-title mb-6 text-center">{t("create_administrator_account", lang)}</h2>

                                    {if let Some(ref error_msg) = *error {
                                        html! { <div class="alert alert-error"><div class="alert-content">{error_msg}</div></div> }
                                    } else {
                                        html! {}
                                    }}

                                    <form>
                                        <div class="form-group">
                                            <label class="form-label">{t("name", lang)}</label>
                                            <input
                                                type="text"
                                                value={(*name).clone()}
                                                oninput={on_name_change}
                                                placeholder={t("enter_your_name", lang)}
                                                class="form-input"
                                                disabled={*loading}
                                            />
                                        </div>
                                        <div class="form-group">
                                            <label class="form-label">{t("email_address", lang)}</label>
                                            <input
                                                type="email"
                                                value={(*email).clone()}
                                                oninput={on_email_change}
                                                placeholder={t("admin_example_email", lang)}
                                                class="form-input"
                                                disabled={*loading}
                                            />
                                        </div>
                                        <div class="form-group">
                                            <label class="form-label">{t("password", lang)}</label>
                                            <input
                                                type="password"
                                                value={(*password).clone()}
                                                oninput={on_password_change}
                                                placeholder={t("create_strong_password", lang)}
                                                class="form-input"
                                                disabled={*loading}
                                            />
                                        </div>
                                        <div class="form-group">
                                            <label class="form-label">{t("confirm_password", lang)}</label>
                                            <input
                                                type="password"
                                                value={(*confirm_password).clone()}
                                                oninput={on_confirm_password_change}
                                                placeholder={t("confirm_your_password", lang)}
                                                class="form-input"
                                                disabled={*loading}
                                            />
                                        </div>

                                        <button type="button" onclick={on_create_admin} disabled={*loading} class="btn-primary w-full">
                                            {if *loading {
                                                html! { <><span class="spinner-spin"></span> {t("creating_admin", lang)}</> }
                                            } else {
                                                html! { {t("create_administrator", lang)} }
                                            }}
                                        </button>
                                    </form>
                                </div>
                            },
                            SetupStep::Success => html! {
                                <div class="text-center">
                                    <h2 class="section-title mb-4">{t("setup_complete", lang)}</h2>
                                    <p class="text-muted mb-6">{t("admin_created_success", lang)}</p>
                                    <button onclick={Callback::from(|_| {
                                        if let Some(window) = window() {
                                            let _ = window.location().reload();
                                        }
                                    })} class="btn-primary">
                                        {t("go_to_login", lang)}
                                    </button>
                                </div>
                            }
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
