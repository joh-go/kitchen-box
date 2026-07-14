#!/usr/bin/env bash
#
# Seed-Script für lokale Entwicklung
# Legt den Benutzer "joe" an und generiert Beispiel-Rezepte.
#
# Voraussetzung: Das Backend läuft unter http://localhost:8000
# und die Datenbank ist migriert (passiert automatisch beim Backend-Start).
#

set -euo pipefail

BASE_URL="${1:-http://localhost:8000}"
API="$BASE_URL/api"

echo "=== Kitchen Box Seed Script ==="
echo "Base URL: $BASE_URL"
echo ""

# ------------------------------------------------------------------
# 1. Benutzer "joe" anlegen (POST /api/users – keine Auth nötig)
# ------------------------------------------------------------------
echo ">>> Lege Benutzer 'joe' an (joe / 12345678) …"

USER_RESPONSE=$(curl -s -X POST "$API/users" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "joe",
    "password": "12345678"
  }')

echo "$USER_RESPONSE" | python3 -m json.tool 2>/dev/null || echo "$USER_RESPONSE"
echo ""

# ------------------------------------------------------------------
# 2. Als joe einloggen (JWT-Token holen)
# ------------------------------------------------------------------
echo ">>> Logge joe ein …"

LOGIN_RESPONSE=$(curl -s -X POST "$API/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "joe",
    "password": "12345678"
  }')

TOKEN=$(echo "$LOGIN_RESPONSE" | python3 -c "import sys,json; print(json.load(sys.stdin)['token'])" 2>/dev/null || true)

if [ -z "$TOKEN" ]; then
  echo "FEHLER: Konnte keinen Token abrufen. Login-Response:"
  echo "$LOGIN_RESPONSE"
  exit 1
fi

echo "Token erhalten: ${TOKEN:0:20}…"
echo ""

# ------------------------------------------------------------------
# 3. Beispiel-Kategorien anlegen
# ------------------------------------------------------------------
echo ">>> Lege Kategorien an …"

create_category() {
  local name="$1"
  local slug="$2"
  local desc="$3"
  curl -s -X POST "$API/categories" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $TOKEN" \
    -d "{
      \"name\": \"$name\",
      \"slug\": \"$slug\",
      \"description\": \"$desc\",
      \"position\": 0
    }" > /dev/null
  echo "  ✓ Kategorie: $name"
}

create_category "Frühstück" "fruehstueck" "Leckere Frühstücksideen"
create_category "Hauptgerichte" "hauptgerichte" "Herzhafte Hauptgerichte"
create_category "Suppen & Eintöpfe" "suppen-eintoepfe" "Wärmende Suppen und Eintöpfe"
create_category "Salate" "salate" "Frische Salate"
create_category "Desserts" "desserts" "Süße Nachspeisen"
create_category "Backen" "backen" "Brot, Kuchen & Gebäck"
create_category "Vorspeisen" "vorspeisen" "Kleine Gerichte für den Start"
create_category "Vegetarisch" "vegetarisch" "Gerichte ohne Fleisch"
echo ""

# ------------------------------------------------------------------
# 4. Beispiel-Rezepte anlegen
# ------------------------------------------------------------------
echo ">>> Lege Beispiel-Rezepte an …"

