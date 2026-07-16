use crate::api;
use crate::i18n::{Language, t};
use crate::language_provider::LanguageState;
use shared_types::RecipeImage;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub recipe_id: Option<i32>,
    pub images: Vec<RecipeImage>,
    pub on_images_changed: Callback<Vec<RecipeImage>>,
}

#[function_component(ImageManager)]
pub fn image_manager(props: &Props) -> Html {
    let lang_ctx = use_context::<LanguageState>();
    let lang = lang_ctx.as_ref().map(|c| c.language).unwrap_or(Language::English);

    let images = use_state(|| props.images.clone());
    let uploading = use_state(|| false);
    let error = use_state(|| None::<String>);

    {
        let images = images.clone();
        let props_images = props.images.clone();
        use_effect_with(props.images.clone(), move |_| {
            images.set(props_images);
            || ()
        });
    }

    let on_file_select = {
        let recipe_id = props.recipe_id;
        let images = images.clone();
        let uploading = uploading.clone();
        let error = error.clone();
        let on_images_changed = props.on_images_changed.clone();

        Callback::from(move |e: web_sys::Event| {
            let recipe_id = recipe_id;
            let images = images.clone();
            let uploading = uploading.clone();
            let error = error.clone();
            let on_images_changed = on_images_changed.clone();

            if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                if let Some(files) = input.files() {
                    if files.length() > 0 {
                        let file = files.get(0).unwrap();
                        spawn_local(async move {
                            uploading.set(true);
                            error.set(None);

                            if let Some(rid) = recipe_id {
                                match api::upload_recipe_image(rid, &file).await {
                                    Ok(new_image) => {
                                        let mut current_images = (*images).clone();
                                        current_images.push(new_image);
                                        images.set(current_images.clone());
                                        on_images_changed.emit(current_images);
                                    }
                                    Err(err) => {
                                        error.set(Some(format!("{}: {}", t("failed_upload_image", lang), err)));
                                    }
                                }
                            }
                            uploading.set(false);
                        });
                    }
                }
            }
        })
    };

    let current_images = (*images).clone();

    html! {
        <div class="image-manager">
            <h3 class="section-title mb-3">{ t("images", lang) }</h3>

            <div class="image-upload-area">
                <div class="image-upload-btn">
                    <input
                        type="file"
                        accept="image/*"
                        onchange={on_file_select}
                        disabled={props.recipe_id.is_none() || *uploading}
                        id="image-upload"
                    />
                    <label for="image-upload" class="btn btn-primary">
                        { if *uploading { t("uploading", lang) } else { t("choose_image", lang) } }
                    </label>
                </div>
                { if *uploading {
                    html! { <span class="image-upload-progress">{t("uploading", lang)}</span> }
                } else { html!{} }}
                { if let Some(err) = (*error).clone() {
                    html! { <span class="image-upload-error">{ err }</span> }
                } else { html!{} }}
            </div>

            { if !current_images.is_empty() {
                html! {
                    <div class="image-grid">
                        { for current_images.iter().map(|image| {
                            let image_id = image.id.unwrap_or(0);
                            let is_primary = image.is_primary.unwrap_or(false);
                            let filename = &image.filename;
                            let recipe_id = props.recipe_id;
                            let on_images_changed_primary = props.on_images_changed.clone();
                            let on_images_changed_delete = props.on_images_changed.clone();

                            let image_url = format!("/uploads/recipes/{}/{}",
                                recipe_id.unwrap_or(0), filename);

                            html! {
                                <div class="image-grid-item">
                                    <img
                                        src={image_url.clone()}
                                        alt={filename.clone()}
                                    />
                                    { if is_primary {
                                        html! {
                                            <div class="image-primary-badge">
                                                <svg fill="currentColor" viewBox="0 0 20 20">
                                                    <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                                                </svg>
                                            </div>
                                        }
                                    } else { html!{} }}

                                    <div class="image-grid-item-actions">
                                        { if !is_primary {
                                            html! {
                                                <button
                                                    type="button"
                                                    onclick={Callback::from(move |_| {
                                                        let recipe_id = recipe_id;
                                                        let on_images_changed = on_images_changed_primary.clone();
                                                        spawn_local(async move {
                                                            if let Some(rid) = recipe_id {
                                                                if let Ok(()) = api::set_primary_image(rid, image_id).await {
                                                                    if let Ok(refreshed_images) = api::get_recipe_images(rid).await {
                                                                        on_images_changed.emit(refreshed_images);
                                                                    }
                                                                }
                                                            }
                                                        });
                                                    })}
                                                    class="image-action-btn image-action-btn-primary"
                                                    title={t("set_as_primary", lang)}
                                                >
                                                    <svg fill="currentColor" viewBox="0 0 20 20">
                                                        <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                                                    </svg>
                                                </button>
                                            }
                                        } else { html!{} }}
                                        <button
                                            type="button"
                                            onclick={Callback::from(move |_| {
                                                let recipe_id = recipe_id;
                                                let on_images_changed = on_images_changed_delete.clone();
                                                spawn_local(async move {
                                                    if let Some(rid) = recipe_id {
                                                        if let Ok(()) = api::delete_recipe_image(rid, image_id).await {
                                                            if let Ok(refreshed_images) = api::get_recipe_images(rid).await {
                                                                on_images_changed.emit(refreshed_images);
                                                            }
                                                        }
                                                    }
                                                });
                                            })}
                                            class="image-action-btn image-action-btn-danger"
                                            title={t("delete_image", lang)}
                                        >
                                            <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                                            </svg>
                                        </button>
                                    </div>
                                </div>
                            }
                        }) }
                    </div>
                }
            } else {
                html! {
                    <div class="empty-state">
                        <p class="text-muted">{ t("no_images_uploaded", lang) }</p>
                    </div>
                }
            }}
        </div>
    }
}
