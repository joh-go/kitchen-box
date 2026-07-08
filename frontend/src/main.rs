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

fn render_page(page: &Page, navigate: Callback<Page>, search: String, on_search: Callback<String>, _lang: Language) -> Html {
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
            html! { <crate::pages::login::LoginPage /> }
        }
        Page::Register => {
            html! { <crate::pages::register::RegisterPage /> }
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
        Page::Settings => html! { <crate::pages::settings::SettingsPage /> },
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
            html! { <AdminSetupWithGuard /> }
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

#[function_component(AdminSetupWithGuard)]
fn admin_setup_with_guard() -> Html {
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
                        onclick={Callback::from(|_| {
                            if let Some(window) = web_sys::window() {
                                let _ = window.location().set_href("/");
                            }
                        })}
                        class="btn btn-primary w-full"
                    >
                        {t("go_to_login", lang)}
                    </button>
                </div>
            </div>
        }
    } else {
        html! { <AdminSetupPage /> }
    }
}

#[function_component(App)]
fn app() -> Html {
    let page = use_state(|| Page::Home);
    let sidebar_open = use_state(|| false);
    let search = use_state(|| String::new());
    let admin_check_done = use_state(|| false);
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

    let toggle_sidebar = {
        let sidebar_open = sidebar_open.clone();
        Callback::from(move |_| {
            sidebar_open.set(!*sidebar_open);
        })
    };

    let close_sidebar = {
        let sidebar_open = sidebar_open.clone();
        Callback::from(move |_: yew::MouseEvent| {
            sidebar_open.set(false);
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
                { render_page(&current, navigate.clone(), search_value, on_search_input, lang) }
            </div>
        };
    }

    let search_navigate = navigate.clone();
    let search_callback_input = on_search_input.clone();
    let mobile_nav_home = navigate.reform(|_: yew::MouseEvent| Page::Home);
    let mobile_nav_add = navigate.reform(|_: yew::MouseEvent| Page::Add);
    let mobile_nav_settings = navigate.reform(|_: yew::MouseEvent| Page::Settings);
    let mobile_sidebar_toggle = toggle_sidebar.clone();

    let sidebar_class = if *sidebar_open { "sidebar open" } else { "sidebar" };
    let backdrop_class = if *sidebar_open { "sidebar-backdrop open" } else { "sidebar-backdrop" };

    html! {
        <>
            // Sidebar backdrop (mobile only)
            <div class={backdrop_class} onclick={close_sidebar.clone()}></div>

            // Sidebar (fixed, dark gradient — slides on mobile)
            <aside class={sidebar_class}>
                <Sidebar on_navigate={navigate.clone()} on_mobile_close={close_sidebar.clone()} />
            </aside>

            // Main content area
            <main class="app-main">
                <div class="app-content">
                    // Search bar (visible on all pages)
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
                    <div class="page-enter">
                        { render_page(&current, navigate, search_value, on_search_input, lang) }
                    </div>
                </div>
            </main>

            // Mobile sidebar toggle button
            <button onclick={mobile_sidebar_toggle} class="sidebar-toggle">
                <svg width="20" height="20" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                </svg>
            </button>

            // Mobile Bottom Navigation
            <nav class="mobile-bottom-nav">
                <div class="mobile-bottom-nav-inner">
                    <button onclick={mobile_nav_home} class="mobile-nav-btn">
                        <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path>
                        </svg>
                        <span>{t("nav_home", lang)}</span>
                    </button>

                    {if api::is_logged_in() {
                        html! {
                            <button onclick={mobile_nav_add} class="mobile-nav-btn">
                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                                </svg>
                                <span>{t("nav_add", lang)}</span>
                            </button>
                        }
                    } else {
                        html! { <div></div> }
                    }}

                    {if api::is_logged_in() {
                        html! {
                            <button onclick={mobile_nav_settings} class="mobile-nav-btn mobile-nav-btn-inactive">
                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                </svg>
                                <span>{t("nav_settings", lang)}</span>
                            </button>
                        }
                    } else {
                        html! { <div></div> }
                    }}

                    <button onclick={toggle_sidebar} class="mobile-nav-btn mobile-nav-btn-inactive">
                        <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                        </svg>
                        <span>{t("menu", lang)}</span>
                    </button>
                </div>
            </nav>

            <div class="mobile-nav-spacer"></div>
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
