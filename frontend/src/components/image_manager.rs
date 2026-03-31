use crate::api;
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
    let images = use_state(|| props.images.clone());
    let uploading = use_state(|| false);
    let error = use_state(|| None::<String>);

    // Sync with props when they change (e.g., after saving/reloading recipe)
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
                                        error.set(Some(format!("Failed to upload image: {}", err)));
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
        <div class="space-y-4">
            <div>
                <h3 class="text-lg font-semibold text-slate-800 dark:text-slate-200 mb-3">
                    { "Images" }
                </h3>
                
                // Upload section
                <div class="border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-lg p-6 text-center">
                    <input
                        type="file"
                        accept="image/*"
                        onchange={on_file_select}
                        disabled={props.recipe_id.is_none() || *uploading}
                        class="hidden"
                        id="image-upload"
                    />
                    <label 
                        for="image-upload"
                        class="cursor-pointer inline-flex items-center px-4 py-2 bg-emerald-600 hover:bg-emerald-700 text-white rounded-md font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        { if *uploading { 
                            "Uploading..." 
                        } else { 
                            "Choose Image" 
                        } }
                    </label>
                    <p class="text-sm text-slate-500 dark:text-slate-400 mt-2">
                        { "Upload images for your recipe (JPG, PNG, etc.)" }
                    </p>
                </div>

                // Error display
                { if let Some(err) = (*error).clone() {
                    html! {
                        <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-300 px-4 py-3 rounded-md">
                            { err }
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>

            // Images grid
            { if !current_images.is_empty() {
                html! {
                    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                        { for current_images.iter().map(|image| {
                            let image_id = image.id.unwrap_or(0);
                            let is_primary = image.is_primary.unwrap_or(false);
                            let filename = &image.filename;
                            let recipe_id = props.recipe_id;
                            let on_images_changed_primary = props.on_images_changed.clone();
                            let on_images_changed_delete = props.on_images_changed.clone();
                            
                            // Create image URL - use the correct backend port
                            let image_url = format!("http://127.0.0.1:8000/uploads/recipes/{}/{}", 
                                recipe_id.unwrap_or(0), filename);
                            
                            html! {
                                <div class="relative group">
                                    <div class="aspect-square bg-slate-100 dark:bg-slate-800 rounded-lg overflow-hidden">
                                        // Try to display the actual image
                                        <img 
                                            src={image_url.clone()}
                                            alt={filename.clone()}
                                            class="w-full h-full object-cover"
                                        />
                                        
                                        // Fallback placeholder if image doesn't load
                                        <div class="w-full h-full flex items-center justify-center hidden">
                                            <div class="text-center p-4">
                                                <div class="text-2xl mb-2">{ "CAMERA" }</div>
                                                <div class="text-xs text-slate-500 dark:text-slate-400 truncate">
                                                    { filename }
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    // Primary badge
                                    { if is_primary {
                                        html! {
                                            <div class="absolute top-2 left-2 bg-emerald-600 text-white text-xs px-2 py-1 rounded-full">
                                                { "Primary" }
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }}

                                    // Action buttons
                                    <div class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity">
                                        <div class="flex gap-1">
                                            { if !is_primary {
                                                html! {
                                                    <button
                                                        onclick={Callback::from(move |_| {
                                                            let recipe_id = recipe_id;
                                                            let on_images_changed = on_images_changed_primary.clone();
                                                            spawn_local(async move {
                                                                if let Some(rid) = recipe_id {
                                                                    if let Ok(()) = api::set_primary_image(rid, image_id).await {
                                                                        // Refresh images by calling API again
                                                                        if let Ok(refreshed_images) = api::get_recipe_images(rid).await {
                                                                            on_images_changed.emit(refreshed_images);
                                                                        }
                                                                    }
                                                                }
                                                            });
                                                        })}
                                                        class="bg-blue-600 hover:bg-blue-700 text-white p-1 rounded text-xs"
                                                        title="Set as primary"
                                                    >
                                                        { "STAR" }
                                                    </button>
                                                }
                                            } else {
                                                html! {}
                                            }}
                                            <button
                                                onclick={Callback::from(move |_| {
                                                    let recipe_id = recipe_id;
                                                    let on_images_changed = on_images_changed_delete.clone();
                                                    spawn_local(async move {
                                                        if let Some(rid) = recipe_id {
                                                            if let Ok(()) = api::delete_recipe_image(rid, image_id).await {
                                                                // Refresh images by calling API again
                                                                if let Ok(refreshed_images) = api::get_recipe_images(rid).await {
                                                                    on_images_changed.emit(refreshed_images);
                                                                }
                                                            }
                                                        }
                                                    });
                                                })}
                                                class="bg-red-600 hover:bg-red-700 text-white p-1 rounded text-xs"
                                                title="Delete image"
                                            >
                                                { "DELETE" }
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            }
                        }) }
                    </div>
                }
            } else {
                html! {
                    <div class="text-center py-8 text-slate-500 dark:text-slate-400">
                        { "No images uploaded yet" }
                    </div>
                }
            }}
        </div>
    }
}
