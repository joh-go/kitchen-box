use yew::prelude::*;
mod api;
mod components;
mod pages;

use components::sidebar::Sidebar;

#[derive(Clone, PartialEq)]
pub enum Page {
    Home,
    Add,
    Edit(i32),
    Users,
}

fn render_page(page: &Page, navigate: Callback<Page>) -> Html {
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

            html! { <crate::components::recipe_list::RecipeList on_edit={on_edit} refresh={0} search={String::new()} /> }
        }
        Page::Add => {
            html! { <crate::components::recipe_form::RecipeForm on_saved={Callback::from(move |_| navigate.emit(Page::Home))} editing={None} /> }
        }
        Page::Edit(id) => html! { <crate::pages::edit::EditRecipe id={*id} /> },
        Page::Users => html! { <crate::pages::users::UsersPage /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let page = use_state(|| Page::Home);
    let navigate = {
        let page = page.clone();
        Callback::from(move |p: Page| {
            page.set(p);
        })
    };

    let current = (*page).clone();

    html! {
        <div class="min-h-screen bg-gray-50 dark:bg-gray-900">
            <header class="bg-white dark:bg-gray-800 shadow">
                <div class="container mx-auto px-4 py-4 flex items-center justify-between">
                    <h1 class="text-2xl font-bold text-gray-900 dark:text-white">{ "My Recipes" }</h1>
                    <nav class="text-sm text-gray-600 dark:text-gray-300">{ "Personal cookbook" }</nav>
                </div>
            </header>

            <div class="container mx-auto p-4 grid grid-cols-1 lg:grid-cols-4 gap-6">
                <div class="lg:col-span-1">
                    <Sidebar on_navigate={navigate.clone()} />
                </div>
                <div class="lg:col-span-3">
                    { render_page(&current, navigate.clone()) }
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
