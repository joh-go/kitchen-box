use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_navigate: Callback<crate::Page>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &Props) -> Html {
    let on_nav = props.on_navigate.clone();
    let to_home = {
        let on_nav = on_nav.clone();
        Callback::from(move |_| on_nav.emit(crate::Page::Home))
    };
    let to_add = {
        let on_nav = on_nav.clone();
        Callback::from(move |_| on_nav.emit(crate::Page::Add))
    };
    let to_users = {
        let on_nav = on_nav.clone();
        Callback::from(move |_| on_nav.emit(crate::Page::Users))
    };

    html! {
        <aside class="w-full lg:w-64 bg-white dark:bg-gray-800 p-4 rounded shadow">
            <nav class="flex flex-col gap-2">
                <button onclick={to_home} class="flex items-center gap-3 text-left px-3 py-2 rounded hover:bg-gray-100 dark:hover:bg-gray-700 transition">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-gray-600 dark:text-gray-300" viewBox="0 0 20 20" fill="currentColor"><path d="M10 2L2 8v8a1 1 0 001 1h5v-5h4v5h5a1 1 0 001-1V8l-8-6z"/></svg>
                    <span class="font-medium">{ "Recipes" }</span>
                </button>
                <button onclick={to_add} class="flex items-center gap-3 text-left px-3 py-2 rounded hover:bg-gray-100 dark:hover:bg-gray-700 transition">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-green-600 dark:text-green-400" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M10 5a1 1 0 011 1v3h3a1 1 0 110 2h-3v3a1 1 0 11-2 0v-3H6a1 1 0 110-2h3V6a1 1 0 011-1z" clip-rule="evenodd"/></svg>
                    <span class="font-medium">{ "Add Recipe" }</span>
                </button>
                <button onclick={to_users} class="flex items-center gap-3 text-left px-3 py-2 rounded hover:bg-gray-100 dark:hover:bg-gray-700 transition">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-blue-600 dark:text-blue-400" viewBox="0 0 20 20" fill="currentColor"><path d="M13 7a3 3 0 11-6 0 3 3 0 016 0z"/><path d="M2 18a6 6 0 0112 0H2z"/></svg>
                    <span class="font-medium">{ "Users" }</span>
                </button>
            </nav>
        </aside>
    }
}
