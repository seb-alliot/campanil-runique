-- ============================================================================
-- SCHEMA SQL POSTGRESQL — U CAMPANILE
-- ============================================================================

SET client_encoding = 'UTF8';

-- ============================================================================
-- TYPES ENUM
-- ============================================================================

CREATE TYPE statut_avis AS ENUM ('en_attente', 'valide', 'refuse');
CREATE TYPE statut_avis_plat AS ENUM ('en_attente', 'valide', 'refuse');
CREATE TYPE type_boisson AS ENUM ('vin_rouge', 'vin_rose', 'vin_blanc', 'champagne', 'biere', 'aperitif', 'soft', 'eau_bouteille', 'digestif');
CREATE TYPE type_menu AS ENUM ('menu_resto', 'menu_enfant', 'formule_jour');
CREATE TYPE type_retrait AS ENUM ('sur_place', 'livraison');
CREATE TYPE statut_commande AS ENUM ('en_attente', 'accepte', 'en_preparation', 'pret', 'en_cours_livraison', 'livre', 'termine', 'annule');
CREATE TYPE mode_paiement AS ENUM ('especes', 'carte_bancaire', 'en_ligne');
CREATE TYPE type_article AS ENUM ('plat', 'entree', 'dessert', 'menu', 'boisson', 'supplement');
CREATE TYPE cuisson_viande AS ENUM ('bleu', 'saignant', 'a_point', 'bien_cuit');
CREATE TYPE theme_menu AS ENUM ('mariage', 'bapteme', 'anniversaire', 'autre');
CREATE TYPE regime_menu AS ENUM ('standard', 'vegetarien', 'sans_gluten', 'halal', 'casher');
CREATE TYPE statut_devis AS ENUM ('en_attente', 'en_cours', 'accepte', 'refuse');
CREATE TYPE usage_entree AS ENUM ('carte', 'menu', 'les_deux');
CREATE TYPE usage_dessert AS ENUM ('carte', 'menu', 'les_deux');
CREATE TYPE type_plat AS ENUM ('specialite', 'viande', 'poisson', 'plat');
CREATE TYPE usage_plat AS ENUM ('carte', 'menu', 'les_deux');
CREATE TYPE type_garniture AS ENUM ('feculent', 'legumes', 'sauce');
CREATE TYPE raisoncontact AS ENUM ('reservation', 'traiteur', 'commande', 'autre');

-- ============================================================================
-- TABLES FRAMEWORK RUNIQUE
-- ============================================================================

CREATE TABLE eihwaz_users (
    id          SERIAL PRIMARY KEY,
    username    VARCHAR(150) NOT NULL UNIQUE,
    email       VARCHAR(255) NOT NULL UNIQUE,
    password    VARCHAR(255) NOT NULL,
    is_active   BOOLEAN NOT NULL DEFAULT false,
    is_staff    BOOLEAN NOT NULL DEFAULT false,
    is_superuser BOOLEAN NOT NULL DEFAULT false,
    last_login  TIMESTAMP,
    created_at  TIMESTAMP,
    updated_at  TIMESTAMP,
    -- Extension Campanile (via extend!{})
    telephone   VARCHAR(20),
    adresse     VARCHAR(255),
    ville       VARCHAR(100),
    code_postal VARCHAR(10)
);

CREATE TABLE eihwaz_sessions (
    id          VARCHAR(255) PRIMARY KEY,
    data        BYTEA NOT NULL,
    expiry_date TIMESTAMP NOT NULL
);

-- ============================================================================
-- TABLES DE RÉFÉRENCE
-- ============================================================================

CREATE TABLE allergenes (
    id      SERIAL PRIMARY KEY,
    libelle VARCHAR(100) NOT NULL UNIQUE
);

CREATE TABLE garnitures (
    id              SERIAL PRIMARY KEY,
    libelle         VARCHAR(100) NOT NULL,
    type_garniture  type_garniture NOT NULL,
    disponible      BOOLEAN NOT NULL DEFAULT true
);

CREATE TABLE horaires (
    id              SERIAL PRIMARY KEY,
    jour            VARCHAR(20) NOT NULL UNIQUE,
    ouverture_midi  TIME,
    fermeture_midi  TIME,
    ouverture_soir  TIME,
    fermeture_soir  TIME,
    ferme           BOOLEAN NOT NULL DEFAULT false,
    note            VARCHAR(255)
);

