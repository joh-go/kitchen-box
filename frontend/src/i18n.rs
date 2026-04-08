use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    German,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::German => "de",
        }
    }
    
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::German => "Deutsch",
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

lazy_static! {
    static ref TRANSLATIONS: HashMap<&'static str, HashMap<Language, &'static str>> = {
        let mut m = HashMap::new();
        
        // Common
        m.insert("app_name", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Kitchenbox");
            h.insert(Language::German, "Kitchenbox");
            h
        });
        
        m.insert("app_tagline", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Your personal kitchen companion");
            h.insert(Language::German, "Ihr persönlicher Küchenbegleiter");
            h
        });
        
        m.insert("loading", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Loading...");
            h.insert(Language::German, "Wird geladen...");
            h
        });
        
        m.insert("ready_to_cook", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Ready to cook");
            h.insert(Language::German, "Bereit zum Kochen");
            h
        });
        
        m.insert("search_placeholder", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Search recipes...");
            h.insert(Language::German, "Rezepte suchen...");
            h
        });
        
        m.insert("menu", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Menu");
            h.insert(Language::German, "Menü");
            h
        });
        
        // Navigation
        m.insert("nav_home", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Home");
            h.insert(Language::German, "Startseite");
            h
        });
        
        m.insert("nav_add", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Add Recipe");
            h.insert(Language::German, "Rezept hinzufügen");
            h
        });
        
        m.insert("nav_users", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Users");
            h.insert(Language::German, "Benutzer");
            h
        });
        
        m.insert("nav_settings", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Settings");
            h.insert(Language::German, "Einstellungen");
            h
        });
        
        m.insert("nav_admin", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Administration");
            h.insert(Language::German, "Administration");
            h
        });
        
        m.insert("nav_users_admin", {
            let mut h = HashMap::new();
            h.insert(Language::English, "User Management");
            h.insert(Language::German, "Benutzerverwaltung");
            h
        });
        
        m.insert("nav_recipes_admin", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Recipe Management");
            h.insert(Language::German, "Rezeptverwaltung");
            h
        });
        
        m.insert("nav_categories_admin", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Category Management");
            h.insert(Language::German, "Kategorieverwaltung");
            h
        });
        
        // Login/Register
        m.insert("login", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Login");
            h.insert(Language::German, "Anmelden");
            h
        });
        
        m.insert("logout", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Logout");
            h.insert(Language::German, "Abmelden");
            h
        });
        
        m.insert("register", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Register");
            h.insert(Language::German, "Registrieren");
            h
        });
        
        m.insert("email", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Email");
            h.insert(Language::German, "E-Mail");
            h
        });
        
        m.insert("password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Password");
            h.insert(Language::German, "Passwort");
            h
        });
        
        m.insert("name", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Name");
            h.insert(Language::German, "Name");
            h
        });
        
        m.insert("confirm_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Confirm Password");
            h.insert(Language::German, "Passwort bestätigen");
            h
        });
        
        m.insert("login_button", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Sign In");
            h.insert(Language::German, "Anmelden");
            h
        });
        
        m.insert("register_button", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Create Account");
            h.insert(Language::German, "Konto erstellen");
            h
        });
        
        m.insert("have_account", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Already have an account?");
            h.insert(Language::German, "Bereits ein Konto?");
            h
        });
        
        m.insert("no_account", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Don't have an account?");
            h.insert(Language::German, "Noch kein Konto?");
            h
        });
        
        m.insert("click_here", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Click here");
            h.insert(Language::German, "Hier klicken");
            h
        });
        
        // Recipe Form
        m.insert("recipe_title", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Title");
            h.insert(Language::German, "Titel");
            h
        });
        
        m.insert("recipe_description", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Description");
            h.insert(Language::German, "Beschreibung");
            h
        });
        
        m.insert("recipe_short_desc", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Short Description");
            h.insert(Language::German, "Kurzbeschreibung");
            h
        });
        
        m.insert("recipe_ingredients", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Ingredients");
            h.insert(Language::German, "Zutaten");
            h
        });
        
        m.insert("recipe_instructions", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Instructions");
            h.insert(Language::German, "Zubereitung");
            h
        });
        
        m.insert("recipe_categories", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Categories");
            h.insert(Language::German, "Kategorien");
            h
        });
        
        m.insert("recipe_public", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Make this recipe public");
            h.insert(Language::German, "Dieses Rezept öffentlich machen");
            h
        });
        
        m.insert("recipe_save", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Save Recipe");
            h.insert(Language::German, "Rezept speichern");
            h
        });
        
        m.insert("recipe_create", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Create Recipe");
            h.insert(Language::German, "Rezept erstellen");
            h
        });
        
        m.insert("recipe_edit", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Edit Recipe");
            h.insert(Language::German, "Rezept bearbeiten");
            h
        });
        
        m.insert("recipe_add_ingredient", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Add Ingredient");
            h.insert(Language::German, "Zutat hinzufügen");
            h
        });
        
        m.insert("recipe_add_step", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Add Step");
            h.insert(Language::German, "Schritt hinzufügen");
            h
        });
        
        m.insert("recipe_step", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Step");
            h.insert(Language::German, "Schritt");
            h
        });
        
        // Recipe List
        m.insert("recipes_count", {
            let mut h = HashMap::new();
            h.insert(Language::English, "recipes");
            h.insert(Language::German, "Rezepte");
            h
        });
        
        m.insert("no_recipes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "No recipes found");
            h.insert(Language::German, "Keine Rezepte gefunden");
            h
        });
        
        m.insert("no_recipes_yet", {
            let mut h = HashMap::new();
            h.insert(Language::English, "No recipes yet");
            h.insert(Language::German, "Noch keine Rezepte");
            h
        });
        
        m.insert("create_first", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Create your first recipe");
            h.insert(Language::German, "Erstellen Sie Ihr erstes Rezept");
            h
        });
        
        m.insert("my_recipes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "My Recipes");
            h.insert(Language::German, "Meine Rezepte");
            h
        });
        
        m.insert("all_recipes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "All Recipes");
            h.insert(Language::German, "Alle Rezepte");
            h
        });
        
        m.insert("showing", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Showing");
            h.insert(Language::German, "Zeige");
            h
        });
        
        m.insert("results", {
            let mut h = HashMap::new();
            h.insert(Language::English, "results");
            h.insert(Language::German, "Ergebnisse");
            h
        });
        
        m.insert("view_recipe", {
            let mut h = HashMap::new();
            h.insert(Language::English, "View Recipe");
            h.insert(Language::German, "Rezept ansehen");
            h
        });
        
        m.insert("edit", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Edit");
            h.insert(Language::German, "Bearbeiten");
            h
        });
        
        m.insert("delete", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Delete");
            h.insert(Language::German, "Löschen");
            h
        });
        
        m.insert("cancel", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Cancel");
            h.insert(Language::German, "Abbrechen");
            h
        });
        
        m.insert("save", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Save");
            h.insert(Language::German, "Speichern");
            h
        });
        
        m.insert("close", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Close");
            h.insert(Language::German, "Schließen");
            h
        });
        
        // Recipe View
        m.insert("by", {
            let mut h = HashMap::new();
            h.insert(Language::English, "By");
            h.insert(Language::German, "Von");
            h
        });
        
        m.insert("ingredients", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Ingredients");
            h.insert(Language::German, "Zutaten");
            h
        });
        
        m.insert("instructions", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Instructions");
            h.insert(Language::German, "Zubereitung");
            h
        });
        
        m.insert("back", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Back");
            h.insert(Language::German, "Zurück");
            h
        });
        
        m.insert("public", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Public");
            h.insert(Language::German, "Öffentlich");
            h
        });
        
        m.insert("private", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Private");
            h.insert(Language::German, "Privat");
            h
        });
        
        // Image Manager
        m.insert("images", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Images");
            h.insert(Language::German, "Bilder");
            h
        });
        
        m.insert("upload_image", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Upload Image");
            h.insert(Language::German, "Bild hochladen");
            h
        });
        
        m.insert("drop_images", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Drop images here");
            h.insert(Language::German, "Bilder hier ablegen");
            h
        });
        
        m.insert("drag_drop", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Drag and drop images here");
            h.insert(Language::German, "Bilder hierher ziehen und ablegen");
            h
        });
        
        m.insert("or_click", {
            let mut h = HashMap::new();
            h.insert(Language::English, "or click to browse");
            h.insert(Language::German, "oder klicken zum Durchsuchen");
            h
        });
        
        m.insert("set_primary", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Set as Primary");
            h.insert(Language::German, "Als Hauptbild festlegen");
            h
        });
        
        m.insert("primary", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Primary");
            h.insert(Language::German, "Hauptbild");
            h
        });
        
        // Settings
        m.insert("profile_settings", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Profile Settings");
            h.insert(Language::German, "Profileinstellungen");
            h
        });
        
        m.insert("account_info", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Account Information");
            h.insert(Language::German, "Kontoinformationen");
            h
        });
        
        m.insert("change_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Change Password");
            h.insert(Language::German, "Passwort ändern");
            h
        });
        
        m.insert("current_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Current Password");
            h.insert(Language::German, "Aktuelles Passwort");
            h
        });
        
        m.insert("new_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "New Password");
            h.insert(Language::German, "Neues Passwort");
            h
        });
        
        m.insert("confirm_new_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Confirm New Password");
            h.insert(Language::German, "Neues Passwort bestätigen");
            h
        });
        
        m.insert("update_profile", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Update Profile");
            h.insert(Language::German, "Profil aktualisieren");
            h
        });
        
        m.insert("language", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Language");
            h.insert(Language::German, "Sprache");
            h
        });
        
        m.insert("theme", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Theme");
            h.insert(Language::German, "Design");
            h
        });
        
        m.insert("light", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Light");
            h.insert(Language::German, "Hell");
            h
        });
        
        m.insert("dark", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Dark");
            h.insert(Language::German, "Dunkel");
            h
        });
        
        // Admin
        m.insert("user_management", {
            let mut h = HashMap::new();
            h.insert(Language::English, "User Management");
            h.insert(Language::German, "Benutzerverwaltung");
            h
        });
        
        m.insert("recipe_management", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Recipe Management");
            h.insert(Language::German, "Rezeptverwaltung");
            h
        });
        
        m.insert("category_management", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Category Management");
            h.insert(Language::German, "Kategorieverwaltung");
            h
        });
        
        m.insert("manage_users", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Manage all users in the system");
            h.insert(Language::German, "Verwalten Sie alle Benutzer im System");
            h
        });
        
        m.insert("manage_recipes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Manage all recipes in the system");
            h.insert(Language::German, "Verwalten Sie alle Rezepte im System");
            h
        });
        
        m.insert("manage_categories", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Manage all categories in the system");
            h.insert(Language::German, "Verwalten Sie alle Kategorien im System");
            h
        });
        
        m.insert("add_category", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Add New Category");
            h.insert(Language::German, "Neue Kategorie hinzufügen");
            h
        });
        
        m.insert("category_name", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Category Name");
            h.insert(Language::German, "Kategoriename");
            h
        });
        
        m.insert("enter_category", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Enter category name...");
            h.insert(Language::German, "Kategoriename eingeben...");
            h
        });
        
        m.insert("creating", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Creating...");
            h.insert(Language::German, "Wird erstellt...");
            h
        });
        
        m.insert("add", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Add");
            h.insert(Language::German, "Hinzufügen");
            h
        });
        
        m.insert("user", {
            let mut h = HashMap::new();
            h.insert(Language::English, "User");
            h.insert(Language::German, "Benutzer");
            h
        });
        
        m.insert("admin", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Admin");
            h.insert(Language::German, "Administrator");
            h
        });
        
        m.insert("status", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Status");
            h.insert(Language::German, "Status");
            h
        });
        
        m.insert("actions", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Actions");
            h.insert(Language::German, "Aktionen");
            h
        });
        
        m.insert("created", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Created");
            h.insert(Language::German, "Erstellt");
            h
        });
        
        m.insert("recipe", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Recipe");
            h.insert(Language::German, "Rezept");
            h
        });
        
        m.insert("author", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Author");
            h.insert(Language::German, "Autor");
            h
        });
        
        m.insert("unknown", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Unknown");
            h.insert(Language::German, "Unbekannt");
            h
        });
        
        m.insert("no_description", {
            let mut h = HashMap::new();
            h.insert(Language::English, "No description");
            h.insert(Language::German, "Keine Beschreibung");
            h
        });
        
        m.insert("description", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Description");
            h.insert(Language::German, "Beschreibung");
            h
        });
        
        m.insert("no_description_available", {
            let mut h = HashMap::new();
            h.insert(Language::English, "No description available");
            h.insert(Language::German, "Keine Beschreibung verfügbar");
            h
        });
        
        // Admin Setup
        m.insert("setup_complete", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Setup Already Complete");
            h.insert(Language::German, "Einrichtung bereits abgeschlossen");
            h
        });
        
        m.insert("setup_complete_desc", {
            let mut h = HashMap::new();
            h.insert(Language::English, "An administrator account already exists. You can now log in with your credentials.");
            h.insert(Language::German, "Ein Administratorkonto existiert bereits. Sie können sich jetzt mit Ihren Anmeldedaten anmelden.");
            h
        });
        
        m.insert("go_to_login", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Go to Login");
            h.insert(Language::German, "Zum Login");
            h
        });
        
        m.insert("initial_setup", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Initial Setup");
            h.insert(Language::German, "Ersteinrichtung");
            h
        });
        
        m.insert("create_admin", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Create Admin Account");
            h.insert(Language::German, "Administratorkonto erstellen");
            h
        });
        
        m.insert("create_admin_desc", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Create your first administrator account to get started.");
            h.insert(Language::German, "Erstellen Sie Ihr erstes Administratorkonto, um zu beginnen.");
            h
        });
        
        // Errors
        m.insert("error_loading", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Error loading data");
            h.insert(Language::German, "Fehler beim Laden der Daten");
            h
        });
        
        m.insert("invalid_credentials", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Invalid credentials");
            h.insert(Language::German, "Ungültige Anmeldedaten");
            h
        });
        
        m.insert("passwords_match", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Passwords must match");
            h.insert(Language::German, "Passwörter müssen übereinstimmen");
            h
        });
        
        m.insert("fill_all_fields", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Please fill in all fields");
            h.insert(Language::German, "Bitte füllen Sie alle Felder aus");
            h
        });
        
        m.insert("password_short", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Password must be at least 8 characters");
            h.insert(Language::German, "Passwort muss mindestens 8 Zeichen lang sein");
            h
        });
        
        // Success
        m.insert("saved", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Saved");
            h.insert(Language::German, "Gespeichert");
            h
        });
        
        m.insert("created", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Created");
            h.insert(Language::German, "Erstellt");
            h
        });
        
        m.insert("deleted", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Deleted");
            h.insert(Language::German, "Gelöscht");
            h
        });
        
        m.insert("updated", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Updated");
            h.insert(Language::German, "Aktualisiert");
            h
        });
        
        // Time
        m.insert("updated_at", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Updated");
            h.insert(Language::German, "Aktualisiert");
            h
        });
        
        // Categories
        m.insert("category", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Category");
            h.insert(Language::German, "Kategorie");
            h
        });
        
        m.insert("id", {
            let mut h = HashMap::new();
            h.insert(Language::English, "ID");
            h.insert(Language::German, "ID");
            h
        });
        
        // Settings page
        m.insert("account_settings", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Account Settings");
            h.insert(Language::German, "Kontoeinstellungen");
            h
        });
        
        m.insert("manage_profile_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Manage your profile and password");
            h.insert(Language::German, "Verwalten Sie Ihr Profil und Passwort");
            h
        });
        
        m.insert("profile_information", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Profile Information");
            h.insert(Language::German, "Profilinformationen");
            h
        });
        
        m.insert("display_name", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Display Name");
            h.insert(Language::German, "Anzeigename");
            h
        });
        
        m.insert("enter_display_name", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Enter your display name");
            h.insert(Language::German, "Geben Sie Ihren Anzeigenamen ein");
            h
        });
        
        m.insert("email_address", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Email Address");
            h.insert(Language::German, "E-Mail-Adresse");
            h
        });
        
        m.insert("enter_email", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Enter your email");
            h.insert(Language::German, "Geben Sie Ihre E-Mail ein");
            h
        });
        
        m.insert("passwords_do_not_match", {
            let mut h = HashMap::new();
            h.insert(Language::English, "New passwords do not match");
            h.insert(Language::German, "Neue Passwörter stimmen nicht überein");
            h
        });
        
        m.insert("current_password_required", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Current password is required to change password");
            h.insert(Language::German, "Aktuelles Passwort ist erforderlich, um das Passwort zu ändern");
            h
        });
        
        m.insert("profile_updated", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Profile updated successfully");
            h.insert(Language::German, "Profil erfolgreich aktualisiert");
            h
        });
        
        m
    };
}

pub fn t(key: &str, lang: Language) -> String {
    TRANSLATIONS
        .get(key)
        .and_then(|translations| translations.get(&lang).copied())
        .unwrap_or(key)
        .to_string()
}

// Helper macro for using translations in Yew components
#[macro_export]
macro_rules! tr {
    ($key:expr, $lang:expr) => {
        $crate::i18n::t($key, $lang)
    };
}
