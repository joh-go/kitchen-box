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

    html!{
        <aside class="w-full lg:w-64 bg-white dark:bg-gray-800 p-4 rounded shadow">
            <nav class="flex flex-col gap-2">
                <button onclick={to_home} class="text-left px-3 py-2 rounded hover:bg-gray-100 dark:hover:bg-gray-700">{ "Recipes" }</button>
                <button onclick={to_add} class="text-left px-3 py-2 rounded hover:bg-gray-100 dark:hover:bg-gray-700">{ "Add Recipe" }</button>
                <button onclick={to_users} class="text-left px-3 py-2 rounded hover:bg-gray-100 dark:hover:bg-gray-700">{ "Users" }</button>
            </nav>
        </aside>
    }
}