CREATE TABLE info_resto (
    id                  SERIAL PRIMARY KEY,
    nom                 VARCHAR(150) NOT NULL,
    adresse             VARCHAR(200) NOT NULL,
    telephone           VARCHAR(20) NOT NULL,
    email               VARCHAR(150),
    periode_ouverture   VARCHAR(100),
    facebook            VARCHAR(255),
    instagram           VARCHAR(255),
    tripadvisor         VARCHAR(255),
    google_maps         VARCHAR(500),
    description         TEXT,
    ville               VARCHAR(100),
    prix_livraison      NUMERIC(10,2),
    latitude            NUMERIC(10,7),
    longitude           NUMERIC(10,7)
);

-- ============================================================================
-- TABLES DE CATALOGUE
-- ============================================================================

CREATE TABLE entrees (
    id          SERIAL PRIMARY KEY,
    titre       VARCHAR(255) NOT NULL,
    label       VARCHAR(80),
    description TEXT,
    image       VARCHAR(255),
    prix        NUMERIC(10,2) NOT NULL,
    disponible  BOOLEAN NOT NULL DEFAULT true,
    usage       usage_entree NOT NULL DEFAULT 'les_deux',
    ordre       INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE plats (
    id          SERIAL PRIMARY KEY,
    titre       VARCHAR(255) NOT NULL,
    label       VARCHAR(80),
    type_plat   type_plat NOT NULL,
    description TEXT,
    image       VARCHAR(255),
    prix        NUMERIC(10,2) NOT NULL,
    disponible  BOOLEAN NOT NULL DEFAULT true,
    est_viande  BOOLEAN NOT NULL DEFAULT false,
    usage       usage_plat NOT NULL DEFAULT 'les_deux',
    ordre       INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE desserts (
    id          SERIAL PRIMARY KEY,
    titre       VARCHAR(255) NOT NULL,
    label       VARCHAR(80),
    description TEXT,
    image       VARCHAR(255),
    prix        NUMERIC(10,2) NOT NULL,
    disponible  BOOLEAN NOT NULL DEFAULT true,
    usage       usage_dessert NOT NULL DEFAULT 'les_deux',
    ordre       INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE boissons (
    id           SERIAL PRIMARY KEY,
    titre        VARCHAR(255) NOT NULL,
    type_boisson type_boisson NOT NULL,
    prix         NUMERIC(10,2) NOT NULL,
    description  TEXT,
    image        VARCHAR(255),
    disponible   BOOLEAN NOT NULL DEFAULT true,
    ordre        INTEGER NOT NULL DEFAULT 0,
    created_at   TIMESTAMP
);

CREATE TABLE supplements (
    id          SERIAL PRIMARY KEY,
    garniture_id INTEGER REFERENCES garnitures(id) ON DELETE SET NULL,
    titre       VARCHAR(255),
    libelle     VARCHAR(500),
    prix        NUMERIC(10,2) NOT NULL,
    disponible  BOOLEAN NOT NULL DEFAULT true,
    ordre       INTEGER NOT NULL DEFAULT 0
);

-- ============================================================================
-- RELATIONS CATALOGUE
-- ============================================================================

CREATE TABLE entree_allergene (
    id          SERIAL PRIMARY KEY,
    entree_id   INTEGER NOT NULL REFERENCES entrees(id) ON DELETE CASCADE,
    allergene_id INTEGER NOT NULL REFERENCES allergenes(id) ON DELETE CASCADE
);

CREATE TABLE plat_allergene (
    id          SERIAL PRIMARY KEY,
    plat_id     INTEGER NOT NULL REFERENCES plats(id) ON DELETE CASCADE,
    allergene_id INTEGER NOT NULL REFERENCES allergenes(id) ON DELETE CASCADE
);

CREATE TABLE dessert_allergene (
    id          SERIAL PRIMARY KEY,
    dessert_id  INTEGER NOT NULL REFERENCES desserts(id) ON DELETE CASCADE,
    allergene_id INTEGER NOT NULL REFERENCES allergenes(id) ON DELETE CASCADE
);

CREATE TABLE plat_garnitures (
    id          SERIAL PRIMARY KEY,
    plat_id     INTEGER NOT NULL REFERENCES plats(id) ON DELETE CASCADE,
    garniture_id INTEGER NOT NULL REFERENCES garnitures(id) ON DELETE CASCADE,
    est_defaut  BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE plat_supplements (
    id           SERIAL PRIMARY KEY,
    plat_id      INTEGER NOT NULL REFERENCES plats(id) ON DELETE CASCADE,
    supplement_id INTEGER NOT NULL REFERENCES supplements(id) ON DELETE CASCADE,
    UNIQUE (plat_id, supplement_id)
);

-- ============================================================================
-- MENUS RESTAURANT
-- ============================================================================

CREATE TABLE menus (
    id            SERIAL PRIMARY KEY,
    type_menu     type_menu NOT NULL DEFAULT 'menu_resto',
    nom           VARCHAR(255) NOT NULL,
    description   TEXT,
    image         VARCHAR(255),
    prix          NUMERIC(10,2) NOT NULL,
    ordre         INTEGER NOT NULL DEFAULT 0,
    entree_libre  VARCHAR(500),
    plat_libre    VARCHAR(500),
    dessert_libre VARCHAR(500)
);

CREATE TABLE menu_entrees (
    id        SERIAL PRIMARY KEY,
    menu_id   INTEGER NOT NULL REFERENCES menus(id) ON DELETE CASCADE,
    entree_id INTEGER NOT NULL REFERENCES entrees(id) ON DELETE CASCADE
);

CREATE TABLE menu_plats (
    id      SERIAL PRIMARY KEY,
    menu_id INTEGER NOT NULL REFERENCES menus(id) ON DELETE CASCADE,
    plat_id INTEGER NOT NULL REFERENCES plats(id) ON DELETE CASCADE
);

CREATE TABLE menu_desserts (
    id         SERIAL PRIMARY KEY,
    menu_id    INTEGER NOT NULL REFERENCES menus(id) ON DELETE CASCADE,
    dessert_id INTEGER NOT NULL REFERENCES desserts(id) ON DELETE CASCADE
);

-- ============================================================================
-- COMMANDES
-- ============================================================================

CREATE TABLE commandes (
    id                       SERIAL PRIMARY KEY,
    numero                   VARCHAR(20) NOT NULL UNIQUE,
    user_id                  INTEGER NOT NULL REFERENCES eihwaz_users(id) ON DELETE RESTRICT,
    statut                   statut_commande NOT NULL DEFAULT 'en_attente',
    mode_paiement            mode_paiement NOT NULL,
    prix_total               NUMERIC(10,2) NOT NULL,
    type_retrait             type_retrait NOT NULL,
    heure_retrait            TIMESTAMP,
    adresse_livraison        VARCHAR(255),
    ville_livraison          VARCHAR(100),
    cp_livraison             VARCHAR(10),
    prix_livraison           NUMERIC(10,2),
    modifiable               BOOLEAN NOT NULL DEFAULT true,
    stripe_payment_intent_id VARCHAR(255),
    motif_annulation         TEXT,
    mode_contact_annulation  VARCHAR(100),
    date_annulation          TIMESTAMP,
    created_at               TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at               TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE commande_lignes (
    id            SERIAL PRIMARY KEY,
    commande_id   INTEGER NOT NULL REFERENCES commandes(id) ON DELETE CASCADE,
    type_article  type_article NOT NULL,
    plat_id       INTEGER REFERENCES plats(id) ON DELETE RESTRICT,
    entree_id     INTEGER REFERENCES entrees(id) ON DELETE RESTRICT,
    dessert_id    INTEGER REFERENCES desserts(id) ON DELETE RESTRICT,
    menu_id       INTEGER REFERENCES menus(id) ON DELETE RESTRICT,
    boisson_id    INTEGER REFERENCES boissons(id) ON DELETE RESTRICT,
    supplement_id INTEGER REFERENCES supplements(id) ON DELETE RESTRICT,
    cuisson       cuisson_viande,
    sans_sel      BOOLEAN NOT NULL DEFAULT false,
    note          VARCHAR(500),
    quantite      INTEGER NOT NULL DEFAULT 1,
    prix_unitaire NUMERIC(10,2) NOT NULL
);

CREATE TABLE commande_ligne_garnitures (
    id                SERIAL PRIMARY KEY,
    commande_ligne_id INTEGER NOT NULL REFERENCES commande_lignes(id) ON DELETE CASCADE,
    garniture_id      INTEGER NOT NULL REFERENCES garnitures(id) ON DELETE RESTRICT
);

CREATE TABLE commande_menu_choix (
    id                SERIAL PRIMARY KEY,
    commande_ligne_id INTEGER NOT NULL REFERENCES commande_lignes(id) ON DELETE CASCADE,
    cours             VARCHAR(20) NOT NULL,
    plat_id           INTEGER REFERENCES plats(id) ON DELETE RESTRICT,
    entree_id         INTEGER REFERENCES entrees(id) ON DELETE RESTRICT,
    dessert_id        INTEGER REFERENCES desserts(id) ON DELETE RESTRICT
);

CREATE TABLE commande_statuts (
    id          SERIAL PRIMARY KEY,
    commande_id INTEGER NOT NULL REFERENCES commandes(id) ON DELETE CASCADE,
    statut      VARCHAR(50) NOT NULL,
    note        TEXT,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- AVIS
-- ============================================================================

CREATE TABLE avis (
    id          SERIAL PRIMARY KEY,
    commande_id INTEGER NOT NULL UNIQUE REFERENCES commandes(id) ON DELETE CASCADE,
    user_id     INTEGER NOT NULL REFERENCES eihwaz_users(id) ON DELETE RESTRICT,
    note        INTEGER NOT NULL CHECK (note BETWEEN 1 AND 5),
    commentaire TEXT NOT NULL,
    statut      statut_avis NOT NULL DEFAULT 'en_attente',
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE avis_plats (
    id          SERIAL PRIMARY KEY,
    plat_id     INTEGER NOT NULL REFERENCES plats(id) ON DELETE CASCADE,
    user_id     INTEGER REFERENCES eihwaz_users(id) ON DELETE SET NULL,
    note        INTEGER NOT NULL CHECK (note BETWEEN 1 AND 5),
    commentaire TEXT NOT NULL,
    statut      statut_avis_plat NOT NULL DEFAULT 'en_attente',
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- TRAITEUR
-- ============================================================================

CREATE TABLE menus_traiteur (
    id                SERIAL PRIMARY KEY,
    titre             VARCHAR(255) NOT NULL,
    description       TEXT NOT NULL,
    prix_par_personne NUMERIC(10,2) NOT NULL,
    nb_personnes_min  INTEGER NOT NULL,
    theme             theme_menu NOT NULL DEFAULT 'autre',
    regime            regime_menu NOT NULL DEFAULT 'standard',
    conditions        TEXT,
    stock             INTEGER NOT NULL DEFAULT 0,
    actif             BOOLEAN NOT NULL DEFAULT true,
    created_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE menu_traiteur_plats (
    id               SERIAL PRIMARY KEY,
    menu_traiteur_id INTEGER NOT NULL REFERENCES menus_traiteur(id) ON DELETE CASCADE,
    plat_id          INTEGER NOT NULL REFERENCES plats(id) ON DELETE CASCADE,
    UNIQUE (menu_traiteur_id, plat_id)
);

CREATE TABLE devis_traiteur (
    id               SERIAL PRIMARY KEY,
    menu_id          INTEGER REFERENCES menus_traiteur(id) ON DELETE SET NULL,
    nom              VARCHAR(150) NOT NULL,
    email            VARCHAR(255) NOT NULL,
    telephone        VARCHAR(30),
    date_evenement   VARCHAR(10) NOT NULL,
    nb_personnes     INTEGER NOT NULL,
    message          TEXT NOT NULL,
    statut           statut_devis NOT NULL DEFAULT 'en_attente',
    created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- CONTACTS
-- ============================================================================

CREATE TABLE contacts (
    id          SERIAL PRIMARY KEY,
    raison      raisoncontact NOT NULL DEFAULT 'autre',
    titre       VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    email       VARCHAR(255) NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- INDEX
-- ============================================================================

CREATE INDEX idx_commandes_user_id        ON commandes(user_id);
CREATE INDEX idx_commandes_statut         ON commandes(statut);
CREATE INDEX idx_commandes_created_at     ON commandes(created_at);
CREATE INDEX idx_commandes_numero         ON commandes(numero);
CREATE INDEX idx_commande_lignes_cmd      ON commande_lignes(commande_id);
CREATE INDEX idx_commande_statuts_cmd     ON commande_statuts(commande_id);
CREATE INDEX idx_avis_commande_id         ON avis(commande_id);
CREATE INDEX idx_avis_plats_plat_id       ON avis_plats(plat_id);
CREATE INDEX idx_avis_plats_user_id       ON avis_plats(user_id);
CREATE INDEX idx_plat_allergene_plat      ON plat_allergene(plat_id);
CREATE INDEX idx_entree_allergene_entree  ON entree_allergene(entree_id);
CREATE INDEX idx_dessert_allergene_dessert ON dessert_allergene(dessert_id);
CREATE INDEX idx_plat_garnitures_plat     ON plat_garnitures(plat_id);
CREATE INDEX idx_menu_plats_menu          ON menu_plats(menu_id);
CREATE INDEX idx_menu_entrees_menu        ON menu_entrees(menu_id);
CREATE INDEX idx_menu_desserts_menu       ON menu_desserts(menu_id);
CREATE INDEX idx_menu_traiteur_plats_menu ON menu_traiteur_plats(menu_traiteur_id);
CREATE INDEX idx_devis_traiteur_created   ON devis_traiteur(created_at);
CREATE INDEX idx_contacts_created         ON contacts(created_at);
