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
                <div class="relative">
                    <select 
                        value={current_lang.as_str()}
                        onchange={on_language_change}
                        class="appearance-none bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 px-3 py-2 pr-8 text-sm focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent cursor-pointer hover:border-emerald-300 dark:hover:border-emerald-600 transition-colors"
                    >
                        <option value="en">{Language::English.display_name()}</option>
                        <option value="de">{Language::German.display_name()}</option>
                    </select>
                    <div class="absolute inset-y-0 right-0 flex items-center pr-2 pointer-events-none">
                        <svg class="w-4 h-4 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                        </svg>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {
            <div class={class}></div>
        }
    }
}
