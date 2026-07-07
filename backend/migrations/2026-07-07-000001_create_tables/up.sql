CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  is_admin BOOLEAN DEFAULT false,
  created_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS categories (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  slug TEXT NOT NULL,
  description TEXT,
  parent_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
  position INTEGER DEFAULT 0,
  created_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS recipes (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  slug TEXT NOT NULL,
  short_description TEXT,
  ingredients JSONB NOT NULL,
  steps JSONB NOT NULL,
  prep_minutes INTEGER,
  cook_minutes INTEGER,
  servings INTEGER,
  notes TEXT,
  author_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
  is_public BOOLEAN DEFAULT true,
  created_at TIMESTAMPTZ DEFAULT now(),
  updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS recipe_categories (
  recipe_id INTEGER REFERENCES recipes(id) ON DELETE CASCADE,
  category_id INTEGER REFERENCES categories(id) ON DELETE CASCADE,
  PRIMARY KEY (recipe_id, category_id)
);

CREATE TABLE IF NOT EXISTS images (
  id SERIAL PRIMARY KEY,
  recipe_id INTEGER REFERENCES recipes(id) ON DELETE CASCADE,
  filename TEXT NOT NULL,
  original_filename TEXT,
  file_path TEXT NOT NULL,
  file_size INTEGER,
  mime_type TEXT,
  alt TEXT,
  is_primary BOOLEAN DEFAULT false,
  position INTEGER DEFAULT 0,
  uploaded_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS recipe_versions (
  id SERIAL PRIMARY KEY,
  recipe_id INTEGER REFERENCES recipes(id) ON DELETE CASCADE,
  payload JSONB,
  created_at TIMESTAMPTZ DEFAULT now(),
  author_id INTEGER REFERENCES users(id) ON DELETE SET NULL
);