create_recipe() {
  local data="$1"
  local title
  title=$(echo "$data" | python3 -c "import sys,json; print(json.load(sys.stdin)['title'])" 2>/dev/null)
  
  local response
  response=$(curl -s -w "\n%{http_code}" -X POST "$API/recipes" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $TOKEN" \
    -d "$data")
  
  local http_code
  http_code=$(echo "$response" | tail -1)
  local body
  body=$(echo "$response" | sed '$d')
  
  if [ "$http_code" != "200" ]; then
    echo "  ✗ FEHLER bei '$title' (HTTP $http_code): $body"
  else
    local recipe_id
    recipe_id=$(echo "$body" | python3 -c "import sys,json; print(json.load(sys.stdin).get('id', '?'))" 2>/dev/null || echo "?")
    echo "  ✓ Rezept '$title' angelegt (ID: $recipe_id)"
  fi
}

# ---- Rezept 1: Klassisches Rührei ----
create_recipe '{
  "title": "Klassisches Rührei",
  "slug": "klassisches-ruehrei",
  "short_description": "Fluffiges Rührei mit frischen Kräutern – perfekt zum Frühstück.",
  "ingredients": [
    {"name": "Eier", "amount": 4, "unit": "Stück", "notes": null},
    {"name": "Milch", "amount": 2, "unit": "EL", "notes": null},
    {"name": "Butter", "amount": 1, "unit": "EL", "notes": null},
    {"name": "Schnittlauch", "amount": 1, "unit": "Bund", "notes": "frisch, gehackt"},
    {"name": "Salz", "amount": 0.5, "unit": "TL", "notes": null},
    {"name": "Pfeffer", "amount": 0.25, "unit": "TL", "notes": "frisch gemahlen"}
  ],
  "steps": [
    "Eier in einer Schüssel aufschlagen, Milch zugeben und mit dem Schneebesen schaumig schlagen.",
    "Butter in einer beschichteten Pfanne bei mittlerer Hitze schmelzen.",
    "Die Eiermasse in die Pfanne geben und langsam stocken lassen. Dabei mit einem Pfannenwender immer wieder vom Rand zur Mitte schieben.",
    "Wenn die Eier noch leicht feucht sind, mit Salz und Pfeffer würzen und den gehackten Schnittlauch unterheben.",
    "Sofort servieren – am besten mit frischem Brot oder Toast."
  ],
  "prep_minutes": 5,
  "cook_minutes": 5,
  "servings": 2,
  "notes": "Nicht zu lange braten – Rührei soll saftig bleiben!",
  "is_public": true,
  "categories": [],
  "images": []
}'

# ---- Rezept 2: Spaghetti Carbonara ----
create_recipe '{
  "title": "Spaghetti Carbonara",
  "slug": "spaghetti-carbonara",
  "short_description": "Cremige Pasta mit Speck und Parmesan – ein italienischer Klassiker.",
  "ingredients": [
    {"name": "Spaghetti", "amount": 400, "unit": "g", "notes": null},
    {"name": "Guanciale oder Pancetta", "amount": 150, "unit": "g", "notes": "in kleinen Würfeln"},
    {"name": "Eier", "amount": 4, "unit": "Stück", "notes": "frisch"},
    {"name": "Parmesan", "amount": 80, "unit": "g", "notes": "frisch gerieben"},
    {"name": "Pfeffer", "amount": 1, "unit": "TL", "notes": "frisch gemahlen"},
    {"name": "Salz", "amount": 0.5, "unit": "TL", "notes": null}
  ],
  "steps": [
    "Spaghetti in reichlich Salzwasser bissfest kochen.",
    "Guanciale in einer kalten Pfanne ohne Öl langsam auslassen, bis er knusprig ist.",
    "Eier, geriebenen Parmesan und Pfeffer in einer Schüssel verquirlen.",
    "Die heißen Spaghetti abgießen und sofort in die Pfanne zum Guanciale geben. Gut vermengen.",
    "Die Pfanne vom Herd nehmen, die Ei-Parmesan-Mischung unterrühren und zügig durchmischen. Die Restwärme der Nudeln lässt die Sauce cremig werden.",
    "Sofort servieren, mit extra Parmesan und Pfeffer bestreuen."
  ],
  "prep_minutes": 10,
  "cook_minutes": 15,
  "servings": 4,
  "notes": "Niemals Sahne verwenden – originale Carbonara kommt ohne aus!",
  "is_public": true,
  "categories": [],
  "images": []
}'

