use yew::prelude::*;
use crate::i18n::Language;
use crate::language_provider::{LanguageState, LanguageAction};

#[derive(Properties, PartialEq)]
pub struct LanguageSwitcherProps {
    pub class: Option<String>,
}

#[function_component(LanguageSwitcher)]
pub fn language_switcher(props: &LanguageSwitcherProps) -> Html {
    let lang_ctx = use_context::<LanguageState>();
    let class = props.class.clone().unwrap_or_default();

    if let Some(ref ctx) = lang_ctx {
        let current_lang = ctx.language;
        let lang_ctx = lang_ctx.clone();

        let on_language_change = {
            Callback::from(move |e: yew::Event| {
                let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                let value = select.value();
                let lang = match value.as_str() {
                    "de" => Language::German,
                    _ => Language::English,
                };
                if let Some(ref ctx) = lang_ctx {
                    ctx.dispatch(LanguageAction::SetLanguage(lang));
                }
            })
        };

        html! {
            <div class={class}>
                <div class="language-switcher">
                    <select
                        value={current_lang.as_str()}
                        onchange={on_language_change}
                        class="language-select"
                    >
                        <option value="en">{Language::English.display_name()}</option>
                        <option value="de">{Language::German.display_name()}</option>
                    </select>
                    <svg class="language-switcher-chevron" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                    </svg>
                </div>
            </div>
        }
    } else {
        html! {
            <div class={class}></div>
        }
    }
}
