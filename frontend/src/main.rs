use yew::prelude::*;
mod api;
mod components;
mod i18n;
mod language_provider;
mod pages;
mod theme;

use components::sidebar::Sidebar;
use i18n::{Language, t};
use language_provider::LanguageState;
use language_provider::LanguageProvider;
use pages::admin_setup::AdminSetupPage;
use pages::admin_recipes::AdminRecipesPage;
use pages::admin_categories::AdminCategoriesPage;

#[derive(Clone, PartialEq)]
pub enum Page {
    Home,
    Login,
    Register,
    Add,
    Edit(i32),
    Users,
    Recipes,
    Categories,
    Settings,
    View(i32),
    AdminSetup,
    AdminUsers,
}

fn render_page(page: &Page, navigate: Callback<Page>, search: String, on_search: Callback<String>, _lang: Language, theme_revision: u32, on_theme_changed: Callback<()>) -> Html {
    match page {
        Page::Home => {
            let on_edit = {
                let navigate = navigate.clone();
                Callback::from(move |r: shared_types::Recipe| {
                    if let Some(id) = r.id {
                        navigate.emit(Page::Edit(id));
                    }
                })
            };
            let on_view = {
                let navigate = navigate.clone();
                Callback::from(move |id: i32| {
                    navigate.emit(Page::View(id));
                })
            };
            let on_add = {
                let navigate = navigate.clone();
                Callback::from(move |_| {
                    navigate.emit(Page::Add);
                })
            };

            html! { <crate::components::recipe_list::RecipeList on_edit={on_edit} on_view={on_view} on_add={on_add} refresh={0} search={search} on_search={on_search} /> }
        }
        Page::Login => {
            html! { <crate::pages::login::LoginPage on_navigate={navigate.clone()} /> }
        }
        Page::Register => {
            html! { <crate::pages::register::RegisterPage on_navigate={navigate.clone()} /> }
        }
        Page::Add => {
            html! { <crate::components::recipe_form::RecipeForm on_saved={Callback::from(move |_| navigate.emit(Page::Home))} editing={None} /> }
        }
        Page::Edit(id) => {
            let on_saved = {
                let navigate = navigate.clone();
                Callback::from(move |id: i32| {
                    navigate.emit(Page::View(id));
                })
            };
            html! { <crate::pages::edit::EditRecipe id={*id} on_saved={on_saved} /> }
        }
        Page::Users => html! { <crate::pages::users::UsersPage /> },
        Page::Settings => html! { <crate::pages::settings::SettingsPage theme_revision={theme_revision} on_theme_changed={on_theme_changed} /> },
        Page::View(id) => {
            let on_edit = {
                let navigate = navigate.clone();
                Callback::from(move |id: i32| {
                    navigate.emit(Page::Edit(id));
                })
            };
            let on_back = {
                let navigate = navigate.clone();
                Callback::from(move |_| {
                    navigate.emit(Page::Home);
                })
            };
            html! { <crate::pages::view::ViewRecipe id={*id} on_edit={on_edit} on_back={on_back} /> }
        }
        Page::AdminSetup => {
            html! { <AdminSetupWithGuard on_navigate={navigate.clone()} /> }
        }
        Page::AdminUsers => {
            html! { <crate::pages::admin_users::AdminUsersPage /> }
        }
        Page::Recipes => {
            html! { <AdminRecipesPage /> }
        }
        Page::Categories => {
            html! { <AdminCategoriesPage /> }
        }
    }
}

#[derive(Properties, PartialEq)]
struct AdminSetupGuardProps {
    pub on_navigate: Callback<Page>,
}

