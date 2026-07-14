use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
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
    let on_nav = props.on_navigate.clone();
    let on_mobile_close = props.on_mobile_close.clone();
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

    let to_home = {
        let on_nav = on_nav.clone();
        let on_mobile_close = on_mobile_close.clone();
        Callback::from(move |e: yew::MouseEvent| {
            on_nav.emit(crate::Page::Home);
            on_mobile_close.emit(e);
        })
    };
    let to_add = {
        let on_nav = on_nav.clone();
        let on_mobile_close = on_mobile_close.clone();
        Callback::from(move |e: yew::MouseEvent| {
            on_nav.emit(crate::Page::Add);
            on_mobile_close.emit(e);
        })
    };
    let to_login = {
        let on_nav = on_nav.clone();
        let on_mobile_close = on_mobile_close.clone();
        Callback::from(move |e: yew::MouseEvent| {
            on_nav.emit(crate::Page::Login);
            on_mobile_close.emit(e);
        })
    };
    let to_register = {
        let on_nav = on_nav.clone();
        let on_mobile_close = on_mobile_close.clone();
        Callback::from(move |e: yew::MouseEvent| {
            on_nav.emit(crate::Page::Register);
            on_mobile_close.emit(e);
        })
    };

    let to_settings = {
        let on_nav = on_nav.clone();
        let on_mobile_close = on_mobile_close.clone();
        Callback::from(move |e: yew::MouseEvent| {
            on_nav.emit(crate::Page::Settings);
            on_mobile_close.emit(e);
        })
    };

    let on_logout = {
        let on_mobile_close = on_mobile_close.clone();
        Callback::from(move |e: yew::MouseEvent| {
            api::logout();
            on_mobile_close.emit(e);
        })
    };

    html! {
        <>
            <div class="sidebar-header">
                <div class="sidebar-logo">
                    <div class="sidebar-logo-icon">
                        <svg fill="currentColor" viewBox="0 0 24 24">
                            <path d="M21 4.5C19.9 4.15 18.7 4 17.5 4c-1.95 0-4.05.4-5.5 1.5C10.55 4.4 8.45 4 6.5 4 5.3 4 4.1 4.15 3 4.5v14.65c0 .25.25.5.5.5.1 0 .15-.05.25-.05 1.1-.35 2.3-.5 3.5-.5 1.95 0 4.05.4 5.5 1.5 1.45-1.1 3.55-1.5 5.5-1.5 1.2 0 2.4.15 3.5.5.1.05.15.05.25.05.25 0 .5-.25.5-.5V4.5z"/>
                        </svg>
                    </div>
                    <span class="sidebar-logo-text">{t("app_name", lang)}</span>
                </div>
            </div>

            {if is_logged_in {
                html! {
                    <div class="sidebar-user">
                        <div class="sidebar-user-name">
                            {if let Some(ref name) = user_name { name.clone() } else { t("user", lang).to_string() }}
                        </div>
                        <div class="sidebar-user-role">{t("my_recipes", lang)}</div>
                    </div>
                }
            } else {
                html! {}
            }}

            <nav class="sidebar-nav">
                <button onclick={to_home} class="sidebar-nav-item">
                    <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path>
                    </svg>
                    {t("all_recipes", lang)}
                </button>

                {if is_logged_in {
                    html! {
                        <>
                            <button onclick={to_add} class="sidebar-nav-item">
                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                                </svg>
                                {t("nav_add", lang)}
                            </button>

                            <button onclick={to_settings} class="sidebar-nav-item">
                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                </svg>
                                {t("nav_settings", lang)}
                            </button>

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
                            <button onclick={to_login} class="sidebar-nav-item">
                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 16l-4-4a4 4 0 0112-4.354a6 6 0 112.354 0 018-1.18l4-4 4a4 4 0 0112-4.354 0-6.47a6 6 0 00-9.542 4.438 0 018-1.18z"></path>
                                </svg>
                                {t("login", lang)}
                            </button>
                            <button onclick={to_register} class="sidebar-nav-item">
                                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 16l-4-4a4 4 0 0112-4.354a6 6 0 112.354 0 018-1.18l4-4 4a4 4 0 0112-4.354 0-6.47a6 6 0 00-9.542 4.438 0 018-1.18z"></path>
                                </svg>
                                {t("register", lang)}
                            </button>
                        </>
                    }
                }}
            </nav>

            <div class="sidebar-footer">
                <div class="flex items-center gap-3 mb-3" style="padding: 0 var(--space-1);">
                    <LanguageSwitcher />
                    <ThemeToggle />
                </div>
                {if is_logged_in {
                    html! {
                        <button onclick={on_logout} class="sidebar-footer-item">
                            <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4 4m4-4H3"></path>
                            </svg>
                            {t("logout", lang)}
                        </button>
                    }
                } else {
                    html! {}
                }}
            </div>
        </>
    }
}
