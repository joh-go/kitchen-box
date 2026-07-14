use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use home_hub_shared::icons::{Icon, IconComponent};
use crate::api;
use crate::i18n::t;
use crate::language_provider::LanguageState;
use crate::i18n::Language;
use crate::components::language_switcher::LanguageSwitcher;
use crate::components::theme_provider::ThemeToggle;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_navigate: Callback<crate::Page>,
    #[prop_or(Callback::from(|_: yew::MouseEvent| ()))]
    pub on_mobile_close: Callback<yew::MouseEvent>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &Props) -> Html {
    let is_open = use_state(|| false);
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

    let is_logged_in = api::is_logged_in();
    let user_name = api::get_current_user_name();

    let recipe_count = use_state(|| 0i32);
    {
        let recipe_count = recipe_count.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(recipes) = api::get_recipes().await {
                    recipe_count.set(recipes.len() as i32);
                }
            });
            || ()
        });
    }

    let close_sidebar = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(false))
    };

    let toggle_sidebar = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    let navigate = {
        let is_open = is_open.clone();
        let on_nav = props.on_navigate.clone();
        Callback::from(move |page| {
            is_open.set(false);
            on_nav.emit(page);
        })
    };

    let nav_item = |icon: Icon, label: &str, page: crate::Page| -> Html {
        let cb = { let n = navigate.clone(); Callback::from(move |_: yew::MouseEvent| n.emit(page.clone())) };
        html! {
            <button class="sidebar-nav-item" onclick={cb}>
                <IconComponent kind={icon} size={20} color="currentColor" />
                <span>{label}</span>
            </button>
        }
    };

    let avatar_initial = user_name.as_deref().and_then(|n| n.chars().next()).map(|c| c.to_ascii_uppercase()).unwrap_or(' ');

    html! {
        <>
            <button class="sidebar-toggle" onclick={toggle_sidebar} aria-label="Menü öffnen">
                <IconComponent kind={Icon::Menu} size={20} color="currentColor" />
            </button>

            <div class={classes!("sidebar-backdrop", if *is_open { "open" } else { "" })}
                onclick={close_sidebar.clone()} />

            <aside class={classes!("sidebar", if *is_open { "open" } else { "" })}>
                <div class="sidebar-header">
                    <div class="sidebar-logo">
                        <div class="sidebar-logo-icon">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M21 4.5C19.9 4.15 18.7 4 17.5 4c-1.95 0-4.05.4-5.5 1.5C10.55 4.4 8.45 4 6.5 4 5.3 4 4.1 4.15 3 4.5v14.65c0 .25.25.5.5.5.1 0 .15-.05.25-.05 1.1-.35 2.3-.5 3.5-.5 1.95 0 4.05.4 5.5 1.5 1.45-1.1 3.55-1.5 5.5-1.5 1.2 0 2.4.15 3.5.5.1.05.15.05.25.05.25 0 .5-.25.5-.5V4.5z"/>
                            </svg>
                        </div>
                        <span class="sidebar-logo-text">{t("app_name", lang)}</span>
                    </div>
                </div>

                {if is_logged_in {
                    html! {
                        <div class="sidebar-user">
                            <div class="sidebar-avatar">{avatar_initial}</div>
                            <div class="sidebar-user-info">
                                <div class="sidebar-user-name">{user_name.clone().unwrap_or_default()}</div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }}

                <nav class="sidebar-nav">
                    {nav_item(Icon::Home, &t("all_recipes", lang), crate::Page::Home)}

                    {if is_logged_in {
                        html! {
                            <>
                                {nav_item(Icon::Plus, &t("nav_add", lang), crate::Page::Add)}
                                {nav_item(Icon::Settings, &t("nav_settings", lang), crate::Page::Settings)}
                                <div class="sidebar-divider"></div>
                                <div class="sidebar-nav-item" style="pointer-events:none; opacity:0.7;">
                                    <span style="font-size:var(--font-size-xs); text-transform:uppercase; letter-spacing:0.5px;">
                                        {t("recipes_count", lang)}
                                    </span>
                                    <span style="margin-left:auto; font-weight:700; font-size:var(--font-size-lg);">
                                        {*recipe_count}
                                    </span>
                                </div>
                            </>
                        }
                    } else {
                        html! {
                            <>
                                <div class="sidebar-divider"></div>
                                {nav_item(Icon::User, &t("login", lang), crate::Page::Login)}
                                {nav_item(Icon::Plus, &t("register", lang), crate::Page::Register)}
                            </>
                        }
                    }}
                </nav>

                <div class="sidebar-footer">
                    <div class="sidebar-footer-controls">
                        <LanguageSwitcher />
                        <ThemeToggle />
                    </div>
                    {if is_logged_in {
                        html! {
                            <button class="sidebar-footer-item" onclick={Callback::from(move |_: yew::MouseEvent| { api::logout(); close_sidebar.emit(MouseEvent::new("click").unwrap()); })}>
                                <IconComponent kind={Icon::Logout} size={18} color="currentColor" />
                                <span>{t("logout", lang)}</span>
                            </button>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </aside>
        </>
    }
}