#[function_component(AdminSetupWithGuard)]
fn admin_setup_with_guard(props: &AdminSetupGuardProps) -> Html {
    let admin_exists = use_state(|| None::<bool>);
    let loading = use_state(|| true);
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

    {
        let admin_exists = admin_exists.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match api::check_admin_exists().await {
                    Ok(exists) => {
                        admin_exists.set(Some(exists));
                        loading.set(false);
                    }
                    Err(_) => {
                        admin_exists.set(Some(true));
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    if *loading {
        html! {
            <div class="auth-page">
                <div class="spinner"><div class="spinner-circle"></div></div>
            </div>
        }
    } else if let Some(true) = *admin_exists {
        html! {
            <div class="auth-page">
                <div class="auth-card" style="text-align:center;">
                    <div class="auth-logo">
                        <div class="auth-logo-icon">
                            <svg fill="currentColor" viewBox="0 0 24 24">
                                <path d="M21 4.5C19.9 4.15 18.7 4 17.5 4c-1.95 0-4.05.4-5.5 1.5C10.55 4.4 8.45 4 6.5 4 5.3 4 4.1 4.15 3 4.5v14.65c0 .25.25.5.5.5.1 0 .15-.05.25-.05 1.1-.35 2.3-.5 3.5-.5 1.95 0 4.05.4 5.5 1.5 1.45-1.1 3.55-1.5 5.5-1.5 1.2 0 2.4.15 3.5.5.1.05.15.05.25.05.25 0 .5-.25.5-.5V4.5z"/>
                            </svg>
                        </div>
                        <h1>{t("setup_complete", lang)}</h1>
                        <p>{t("setup_complete_desc", lang)}</p>
                    </div>
                    <button
                        onclick={Callback::from({
                            let nav = props.on_navigate.clone();
                            move |_| nav.emit(Page::Home)
                        })}
                        class="btn btn-primary w-full"
                    >
                        {t("go_to_login", lang)}
                    </button>
                </div>
            </div>
        }
    } else {
        html! { <AdminSetupPage on_navigate={props.on_navigate.clone()} /> }
    }
}

#[function_component(App)]
fn app() -> Html {
    let page = use_state(|| Page::Home);
    let search = use_state(|| String::new());
    let admin_check_done = use_state(|| false);
    let theme_revision = use_state(|| 0u32);
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

    {
        let page = page.clone();
        use_effect_with((), move |_| {
            if let Some(window) = web_sys::window() {
                let location = window.location();
                let pathname = location.pathname().unwrap_or_default();
                match pathname.as_str() {
                    "/admin/users" => page.set(Page::AdminUsers),
                    "/admin/recipes" => page.set(Page::Recipes),
                    "/admin/categories" => page.set(Page::Categories),
                    "/admin/setup" => page.set(Page::AdminSetup),
                    "/settings" => page.set(Page::Settings),
                    "/login" => page.set(Page::Login),
                    "/register" => page.set(Page::Register),
                    "/add" => page.set(Page::Add),
                    path if path.starts_with("/edit/") => {
                        let parts: Vec<&str> = path.split('/').collect();
                        if parts.len() > 2 {
                            if let Ok(id) = parts[2].parse::<i32>() {
                                page.set(Page::Edit(id));
                            }
                        }
                    }
                    path if path.starts_with("/view/") => {
                        let parts: Vec<&str> = path.split('/').collect();
                        if parts.len() > 2 {
                            if let Ok(id) = parts[2].parse::<i32>() {
                                page.set(Page::View(id));
                            }
                        }
                    }
                    _ => page.set(Page::Home),
                }
            }
            || ()
        });
    }

    {
        let admin_check_done = admin_check_done.clone();
        let page = page.clone();

        use_effect_with(admin_check_done.clone(), move |_| {
            if !*admin_check_done {
                wasm_bindgen_futures::spawn_local(async move {
                    match api::check_admin_exists().await {
                        Ok(admin_exists) => {
                            if !admin_exists {
                                page.set(Page::AdminSetup);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to check admin existence: {}", e);
                        }
                    }
                    admin_check_done.set(true);
                });
            }
            || ()
        });
    }
    let navigate = {
        let page = page.clone();
        Callback::from(move |p: Page| {
            if let Some(window) = web_sys::window() {
                let history = window.history().unwrap();
                let path = match p {
                    Page::AdminUsers => "/admin/users",
                    Page::Recipes => "/admin/recipes",
                    Page::Categories => "/admin/categories",
                    Page::AdminSetup => "/admin/setup",
                    Page::Settings => "/settings",
                    Page::Login => "/login",
                    Page::Register => "/register",
                    Page::Add => "/add",
                    Page::Edit(id) => &format!("/edit/{}", id),
                    Page::View(id) => &format!("/view/{}", id),
                    _ => "/",
                };
                let state = wasm_bindgen::JsValue::from_str("");
                let url = format!("{}{}", window.location().origin().unwrap_or_default(), path);
                let _ = history.push_state_with_url(&state, &url, None);
            }
            page.set(p);
        })
    };

    let inc_revision = {
        let tr = theme_revision.clone();
        Callback::from(move |_: ()| {
            tr.set(*tr + 1);
        })
    };

    let current = (*page).clone();
    let search_value = (*search).clone();

    let on_search_input = {
        let search = search.clone();
        Callback::from(move |value: String| {
            search.set(value);
        })
    };

    // Auth pages render outside the app layout (full-screen)
    let is_auth_page = matches!(current, Page::Login | Page::Register | Page::AdminSetup);

    if is_auth_page {
        return html! {
            <div class="auth-page">
                { render_page(&current, navigate.clone(), search_value, on_search_input, lang, *theme_revision, inc_revision.clone()) }
            </div>
        };
    }

    let search_navigate = navigate.clone();
    let search_callback_input = on_search_input.clone();

    html! {
        <>
            <Sidebar on_navigate={navigate.clone()}
                theme_revision={*theme_revision}
                on_theme_toggle={Some({
                    let inc = inc_revision.clone();
                    Callback::from(move |theme: String| {
                        let mut prefs = home_hub_shared::prefs::UserPrefs::load();
                        prefs.theme = theme.clone();
                        prefs.save_to_local();
                        inc.emit(());
                        let pj = serde_json::json!({"theme": theme, "primary_color": prefs.primary_color});
                        let pj_str = serde_json::to_string(&pj).unwrap_or_default();
                        wasm_bindgen_futures::spawn_local(async move {
                            let _ = api::save_prefs(&pj_str).await;
                        });
                    })
                })}
            />

            <main class="app-main">
                <div class="app-content">
                    // Search bar (visible only on Home page)
                    {if current == Page::Home {
                        html! {
                            <div class="content-search">
                                <div class="content-search-inner">
                                    <svg class="content-search-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                                    </svg>
                                    <input
                                        type="text"
                                        placeholder={t("search_placeholder", lang)}
                                        value={search_value.clone()}
                                        oninput={Callback::from(move |e: yew::InputEvent| {
                                            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                            let value = input.value();
                                            search_callback_input.emit(value);
                                            search_navigate.emit(Page::Home);
                                        })}
                                        class="content-search-input"
                                    />
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                    <div class="page-enter">
                        { render_page(&current, navigate, search_value, on_search_input, lang, *theme_revision, inc_revision.clone()) }
                    </div>
                </div>
            </main>
        </>
    }
}

fn main() {
    wasm_bindgen_futures::spawn_local(async move {
        gloo::timers::future::sleep(std::time::Duration::from_millis(100)).await;
        crate::theme::init_theme();
    });

    yew::Renderer::<AppWrapper>::new().render();
}

#[function_component(AppWrapper)]
fn app_wrapper() -> Html {
    html! {
        <LanguageProvider>
            <App />
        </LanguageProvider>
    }
}