# ---- Rezept 3: Kürbissuppe ----
create_recipe '{
  "title": "Kürbissuppe mit Ingwer",
  "slug": "kuerbissuppe-mit-ingwer",
  "short_description": "Sämige Kürbissuppe mit einem Hauch Ingwer und Kokosmilch.",
  "ingredients": [
    {"name": "Hokkaido-Kürbis", "amount": 1, "unit": "Stück", "notes": "ca. 1 kg"},
    {"name": "Ingwer", "amount": 30, "unit": "g", "notes": "frisch"},
    {"name": "Knoblauchzehen", "amount": 2, "unit": "Stück", "notes": null},
    {"name": "Zwiebel", "amount": 1, "unit": "Stück", "notes": "groß"},
    {"name": "Kokosmilch", "amount": 200, "unit": "ml", "notes": null},
    {"name": "Gemüsebrühe", "amount": 500, "unit": "ml", "notes": null},
    {"name": "Kürbiskernöl", "amount": 2, "unit": "EL", "notes": "zum Servieren"},
    {"name": "Salz", "amount": 1, "unit": "TL", "notes": null},
    {"name": "Kürbiskerne", "amount": 30, "unit": "g", "notes": "geröstet, zum Garnieren"}
  ],
  "steps": [
    "Kürbis waschen, entkernen und in grobe Würfel schneiden (Hokkaido muss nicht geschält werden).",
    "Zwiebel und Knoblauch fein würfeln, Ingwer schälen und fein reiben.",
    "Zwiebel, Knoblauch und Ingwer in einem großen Topf mit etwas Öl anschwitzen.",
    "Kürbiswürfel zugeben und kurz mitbraten. Mit Gemüsebrühe ablöschen.",
    "Suppe zugedeckt ca. 20 Minuten köcheln lassen, bis der Kürbis weich ist.",
    "Suppe fein pürieren, Kokosmilch unterrühren und mit Salz abschmecken.",
    "Mit Kürbiskernöl beträufeln und mit gerösteten Kürbiskernen garnieren."
  ],
  "prep_minutes": 15,
  "cook_minutes": 25,
  "servings": 4,
  "notes": "Wer es etwas schärfer mag, kann eine Chilischote mitkochen.",
  "is_public": true,
  "categories": [],
  "images": []
}'

# ---- Rezept 4: Griechischer Bauernsalat ----
create_recipe '{
  "title": "Griechischer Bauernsalat",
  "slug": "griechischer-bauernsalat",
  "short_description": "Frischer Bauernsalat mit Schafskäse und Oliven – schnell und einfach.",
  "ingredients": [
    {"name": "Tomaten", "amount": 4, "unit": "Stück", "notes": "reif"},
    {"name": "Salatgurke", "amount": 1, "unit": "Stück", "notes": null},
    {"name": "Rote Zwiebel", "amount": 1, "unit": "Stück", "notes": null},
    {"name": "Feta (Schafskäse)", "amount": 200, "unit": "g", "notes": null},
    {"name": "Kalamata-Oliven", "amount": 80, "unit": "g", "notes": "schwarz"},
    {"name": "Natives Olivenöl", "amount": 4, "unit": "EL", "notes": null},
    {"name": "Zitronensaft", "amount": 2, "unit": "EL", "notes": null},
    {"name": "Oregano", "amount": 1, "unit": "TL", "notes": "getrocknet"},
    {"name": "Salz", "amount": 0.5, "unit": "TL", "notes": null}
  ],
  "steps": [
    "Tomaten waschen und in Spalten schneiden, Gurke halbieren und in Scheiben schneiden.",
    "Zwiebel in feine Ringe schneiden.",
    "Alles in einer großen Schüssel vermengen.",
    "Olivenöl, Zitronensaft, Oregano und Salz zu einem Dressing verrühren und über den Salat geben.",
    "Feta in groben Stücken darüber bröseln und die Oliven verteilen.",
    "Vor dem Servieren 10 Minuten durchziehen lassen."
  ],
  "prep_minutes": 15,
  "cook_minutes": 0,
  "servings": 3,
  "notes": "Dazu passt frisches Fladenbrot oder Baguette.",
  "is_public": true,
  "categories": [],
  "images": []
}'

