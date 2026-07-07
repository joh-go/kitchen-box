#[derive(Clone, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

pub fn get_theme() -> Theme {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(theme)) = storage.get_item("theme") {
                match theme.as_str() {
                    "dark" => Theme::Dark,
                    _ => Theme::Light,
                }
            } else {
                Theme::Light
            }
        } else {
            Theme::Light
        }
    } else {
        Theme::Light
    }
}

pub fn set_theme(theme: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("theme", theme);
        }

        if let Some(document) = window.document() {
            if let Some(html_elem) = document.document_element() {
                let _ = html_elem.set_attribute("data-theme", theme);
            }
        }
    }
}

pub fn init_theme() {
    let theme = get_theme();
    set_theme(theme.as_str());
}
