use yew::prelude::*;
use crate::i18n::Language;
use web_sys::window;

// Language context
#[derive(Clone, Debug, PartialEq)]
pub struct LanguageContext {
    pub language: Language,
}

#[derive(Clone, Debug)]
pub enum LanguageAction {
    SetLanguage(Language),
}

impl Reducible for LanguageContext {
    type Action = LanguageAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            LanguageAction::SetLanguage(lang) => {
                // Save to localStorage
                if let Some(window) = window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        let _ = storage.set_item("language", lang.as_str());
                    }
                }
                std::rc::Rc::new(LanguageContext { language: lang })
            }
        }
    }
}

pub type LanguageState = UseReducerHandle<LanguageContext>;

// Hook to get initial language from localStorage
pub fn get_initial_language() -> Language {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(lang_str)) = storage.get_item("language") {
                return match lang_str.as_str() {
                    "de" => Language::German,
                    _ => Language::English,
                };
            }
        }
    }
    Language::English
}

#[derive(Properties, PartialEq)]
pub struct LanguageProviderProps {
    pub children: Children,
}

#[function_component(LanguageProvider)]
pub fn language_provider(props: &LanguageProviderProps) -> Html {
    let initial_lang = get_initial_language();
    let ctx = use_reducer(|| LanguageContext { language: initial_lang });

    html! {
        <ContextProvider<LanguageState> context={ctx}>
            {props.children.clone()}
        </ContextProvider<LanguageState>>
    }
}