# ---- Rezept 5: Schokoladenkuchen ----
create_recipe '{
  "title": "Saftiger Schokoladenkuchen",
  "slug": "saftiger-schokoladenkuchen",
  "short_description": "Ein extrasaftiger Schokoladenkuchen mit Glasur – ein Traum für alle Schoko-Fans.",
  "ingredients": [
    {"name": "Zartbitterschokolade", "amount": 200, "unit": "g", "notes": null},
    {"name": "Butter", "amount": 150, "unit": "g", "notes": null},
    {"name": "Zucker", "amount": 180, "unit": "g", "notes": null},
    {"name": "Eier", "amount": 4, "unit": "Stück", "notes": null},
    {"name": "Mehl", "amount": 120, "unit": "g", "notes": null},
    {"name": "Backpulver", "amount": 1, "unit": "TL", "notes": null},
    {"name": "Vanilleextrakt", "amount": 1, "unit": "TL", "notes": null},
    {"name": "Puderzucker", "amount": 100, "unit": "g", "notes": "für die Glasur"},
    {"name": "Kakaopulver", "amount": 2, "unit": "EL", "notes": null}
  ],
  "steps": [
    "Backofen auf 180 °C Ober-/Unterhitze vorheizen. Eine Kastenform (26 cm) einfetten und mit Mehl ausstauben.",
    "Schokolade grob hacken und mit der Butter im Wasserbad schmelzen. Anschließend etwas abkühlen lassen.",
    "Zucker und Eier schaumig schlagen. Die geschmolzene Schoko-Butter-Mischung und Vanilleextrakt unterrühren.",
    "Mehl und Backpulver mischen und vorsichtig unter den Teig heben.",
    "Teig in die Form füllen und 35–40 Minuten backen (Stäbchenprobe!).",
    "Kuchen auskühlen lassen. Für die Glasur Puderzucker mit Kakaopulver und 2–3 EL Wasser verrühren.",
    "Den abgekühlten Kuchen mit der Glasur überziehen und trocknen lassen."
  ],
  "prep_minutes": 20,
  "cook_minutes": 40,
  "servings": 12,
  "notes": "Der Kuchen schmeckt am nächsten Tag noch besser, da er durchziehen kann.",
  "is_public": true,
  "categories": [],
  "images": []
}'

# ---- Rezept 6: Hähnchen-Curry ----
create_recipe '{
  "title": "Hähnchen-Curry mit Kokosmilch",
  "slug": "haehnchen-curry-kokosmilch",
  "short_description": "Würziges Hähnchen-Curry mit Gemüse und cremiger Kokossauce.",
  "ingredients": [
    {"name": "Hähnchenbrust", "amount": 500, "unit": "g", "notes": "in Würfel geschnitten"},
    {"name": "Kokosmilch", "amount": 400, "unit": "ml", "notes": null},
    {"name": "Rote Currypaste", "amount": 2, "unit": "EL", "notes": null},
    {"name": "Paprika", "amount": 2, "unit": "Stück", "notes": "rot und gelb"},
    {"name": "Zuckerschoten", "amount": 150, "unit": "g", "notes": null},
    {"name": "Zwiebel", "amount": 1, "unit": "Stück", "notes": null},
    {"name": "Knoblauchzehen", "amount": 2, "unit": "Stück", "notes": null},
    {"name": "Ingwer", "amount": 15, "unit": "g", "notes": "frisch"},
    {"name": "Fischsauce", "amount": 1, "unit": "EL", "notes": null},
    {"name": "Limette", "amount": 1, "unit": "Stück", "notes": "Saft und abgeriebene Schale"},
    {"name": "Basilikum", "amount": 1, "unit": "Bund", "notes": "Thai-Basilikum oder normale"}
  ],
  "steps": [
    "Hähnchenbrust in mundgerechte Stücke schneiden, salzen und pfeffern.",
    "Zwiebel, Knoblauch und Ingwer fein hacken. Paprika in Streifen schneiden.",
    "Etwas Öl in einem Wok oder einer großen Pfanne erhitzen. Hähnchen von allen Seiten goldbraun anbraten und herausnehmen.",
    "Zwiebel, Knoblauch und Ingwer im restlichen Öl anschwitzen. Currypaste zugeben und kurz mitbraten.",
    "Kokosmilch angießen und aufkochen. Paprika und Zuckerschoten zugeben, 10 Minuten köcheln lassen.",
    "Hähnchen zurück in die Pfanne geben, Fischsauce und Limettensaft einrühren. Mit Salz abschmecken.",
    "Basilikumblätter unterheben und mit Limettenschale garniert servieren. Dazu passt Jasmin-Reis."
  ],
  "prep_minutes": 15,
  "cook_minutes": 25,
  "servings": 4,
  "notes": "Die Schärfe lässt sich durch die Menge an Currypaste steuern.",
  "is_public": true,
  "categories": [],
  "images": []
}'

