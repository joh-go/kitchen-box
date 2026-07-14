diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Text,
        slug -> Text,
        description -> Nullable<Text>,
        parent_id -> Nullable<Int4>,
        position -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    images (id) {
        id -> Int4,
        recipe_id -> Nullable<Int4>,
        filename -> Text,
        original_filename -> Nullable<Text>,
        file_path -> Text,
        file_size -> Nullable<Int4>,
        mime_type -> Nullable<Text>,
        alt -> Nullable<Text>,
        is_primary -> Nullable<Bool>,
        position -> Nullable<Int4>,
        uploaded_at -> Timestamptz,
    }
}

diesel::table! {
    recipe_categories (recipe_id, category_id) {
        recipe_id -> Int4,
        category_id -> Int4,
    }
}

diesel::table! {
    recipe_versions (id) {
        id -> Int4,
        recipe_id -> Int4,
        payload -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        author_id -> Nullable<Int4>,
    }
}

diesel::table! {
    recipes (id) {
        id -> Int4,
        title -> Text,
        slug -> Text,
        short_description -> Nullable<Text>,
        ingredients -> Jsonb,
        steps -> Jsonb,
        prep_minutes -> Nullable<Int4>,
        cook_minutes -> Nullable<Int4>,
        servings -> Nullable<Int4>,
        notes -> Nullable<Text>,
        author_id -> Nullable<Int4>,
        is_public -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_prefs (user_id) {
        user_id -> Int4,
        prefs -> Text,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password -> Text,
        is_admin -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(recipe_categories -> recipes (recipe_id));
diesel::joinable!(recipe_categories -> categories (category_id));
diesel::joinable!(images -> recipes (recipe_id));
diesel::joinable!(recipe_versions -> recipes (recipe_id));
diesel::joinable!(recipes -> users (author_id));
diesel::joinable!(user_prefs -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    categories,
    recipes,
    recipe_categories,
    images,
    recipe_versions,
    user_prefs,
);
