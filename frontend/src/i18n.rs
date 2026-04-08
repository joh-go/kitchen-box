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
            h.insert(Language::German, "Start");
            h
        });
        
        m.insert("welcome_back", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Welcome back, {}!");
            h.insert(Language::German, "Willkommen zurück, {}!");
            h
        });
        
        // Admin Setup Page
        m.insert("create_administrator_account", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Create Administrator Account");
            h.insert(Language::German, "Administratorkonto erstellen");
            h
        });
        
        m.insert("passwords_do_not_match", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Passwords do not match");
            h.insert(Language::German, "Passwörter stimmen nicht überein");
            h
        });
        
        m.insert("failed_to_create_admin", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Failed to create admin: {}");
            h.insert(Language::German, "Administrator konnte nicht erstellt werden: {}");
            h
        });
        
        m.insert("enter_your_name", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Enter your name");
            h.insert(Language::German, "Ihren Namen eingeben");
            h
        });
        
        m.insert("admin_example_email", {
            let mut h = HashMap::new();
            h.insert(Language::English, "admin@example.com");
            h.insert(Language::German, "admin@beispiel.de");
            h
        });
        
        m.insert("create_strong_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Create a strong password");
            h.insert(Language::German, "Ein starkes Passwort erstellen");
            h
        });
        
        m.insert("confirm_your_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Confirm your password");
            h.insert(Language::German, "Passwort bestätigen");
            h
        });
        
        m.insert("creating_admin", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Creating Admin...");
            h.insert(Language::German, "Administrator wird erstellt...");
            h
        });
        
        m.insert("setup_complete", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Setup Complete!");
            h.insert(Language::German, "Einrichtung abgeschlossen!");
            h
        });
        
        m.insert("admin_created_success", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Your administrator account has been created successfully. You can now log in and start using Kitchenbox.");
            h.insert(Language::German, "Ihr Administratorkonto wurde erfolgreich erstellt. Sie können sich jetzt anmelden und Kitchenbox nutzen.");
            h
        });
        
        m.insert("go_to_login", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Go to Login");
            h.insert(Language::German, "Zum Login");
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
            h.insert(Language::English, "No recipes yet - start cooking!");
            h.insert(Language::German, "Noch keine Rezepte - fangen Sie an zu kochen!");
            h
        });
        
        m.insert("create_first", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Create your first recipe");
            h.insert(Language::German, "Erstellen Sie Ihr erstes Rezept");
            h
        });
        
        m.insert("add_first_recipe", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Add Your First Recipe");
            h.insert(Language::German, "Fügen Sie Ihr erstes Rezept hinzu");
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
        
        m.insert("reset", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Reset");
            h.insert(Language::German, "Zurücksetzen");
            h
        });
        
        m.insert("instructions", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Instructions");
            h.insert(Language::German, "Anweisungen");
            h
        });
        
        m.insert("gallery", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Gallery");
            h.insert(Language::German, "Galerie");
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
        
        m.insert("manage_admin_settings", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Manage Admin Settings");
            h.insert(Language::German, "Admin-Einstellungen verwalten");
            h
        });
        
        m.insert("admin_panel", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Admin Panel");
            h.insert(Language::German, "Admin-Panel");
            h
        });
        
        m.insert("user_management_title", {
            let mut h = HashMap::new();
            h.insert(Language::English, "User Management");
            h.insert(Language::German, "Benutzerverwaltung");
            h
        });
        
        m.insert("system_statistics", {
            let mut h = HashMap::new();
            h.insert(Language::English, "System Statistics");
            h.insert(Language::German, "Systemstatistiken");
            h
        });
        
        m.insert("view_stats", {
            let mut h = HashMap::new();
            h.insert(Language::English, "View Stats");
            h.insert(Language::German, "Statistiken anzeigen");
            h
        });
        
        m.insert("admin_privileges", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Admin Privileges");
            h.insert(Language::German, "Admin-Rechte");
            h
        });
        
        m.insert("admin_privileges_desc", {
            let mut h = HashMap::new();
            h.insert(Language::English, "You have administrative access. Use these features responsibly to manage the system and users.");
            h.insert(Language::German, "Sie haben administrativen Zugriff. Nutzen Sie diese Funktionen verantwortungsvoll, um das System und die Benutzer zu verwalten.");
            h
        });
        
        m.insert("view_system_stats", {
            let mut h = HashMap::new();
            h.insert(Language::English, "View system usage and statistics");
            h.insert(Language::German, "Systemnutzung und Statistiken anzeigen");
            h
        });
        
        m.insert("manage_categories_button", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Manage Categories");
            h.insert(Language::German, "Kategorien verwalten");
            h
        });
        
        m.insert("saving", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Saving...");
            h.insert(Language::German, "Wird gespeichert...");
            h
        });
        
        m.insert("save_changes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Save Changes");
            h.insert(Language::German, "Änderungen speichern");
            h
        });
        
        // Admin Users Page
        m.insert("user_management_title", {
            let mut h = HashMap::new();
            h.insert(Language::English, "User Management");
            h.insert(Language::German, "Benutzerverwaltung");
            h
        });
        
        m.insert("manage_user_accounts_permissions", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Manage user accounts and permissions");
            h.insert(Language::German, "Benutzerkonten und Berechtigungen verwalten");
            h
        });
        
        m.insert("add_user", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Add User");
            h.insert(Language::German, "Benutzer hinzufügen");
            h
        });
        
        m.insert("user_column", {
            let mut h = HashMap::new();
            h.insert(Language::English, "User");
            h.insert(Language::German, "Benutzer");
            h
        });
        
        m.insert("role_column", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Role");
            h.insert(Language::German, "Rolle");
            h
        });
        
        m.insert("actions_column", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Actions");
            h.insert(Language::German, "Aktionen");
            h
        });
        
        m.insert("edit_user_title", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Edit User");
            h.insert(Language::German, "Benutzer bearbeiten");
            h
        });
        
        m.insert("create_user_title", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Create User");
            h.insert(Language::German, "Benutzer erstellen");
            h
        });
        
        m.insert("enter_name", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Enter name");
            h.insert(Language::German, "Name eingeben");
            h
        });
        
        m.insert("enter_email", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Enter email");
            h.insert(Language::German, "E-Mail eingeben");
            h
        });
        
        m.insert("leave_blank_keep_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Leave blank to keep current password");
            h.insert(Language::German, "Leer lassen, um aktuelles Passwort zu behalten");
            h
        });
        
        m.insert("enter_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Enter password");
            h.insert(Language::German, "Passwort eingeben");
            h
        });
        
        m.insert("administrator_label", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Administrator");
            h.insert(Language::German, "Administrator");
            h
        });
        
        m.insert("delete_user_title", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Delete User");
            h.insert(Language::German, "Benutzer löschen");
            h
        });
        
        m.insert("delete_user_confirmation", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Are you sure you want to delete this user? This action cannot be undone.");
            h.insert(Language::German, "Möchten Sie diesen Benutzer wirklich löschen? Diese Aktion kann nicht rückgängig gemacht werden.");
            h
        });
        
        m.insert("name_required", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Name is required");
            h.insert(Language::German, "Name ist erforderlich");
            h
        });
        
        m.insert("email_required", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Email is required");
            h.insert(Language::German, "E-Mail ist erforderlich");
            h
        });
        
        m.insert("password_required_new_users", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Password is required for new users");
            h.insert(Language::German, "Passwort ist für neue Benutzer erforderlich");
            h
        });
        
        m.insert("invalid_action", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Invalid action");
            h.insert(Language::German, "Ungültige Aktion");
            h
        });
        
        m.insert("failed_parse_users", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Failed to parse users data");
            h.insert(Language::German, "Benutzerdaten konnten nicht verarbeitet werden");
            h
        });
        
        // Admin Recipes Page
        m.insert("recipe_management_title", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Recipe Management");
            h.insert(Language::German, "Rezeptverwaltung");
            h
        });
        
        m.insert("manage_all_recipes_system", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Manage all recipes in system");
            h.insert(Language::German, "Alle Rezepte im System verwalten");
            h
        });
        
        m.insert("recipe_column", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Recipe");
            h.insert(Language::German, "Rezept");
            h
        });
        
        m.insert("author_column", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Author");
            h.insert(Language::German, "Autor");
            h
        });
        
        m.insert("status_column", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Status");
            h.insert(Language::German, "Status");
            h
        });
        
        m.insert("delete_recipe_title", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Delete Recipe");
            h.insert(Language::German, "Rezept löschen");
            h
        });
        
        m.insert("no_description", {
            let mut h = HashMap::new();
            h.insert(Language::English, "No description");
            h.insert(Language::German, "Keine Beschreibung");
            h
        });
        
        m.insert("no_description_available", {
            let mut h = HashMap::new();
            h.insert(Language::English, "No description available");
            h.insert(Language::German, "Keine Beschreibung verfügbar");
            h
        });
        
        m.insert("description_label", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Description: ");
            h.insert(Language::German, "Beschreibung: ");
            h
        });
        
        m.insert("author_label", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Author: ");
            h.insert(Language::German, "Autor: ");
            h
        });
        
        m.insert("status_label", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Status: ");
            h.insert(Language::German, "Status: ");
            h
        });
        
        m.insert("created_label", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Created: ");
            h.insert(Language::German, "Erstellt: ");
            h
        });
        
        m.insert("updated_label", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Updated: ");
            h.insert(Language::German, "Aktualisiert: ");
            h
        });
        
        m.insert("failed_parse_recipes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Failed to parse recipes data");
            h.insert(Language::German, "Rezeptdaten konnten nicht verarbeitet werden");
            h
        });
        
        // Admin Categories Page
        m.insert("category_management_title", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Category Management");
            h.insert(Language::German, "Kategorieverwaltung");
            h
        });
        
        m.insert("manage_all_categories_system", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Manage all categories in the system");
            h.insert(Language::German, "Alle Kategorien im System verwalten");
            h
        });
        
        m.insert("add_new_category", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Add New Category");
            h.insert(Language::German, "Neue Kategorie hinzufügen");
            h
        });
        
        m.insert("enter_category_name", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Enter category name...");
            h.insert(Language::German, "Kategoriename eingeben...");
            h
        });
        
        m.insert("creating_category", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Creating...");
            h.insert(Language::German, "Wird erstellt...");
            h
        });
        
        m.insert("add_category_button", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Add Category");
            h.insert(Language::German, "Kategorie hinzufügen");
            h
        });
        
        m.insert("delete_category_title", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Delete Category");
            h.insert(Language::German, "Kategorie löschen");
            h
        });
        
        m.insert("name_label", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Name: ");
            h.insert(Language::German, "Name: ");
            h
        });
        
        m.insert("id_label", {
            let mut h = HashMap::new();
            h.insert(Language::English, "ID: ");
            h.insert(Language::German, "ID: ");
            h
        });
        
        m.insert("failed_parse_categories", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Failed to parse categories data");
            h.insert(Language::German, "Kategoriedaten konnten nicht verarbeitet werden");
            h
        });
        
        // Login page
        m.insert("sign_in_account", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Sign in to your account");
            h.insert(Language::German, "Melden Sie sich bei Ihrem Konto an");
            h
        });
        
        m.insert("email_address", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Email address");
            h.insert(Language::German, "E-Mail-Adresse");
            h
        });
        
        m.insert("enter_email", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Enter your email");
            h.insert(Language::German, "Geben Sie Ihre E-Mail ein");
            h
        });
        
        m.insert("enter_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Enter your password");
            h.insert(Language::German, "Geben Sie Ihr Passwort ein");
            h
        });
        
        m.insert("remember_me", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Remember me");
            h.insert(Language::German, "Angemeldet bleiben");
            h
        });
        
        m.insert("forgot_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Forgot your password?");
            h.insert(Language::German, "Passwort vergessen?");
            h
        });
        
        m.insert("signing_in", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Signing in...");
            h.insert(Language::German, "Anmeldung...");
            h
        });
        
        m.insert("dont_have_account", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Don't have an account?");
            h.insert(Language::German, "Haben Sie kein Konto?");
            h
        });
        
        m.insert("sign_up", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Sign up");
            h.insert(Language::German, "Registrieren");
            h
        });
        
        m.insert("failed_auth_token", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Failed to get authentication token");
            h.insert(Language::German, "Authentifizierungs-Token konnte nicht abgerufen werden");
            h
        });
        
        m.insert("failed_user_info", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Failed to get user info");
            h.insert(Language::German, "Benutzerinformationen konnten nicht abgerufen werden");
            h
        });
        
        m.insert("failed_user_id", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Failed to get user ID");
            h.insert(Language::German, "Benutzer-ID konnte nicht abgerufen werden");
            h
        });
        
        // Register page
        m.insert("create_account", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Create Account");
            h.insert(Language::German, "Konto erstellen");
            h
        });
        
        m.insert("join_community", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Join our recipe community");
            h.insert(Language::German, "Treten Sie unserer Rezept-Community bei");
            h
        });
        
        m.insert("full_name", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Full Name");
            h.insert(Language::German, "Vollständiger Name");
            h
        });
        
        m.insert("enter_full_name", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Enter your full name");
            h.insert(Language::German, "Geben Sie Ihren vollständigen Namen ein");
            h
        });
        
        m.insert("confirm_password_label", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Confirm Password");
            h.insert(Language::German, "Passwort bestätigen");
            h
        });
        
        m.insert("confirm_password_placeholder", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Confirm your password");
            h.insert(Language::German, "Bestätigen Sie Ihr Passwort");
            h
        });
        
        m.insert("creating_account", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Creating account...");
            h.insert(Language::German, "Konto wird erstellt...");
            h
        });
        
        m.insert("already_have_account", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Already have an account?");
            h.insert(Language::German, "Haben Sie bereits ein Konto?");
            h
        });
        
        m.insert("logged_in", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Logged in");
            h.insert(Language::German, "Angemeldet");
            h
        });
        
        // Recipe form
        m.insert("ingredients_one_per_line", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Ingredients (one per line)");
            h.insert(Language::German, "Zutaten (eine pro Zeile)");
            h
        });
        
        m.insert("ingredient_format", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Format: [amount] [unit] [name] (optional notes)");
            h.insert(Language::German, "Format: [Menge] [Einheit] [Name] (optionale Notizen)");
            h
        });
        
        m.insert("ingredient_examples", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Examples: 2 cups flour, 1 tsp salt, 3 eggs (large)");
            h.insert(Language::German, "Beispiele: 2 Tassen Mehl, 1 TL Salz, 3 Eier (groß)");
            h
        });
        
        m.insert("steps_one_per_line", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Steps (one per line)");
            h.insert(Language::German, "Schritte (eine pro Zeile)");
            h
        });
        
        m.insert("prep_time_minutes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Prep Time (minutes)");
            h.insert(Language::German, "Vorbereitungszeit (Minuten)");
            h
        });
        
        m.insert("cook_time_minutes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Cook Time (minutes)");
            h.insert(Language::German, "Kochzeit (Minuten)");
            h
        });
        
        m.insert("servings", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Servings");
            h.insert(Language::German, "Portionen");
            h
        });
        
        m.insert("servings_label", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Servings:");
            h.insert(Language::German, "Portionen:");
            h
        });
        
        m.insert("passwords_do_not_match", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Passwords do not match");
            h.insert(Language::German, "Passwörter stimmen nicht überein");
            h
        });
        
        m.insert("current_password_required", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Current password is required");
            h.insert(Language::German, "Aktuelles Passwort ist erforderlich");
            h
        });
        
        m.insert("change_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Change Password");
            h.insert(Language::German, "Passwort ändern");
            h
        });
        
        m.insert("current_password_label", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Current Password");
            h.insert(Language::German, "Aktuelles Passwort");
            h
        });
        
        m.insert("new_password_label", {
            let mut h = HashMap::new();
            h.insert(Language::English, "New Password");
            h.insert(Language::German, "Neues Passwort");
            h
        });
        
        m.insert("enter_current_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Enter current password");
            h.insert(Language::German, "Aktuelles Passwort eingeben");
            h
        });
        
        m.insert("enter_new_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Enter new password");
            h.insert(Language::German, "Neues Passwort eingeben");
            h
        });
        
        m.insert("confirm_new_password", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Confirm New Password");
            h.insert(Language::German, "Neues Passwort bestätigen");
            h
        });
        
        m.insert("notes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Notes");
            h.insert(Language::German, "Notizen");
            h
        });
        
        m.insert("category_optional", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Category (optional)");
            h.insert(Language::German, "Kategorie (optional)");
            h
        });
        
        m.insert("select_category", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Select category");
            h.insert(Language::German, "Kategorie auswählen");
            h
        });
        
        m.insert("none_category", {
            let mut h = HashMap::new();
            h.insert(Language::English, "none");
            h.insert(Language::German, "keine");
            h
        });
        
        m.insert("new_category_name", {
            let mut h = HashMap::new();
            h.insert(Language::English, "New category name");
            h.insert(Language::German, "Neuer Kategoriename");
            h
        });
        
        // Recipe list
        m.insert("your_recipes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Your Recipe Collection");
            h.insert(Language::German, "Ihre Rezeptsammlung");
            h
        });
        
        m.insert("delicious_recipes_count", {
            let mut h = HashMap::new();
            h.insert(Language::English, "wonderful recipes");
            h.insert(Language::German, "köstliche Rezepte");
            h
        });
        
        m.insert("filter_by_category", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Filter by category:");
            h.insert(Language::German, "Nach Kategorie filtern:");
            h
        });
        
        m.insert("all_categories", {
            let mut h = HashMap::new();
            h.insert(Language::English, "All Categories");
            h.insert(Language::German, "Alle Kategorien");
            h
        });
        
        m.insert("search_recipes_placeholder", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Search recipes...");
            h.insert(Language::German, "Rezepte suchen...");
            h
        });
        
        m.insert("error_loading_recipes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Error loading recipes");
            h.insert(Language::German, "Fehler beim Laden der Rezepte");
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
        
        // Admin setup page
        m.insert("please_enter_name", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Please enter your name");
            h.insert(Language::German, "Bitte geben Sie Ihren Namen ein");
            h
        });
        
        m.insert("please_enter_email", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Please enter your email");
            h.insert(Language::German, "Bitte geben Sie Ihre E-Mail ein");
            h
        });
        
        m.insert("password_min_chars", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Password must be at least 6 characters");
            h.insert(Language::German, "Passwort muss mindestens 6 Zeichen lang sein");
            h
        });
        
        m.insert("kitchenbox_setup", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Kitchenbox Setup");
            h.insert(Language::German, "Kitchenbox Einrichtung");
            h
        });
        
        m.insert("setup_description", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Let's get your recipe manager configured");
            h.insert(Language::German, "Lassen Sie uns Ihren Rezept-Manager konfigurieren");
            h
        });
        
        m.insert("welcome_kitchenbox", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Welcome to Kitchenbox!");
            h.insert(Language::German, "Willkommen bei Kitchenbox!");
            h
        });
        
        m.insert("setup_welcome_message", {
            let mut h = HashMap::new();
            h.insert(Language::English, "This appears to be your first time running Kitchenbox. Let's create an administrator account to get you started.");
            h.insert(Language::German, "Dies scheint Ihr erster Start von Kitchenbox zu sein. Lassen Sie uns ein Administratorkonto erstellen, um loszulegen.");
            h
        });
        
        m.insert("manage_users_recipes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Manage all users and recipes");
            h.insert(Language::German, "Alle Benutzer und Rezepte verwalten");
            h
        });
        
        m.insert("configure_system", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Configure system settings");
            h.insert(Language::German, "Systemeinstellungen konfigurieren");
            h
        });
        
        m.insert("full_access_features", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Full access to all features");
            h.insert(Language::German, "Vollzugriff auf alle Funktionen");
            h
        });
        
        m.insert("get_started", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Get Started");
            h.insert(Language::German, "Loslegen");
            h
        });
        
        m.insert("add_category", {
            let mut h = HashMap::new();
            h.insert(Language::English, "+ Add");
            h.insert(Language::German, "+ Hinzufügen");
            h
        });
        
        m.insert("save", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Save");
            h.insert(Language::German, "Speichern");
            h
        });
        
        m.insert("cancel", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Cancel");
            h.insert(Language::German, "Abbrechen");
            h
        });
        
        m.insert("ingredient_examples_placeholder", {
            let mut h = HashMap::new();
            h.insert(Language::English, "2 cups flour\n1 tsp salt\n3 eggs (large)\n1 cup milk (whole)\n2 tbsp olive oil (extra virgin)");
            h.insert(Language::German, "2 Tassen Mehl\n1 TL Salz\n3 Eier (groß)\n1 Tasse Milch (Vollmilch)\n2 EL Olivenöl (extra vergine)");
            h
        });
        
        // Image manager
        m.insert("images", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Images");
            h.insert(Language::German, "Bilder");
            h
        });
        
        m.insert("uploading", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Uploading...");
            h.insert(Language::German, "Wird hochgeladen...");
            h
        });
        
        m.insert("choose_image", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Choose Image");
            h.insert(Language::German, "Bild auswählen");
            h
        });
        
        m.insert("upload_images_description", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Upload images for your recipe (JPG, PNG, etc.)");
            h.insert(Language::German, "Laden Sie Bilder für Ihr Rezept hoch (JPG, PNG, etc.)");
            h
        });
        
        m.insert("failed_upload_image", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Failed to upload image");
            h.insert(Language::German, "Bild konnte nicht hochgeladen werden");
            h
        });
        
        m.insert("no_images_uploaded", {
            let mut h = HashMap::new();
            h.insert(Language::English, "No images uploaded yet");
            h.insert(Language::German, "Noch keine Bilder hochgeladen");
            h
        });
        
        m.insert("primary", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Primary");
            h.insert(Language::German, "Hauptbild");
            h
        });
        
        m.insert("set_as_primary", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Set as primary");
            h.insert(Language::German, "Als Hauptbild festlegen");
            h
        });
        
        m.insert("delete_image", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Delete image");
            h.insert(Language::German, "Bild löschen");
            h
        });
        
        // Users page
        m.insert("manage_users", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Manage Users");
            h.insert(Language::German, "Benutzer verwalten");
            h
        });
        
        m.insert("view_delete_recipes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "View and delete any recipe in the system");
            h.insert(Language::German, "Alle Rezepte ansehen und löschen");
            h
        });
        
        m.insert("manage_recipes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Manage Recipes");
            h.insert(Language::German, "Rezepte verwalten");
            h
        });
        
        m.insert("category_management", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Category Management");
            h.insert(Language::German, "Kategorienverwaltung");
            h
        });
        
        m.insert("manage_categories", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Manage recipe categories");
            h.insert(Language::German, "Rezeptkategorien verwalten");
            h
        });
        
        // Recipe view page
        m.insert("back_to_recipes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Back to Recipes");
            h.insert(Language::German, "Zurück zu Rezepten");
            h
        });
        
        m.insert("edit_recipe", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Edit Recipe");
            h.insert(Language::German, "Rezept bearbeiten");
            h
        });
        
        m.insert("prep_minutes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Prep: {} min");
            h.insert(Language::German, "Vorbereitung: {} Min");
            h
        });
        
        m.insert("cook_minutes", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Cook: {} min");
            h.insert(Language::German, "Kochzeit: {} Min");
            h
        });
        
        m.insert("adjusted_for_servings", {
            let mut h = HashMap::new();
            h.insert(Language::English, "(adjusted for {} servings)");
            h.insert(Language::German, "(angepasst für {} Portionen)");
            h
        });
        
        m.insert("category", {
            let mut h = HashMap::new();
            h.insert(Language::English, "Category");
            h.insert(Language::German, "Kategorie");
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