# ---- Rezept 7: Pfannkuchen ----
create_recipe '{
  "title": "Lufige Pfannkuchen",
  "slug": "luftige-pfannkuchen",
  "short_description": "Dicke, fluffige Pfannkuchen – perfekt für ein gemütliches Frühstück.",
  "ingredients": [
    {"name": "Mehl", "amount": 250, "unit": "g", "notes": null},
    {"name": "Milch", "amount": 300, "unit": "ml", "notes": null},
    {"name": "Eier", "amount": 2, "unit": "Stück", "notes": null},
    {"name": "Zucker", "amount": 2, "unit": "EL", "notes": null},
    {"name": "Vanillezucker", "amount": 1, "unit": "TL", "notes": null},
    {"name": "Backpulver", "amount": 1, "unit": "TL", "notes": null},
    {"name": "Prise Salz", "amount": 1, "unit": "Prise", "notes": null},
    {"name": "Butter", "amount": 2, "unit": "EL", "notes": "zum Ausbacken"},
    {"name": "Ahornsirup", "amount": 4, "unit": "EL", "notes": "zum Servieren"},
    {"name": "Frische Heidelbeeren", "amount": 100, "unit": "g", "notes": "optional, zum Garnieren"}
  ],
  "steps": [
    "Mehl, Backpulver, Zucker, Vanillezucker und Salz in einer Schüssel mischen.",
    "Milch dazugießen, Eier zugeben und alles mit einem Schneebesen zu einem glatten Teig verrühren.",
    "Etwas Butter in einer beschichteten Pfanne bei mittlerer Hitze zerlaufen lassen.",
    "Eine Kelle Teig in die Pfanne geben und zu einem runden Pfannkuchen formen.",
    "Wenn sich Bläschen auf der Oberfläche bilden (ca. 2 Minuten), den Pfannkuchen wenden und von der anderen Seite goldbraun backen.",
    "Mit Ahornsirup und frischen Heidelbeeren servieren."
  ],
  "prep_minutes": 10,
  "cook_minutes": 15,
  "servings": 4,
  "notes": "Wer mag, kann den Teig mit Zimt verfeinern.",
  "is_public": true,
  "categories": [],
  "images": []
}'

# ---- Rezept 8: Tomatensuppe ----
create_recipe '{
  "title": "Italienische Tomatensuppe",
  "slug": "italienische-tomatensuppe",
  "short_description": "Wärmende Tomatensuppe mit Basilikum und geröstetem Ciabatta.",
  "ingredients": [
    {"name": "Dosentomaten", "amount": 800, "unit": "g", "notes": "gestückelt"},
    {"name": "Zwiebel", "amount": 1, "unit": "Stück", "notes": "groß"},
    {"name": "Knoblauchzehen", "amount": 3, "unit": "Stück", "notes": null},
    {"name": "Karotte", "amount": 1, "unit": "Stück", "notes": null},
    {"name": "Gemüsebrühe", "amount": 300, "unit": "ml", "notes": null},
    {"name": "Olivenöl", "amount": 3, "unit": "EL", "notes": null},
    {"name": "Basilikum", "amount": 1, "unit": "Bund", "notes": "frisch"},
    {"name": "Ciabatta", "amount": 4, "unit": "Scheiben", "notes": "geröstet"},
    {"name": "Salz", "amount": 1, "unit": "TL", "notes": null},
    {"name": "Zucker", "amount": 0.5, "unit": "TL", "notes": null}
  ],
  "steps": [
    "Zwiebel, Knoblauch und Karotte fein würfeln.",
    "Olivenöl in einem Topf erhitzen. Zwiebel und Karotte darin glasig dünsten, Knoblauch kurz mitbraten.",
    "Dosentomaten zugeben und mit der Gemüsebrühe ablöschen.",
    "Suppe 20 Minuten leise köcheln lassen, dann fein pürieren.",
    "Mit Salz, Pfeffer und einer Prise Zucker abschmecken.",
    "Basilikum in feine Streifen schneiden und unterrühren.",
    "Mit gerösteten Ciabatta-Scheiben servieren."
  ],
  "prep_minutes": 10,
  "cook_minutes": 25,
  "servings": 4,
  "notes": "Ein Schuss Sahne macht die Suppe noch cremiger!",
  "is_public": true,
  "categories": [],
  "images": []
}'

# ---- Rezept 9: Bauernomelett ----
create_recipe '{
  "title": "Bauernomelett mit Kartoffeln und Speck",
  "slug": "bauernomelett-mit-kartoffeln-speck",
  "short_description": "Herzhaftes Omelett mit Kartoffeln, Speck und frischen Kräutern.",
  "ingredients": [
    {"name": "Eier", "amount": 6, "unit": "Stück", "notes": null},
    {"name": "Kartoffeln", "amount": 300, "unit": "g", "notes": "festkochend"},
    {"name": "Speck", "amount": 100, "unit": "g", "notes": "gewürfelt"},
    {"name": "Zwiebel", "amount": 1, "unit": "Stück", "notes": null},
    {"name": "Milch", "amount": 3, "unit": "EL", "notes": null},
    {"name": "Petersilie", "amount": 1, "unit": "Bund", "notes": "glatt"},
    {"name": "Butter", "amount": 2, "unit": "EL", "notes": null},
    {"name": "Salz", "amount": 0.5, "unit": "TL", "notes": null},
    {"name": "Pfeffer", "amount": 0.25, "unit": "TL", "notes": null}
  ],
  "steps": [
    "Kartoffeln schälen und in kleine Würfel schneiden. In Salzwasser 5 Minuten vorkochen, abgießen.",
    "Speck in einer ofenfesten Pfanne knusprig braten. Kartoffelwürfel und gehackte Zwiebel zugeben und goldbraun braten.",
    "Eier mit Milch, Salz und Pfeffer verquirlen. Petersilie hacken und unterrühren.",
    "Die Eimasse über die Kartoffeln und den Speck in der Pfanne gießen. Bei niedriger Hitze stocken lassen (ca. 10 Minuten).",
    "Optional: Das Omelett im vorgeheizten Ofen bei 180 °C Oberhitze für 3 Minuten fertig backen.",
    "Mit frischer Petersilie bestreut servieren."
  ],
  "prep_minutes": 15,
  "cook_minutes": 20,
  "servings": 2,
  "notes": "Ein grüner Salat passt hervorragend dazu!",
  "is_public": true,
  "categories": [],
  "images": []
}'

# ---- Rezept 10: Zitronen-Hähnchen ----
create_recipe '{
  "title": "Zitronen-Hähnchen aus dem Ofen",
  "slug": "zitronen-haehnchen-aus-dem-ofen",
  "short_description": "Saftiges Ofenhähnchen mit Zitrone, Knoblauch und Rosmarin.",
  "ingredients": [
    {"name": "Hähnchenkeulen", "amount": 4, "unit": "Stück", "notes": null},
    {"name": "Zitrone", "amount": 2, "unit": "Stück", "notes": "unbehandelt"},
    {"name": "Knoblauchzehen", "amount": 6, "unit": "Stück", "notes": "ungeacht"},
    {"name": "Rosmarin", "amount": 3, "unit": "Zweige", "notes": "frisch"},
    {"name": "Olivenöl", "amount": 4, "unit": "EL", "notes": null},
    {"name": "Kartoffeln", "amount": 600, "unit": "g", "notes": "in Spalten geschnitten"},
    {"name": "Salz", "amount": 1, "unit": "TL", "notes": null},
    {"name": "Pfeffer", "amount": 0.5, "unit": "TL", "notes": null}
  ],
  "steps": [
    "Backofen auf 200 °C Ober-/Unterhitze vorheizen.",
    "Hähnchenkeulen mit Olivenöl, Salz und Pfeffer einreiben. Zitrone in Scheiben schneiden.",
    "Hähnchenkeulen mit den Zitronenscheiben, Knoblauchzehen und Rosmarin in eine Auflaufform geben.",
    "Kartoffelspalten um das Hähnchen herum verteilen und mit etwas Olivenöl beträufeln.",
    "Alles 40–45 Minuten im Ofen backen, bis das Hähnchen goldbraun und durchgegart ist.",
    "Vor dem Servieren kurz ruhen lassen."
  ],
  "prep_minutes": 15,
  "cook_minutes": 45,
  "servings": 4,
  "notes": "Dazu passt ein gemischter grüner Salat.",
  "is_public": true,
  "categories": [],
  "images": []
}'

echo ""
echo "=== Fertig! ==="
echo "Benutzer:      joe / 12345678"
echo "Rezepte:       10 Beispiel-Rezepte angelegt"
echo "Kategorien:    8 Kategorien angelegt"
