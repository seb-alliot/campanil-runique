-- ══════════════════════════════════════════════════════════
-- Carte réelle U Campanile — à exécuter sur DB existante
-- ══════════════════════════════════════════════════════════

SET client_encoding = 'UTF8';

-- 1. Étendre les types PostgreSQL
ALTER TYPE typeplat    ADD VALUE IF NOT EXISTS 'specialite';
ALTER TYPE typeplat    ADD VALUE IF NOT EXISTS 'viande';
ALTER TYPE typeplat    ADD VALUE IF NOT EXISTS 'poisson';
ALTER TYPE typeboisson ADD VALUE IF NOT EXISTS 'champagne';
ALTER TYPE typeboisson ADD VALUE IF NOT EXISTS 'biere';
ALTER TYPE typeboisson ADD VALUE IF NOT EXISTS 'aperitif';
ALTER TYPE typeboisson ADD VALUE IF NOT EXISTS 'digestif';

-- Valider les nouveaux types avant utilisation
COMMIT;

-- 1b. Colonnes ajoutées depuis la v1
ALTER TABLE plats ADD COLUMN IF NOT EXISTS label                  VARCHAR(80);
ALTER TABLE plats ADD COLUMN IF NOT EXISTS avec_legumes           BOOLEAN NOT NULL DEFAULT TRUE;

CREATE TYPE IF NOT EXISTS typegarniture AS ENUM ('feculent', 'legumes');

CREATE TABLE IF NOT EXISTS garnitures (
    id              SERIAL PRIMARY KEY,
    libelle         VARCHAR(100) NOT NULL,
    type_garniture  typegarniture NOT NULL,
    disponible      BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE IF NOT EXISTS plat_garnitures (
    id           SERIAL PRIMARY KEY,
    plat_id      INT NOT NULL REFERENCES plats(id) ON DELETE CASCADE,
    garniture_id INT NOT NULL REFERENCES garnitures(id) ON DELETE CASCADE,
    est_defaut   BOOLEAN NOT NULL DEFAULT FALSE
);

ALTER TABLE commande_plats DROP   COLUMN IF EXISTS garniture_id;
ALTER TABLE commande_plats ADD    COLUMN IF NOT EXISTS avec_legumes BOOLEAN DEFAULT FALSE;
ALTER TABLE commande_plats ADD    COLUMN IF NOT EXISTS sans_sel     BOOLEAN DEFAULT FALSE;

CREATE TABLE IF NOT EXISTS commande_plat_garnitures (
    id               SERIAL PRIMARY KEY,
    commande_plat_id INT NOT NULL REFERENCES commande_plats(id) ON DELETE CASCADE,
    garniture_id     INT NOT NULL REFERENCES garnitures(id)     ON DELETE RESTRICT
);

CREATE TABLE IF NOT EXISTS menu_resto_plat (
    id      SERIAL PRIMARY KEY,
    menu_id INT NOT NULL REFERENCES menu_resto(id) ON DELETE CASCADE,
    plat_id INT NOT NULL REFERENCES plats(id)      ON DELETE CASCADE,
    cours   VARCHAR(10) NOT NULL
);

CREATE TABLE IF NOT EXISTS commande_menu_choix (
    id               SERIAL PRIMARY KEY,
    commande_plat_id INT  NOT NULL REFERENCES commande_plats(id) ON DELETE CASCADE,
    cours            VARCHAR(10) NOT NULL,
    plat_id          INT  NOT NULL REFERENCES plats(id)          ON DELETE RESTRICT,
    cuisson          VARCHAR(20),
    avec_legumes     BOOLEAN NOT NULL DEFAULT FALSE,
    sans_sel         BOOLEAN NOT NULL DEFAULT FALSE,
    note             TEXT
);

CREATE TABLE IF NOT EXISTS commande_menu_choix_garnitures (
    id                     SERIAL PRIMARY KEY,
    commande_menu_choix_id INT NOT NULL REFERENCES commande_menu_choix(id) ON DELETE CASCADE,
    garniture_id           INT NOT NULL REFERENCES garnitures(id)           ON DELETE RESTRICT
);

-- 0. Créer la table menu_resto si elle n'existe pas
CREATE TABLE IF NOT EXISTS menu_resto (
    id          SERIAL PRIMARY KEY,
    titre       VARCHAR(255) NOT NULL,
    prix        NUMERIC(10, 2) NOT NULL,
    description TEXT,
    disponible  BOOLEAN NOT NULL DEFAULT TRUE
);

-- 2. Vider les anciennes données
DELETE FROM plat_allergene;
DELETE FROM menu_plat;
DELETE FROM commande_menu_choix_garnitures;
DELETE FROM commande_menu_choix;
DELETE FROM commande_plat_garnitures;
DELETE FROM commande_plats;
DELETE FROM plats;
DELETE FROM boissons;

-- 3. Plats réels (titre, label, type, prix, description, image, disponible, est_viande, avec_legumes)
INSERT INTO plats (id, titre, label, type_plat, prix, description, image, disponible, est_viande, avec_legumes) VALUES
    -- Entrées
    (1,  'Soupe traditionnelle mijotée à l''os de jambon',                              'Soupe jambon',      'entree',     12.00, NULL, NULL, true, false, true),
    (2,  'Notre terrine de sanglier aux baies de myrte',                                'Terrine sanglier',  'entree',     14.00, NULL, NULL, true, false, true),
    (3,  'Planche Charcuterie Corse',                                                   'Charcuterie',       'entree',     17.00, NULL, NULL, true, false, true),
    (4,  'Houmous, légumes crus, huile de sésame',                                      'Houmous',           'entree',     16.00, NULL, NULL, true, false, true),
    (5,  'Mi-cuit foie gras de canard Origine France et ses toasts',                    'Foie gras',         'entree',     22.00, '100 g environ', NULL, true, false, true),
    (6,  'Le pain du bandit',                                                           'Pain bandit',       'entree',     15.00, 'Tartine fromage et jambon corse, figues, pesto, roquette', NULL, true, false, true),
    (7,  'Salade de poulpe tiède en persillade, pommes de terre vapeur',                'Poulpe',            'entree',     14.00, NULL, NULL, true, false, true),
    (8,  'Bowl Végé solo',                                                              'Bowl Végé S',       'entree',      8.50, 'Riz, tomate, carotte, concombre, avocat, ananas, houmous', NULL, true, false, true),
    (9,  'Bowl Végé à partager',                                                        'Bowl Végé L',       'entree',     17.00, 'Riz, tomate, carotte, concombre, avocat, ananas, houmous', NULL, true, false, true),
    (10, 'Bowl Saumon fumé solo',                                                       'Bowl Saumon S',     'entree',      9.50, 'Riz, avocat, tomate, concombre, carotte, houmous', NULL, true, false, true),
    (11, 'Bowl Saumon fumé à partager',                                                 'Bowl Saumon L',     'entree',     19.50, 'Riz, avocat, tomate, concombre, carotte, houmous', NULL, true, false, true),
    (12, 'U Spuntinu',                                                                  'U Spuntinu',        'entree',     24.00, 'Planche de charcuterie avec fromages, confiture et terrine de sanglier', NULL, true, false, true),

    -- Spécialités de chez nous
    (13, 'Cannelloni fraîches broccio-blettes, sauce tomate',                           'Cannelloni brocciu', 'specialite', 17.00, NULL, NULL, true, false, true),
    (14, 'Nos saucisses de sanglier maison à l''usu Figatellu, purée au lait',          'Saucisses sanglier', 'specialite', 21.00, NULL, NULL, true, false, true),
    (15, 'Truite fraîche poêlée à la Népita, pommes de terre, légumes croquants',       'Truite Népita',      'specialite', 16.50, NULL, NULL, true, false, true),

    -- Viandes
    (16, 'Gnocchi sauce morilles, foie gras frais poêlé, noisettes torréfiées',         'Gnocchi morilles',   'viande', 32.00, NULL, NULL, true, false, true),
    (17, 'Émincé de volaille panée à la noisette, légumes croquants, pommes de terre',  'Volaille noisette',  'viande', 18.50, NULL, NULL, true, false, true),
    (18, '« Corsican pulled porc thaï style », purée au lait, épinards',               'Pulled porc',        'viande', 22.00, NULL, NULL, true, false, true),
    (19, 'Filet de bœuf charolais, sauce morilles, pommes de terre',                    'Filet bœuf',         'viande', 32.00, 'Option Rossini +7€', NULL, true, true, true),
    (20, 'Gnocchi et veau Corse façon bolognaise, tomme de brebis, châtaignes',         'Gnocchi veau',       'viande', 24.00, NULL, NULL, true, false, true),

    -- Poissons
    (21, 'Filet de daurade cuit sur peau, émulsion citronnée, petits légumes',          'Daurade',            'poisson', 27.00, NULL, NULL, true, false, true),
    (22, 'Penne aux gambas du chef, sauce crustacés',                                   'Penne gambas',       'poisson', 34.00, NULL, NULL, true, false, true),
    (23, 'Seiche poêlée aux noix de cajou, sauce vierge, pomme de terre, salade',       'Seiche cajou',       'poisson', 26.00, NULL, NULL, true, false, true),
    (24, 'Healthy Burger Saumon fumé, avocat, crème cheese, pommes de terre, salade',   'Burger saumon',      'poisson', 21.00, NULL, NULL, true, false, true),

    -- Desserts
    (25, 'Fromages Cortenais, confiture de figue',                                      'Fromages',           'dessert',  8.50, NULL, NULL, true, false, true),
    (26, 'Fiadone — Douceur Brocciu-citron',                                            'Fiadone',            'dessert',  7.00, NULL, NULL, true, false, true),
    (27, 'Myrte glacée',                                                                'Myrte glacée',       'dessert',  6.50, 'Sans lactose', NULL, true, false, true);

-- 4. Allergènes des plats
INSERT INTO plat_allergene (id, plat_id, allergene_id) VALUES
    -- Soupe jambon : gluten, céleri
    (1,  1, 1), (2,  1, 9),
    -- Terrine sanglier : sulfites
    (3,  2, 12),
    -- Charcuterie : sulfites
    (4,  3, 12),
    -- Houmous : sésame
    (5,  4, 11),
    -- Foie gras : sulfites, gluten (toasts)
    (6,  5, 12), (7,  5, 1),
    -- Pain du bandit : gluten, lait
    (8,  6, 1), (9,  6, 7),
    -- Poulpe : mollusques
    (10, 7, 14),
    -- Bowl Végé solo : sésame
    (11, 8, 11),
    -- Bowl Végé partager : sésame
    (12, 9, 11),
    -- Bowl Saumon solo : poissons
    (13, 10, 4),
    -- Bowl Saumon partager : poissons
    (14, 11, 4),
    -- U Spuntinu : sulfites, lait
    (15, 12, 12), (16, 12, 7),
    -- Cannelloni brocciu : gluten, lait, œufs
    (17, 13, 1), (18, 13, 7), (19, 13, 3),
    -- Saucisses sanglier : sulfites
    (20, 14, 12),
    -- Truite : poissons
    (21, 15, 4),
    -- Gnocchi morilles : gluten, lait, fruits à coque (noisettes)
    (22, 16, 1), (23, 16, 7), (24, 16, 8),
    -- Émincé volaille : fruits à coque (noisette)
    (25, 17, 8),
    -- Pulled porc : lait (purée)
    (26, 18, 7),
    -- Gnocchi veau : gluten, lait (tomme)
    (27, 20, 1), (28, 20, 7),
    -- Daurade : poissons
    (29, 21, 4),
    -- Penne gambas : gluten, crustacés
    (30, 22, 1), (31, 22, 2),
    -- Seiche cajou : mollusques, fruits à coque
    (32, 23, 14), (33, 23, 8),
    -- Healthy Burger saumon : poissons, lait (crème cheese)
    (34, 24, 4), (35, 24, 7),
    -- Fromages cortenais : lait
    (36, 25, 7),
    -- Fiadone : lait, œufs
    (37, 26, 7), (38, 26, 3);

-- 5. Menus traiteur — composition avec les vrais plats
INSERT INTO menu_plat (id, menu_id, plat_id) VALUES
    -- Menu 1 : Prestige Mariage
    (1,  1,  3), (2,  1,  5), (3,  1, 12),  -- entrées
    (4,  1, 19), (5,  1, 21), (6,  1, 13),  -- plats
    (7,  1, 25), (8,  1, 26),               -- desserts
    -- Menu 2 : Découverte Corse
    (9,  2,  1), (10, 2,  2),               -- entrées
    (11, 2, 14), (12, 2, 15),               -- plats
    (13, 2, 26), (14, 2, 27),               -- desserts
    -- Menu 3 : Végétarien du Maquis
    (15, 3,  4), (16, 3,  8),               -- entrées
    (17, 3, 13),                             -- plat
    (18, 3, 26), (19, 3, 27),               -- desserts
    -- Menu 4 : Baptême & Communion
    (20, 4,  3), (21, 4,  6),               -- entrées
    (22, 4, 19), (23, 4, 13),               -- plats
    (24, 4, 25), (25, 4, 26),               -- desserts
    -- Menu 5 : Sans Gluten
    (26, 5,  4), (27, 5,  7),               -- entrées sans gluten
    (28, 5, 21), (29, 5, 18),               -- plats sans gluten
    (30, 5, 27);                             -- dessert sans lactose

-- 6. Boissons réelles
INSERT INTO boissons (id, titre, type_boisson, prix, description, image, disponible) VALUES
    -- Vins rouges 75cl
    (1,  'Alzipratu « Pumonte » — Calvi 75cl',                     'vin_rouge', 42.00, NULL, NULL, true),
    (2,  'Pero longo « Le lion de Roccapina » — Sartène 75cl',      'vin_rouge', 46.00, NULL, NULL, true),
    (3,  'Giacometti « Batolaccio » — Patrimonio 75cl',             'vin_rouge', 31.00, NULL, NULL, true),
    (4,  'Ornasca — Ajaccio 75cl',                                  'vin_rouge', 29.00, NULL, NULL, true),
    (5,  'Guelfucci « Sciacarellu » — Corte 75cl',                  'vin_rouge', 34.00, NULL, NULL, true),
    (6,  'Devichi « Mlle D » — Patrimonio 75cl',                    'vin_rouge', 27.00, NULL, NULL, true),
    (7,  'San Armettu « Myrtus » — AOP Sartène 75cl',               'vin_rouge', 49.00, NULL, NULL, true),
    (8,  'Landry « Cuvée Léa » — AOP Calvi 75cl',                   'vin_rouge', 54.00, NULL, NULL, true),
    (9,  'Peretti della Rocca « Jules » — Figari 75cl',              'vin_rouge', 44.00, NULL, NULL, true),
    -- Vins rouges 50cl
    (10, 'Pero longo — Sartène 50cl',                               'vin_rouge', 21.50, NULL, NULL, true),
    (11, 'Ornasca — Ajaccio 50cl',                                  'vin_rouge', 21.00, NULL, NULL, true),
    -- Vin rouge au verre
    (12, 'Vin AOP Rouge au verre',                                  'vin_rouge',  6.50, 'Demandez la suggestion du jour', NULL, true),
    (13, 'Vin IGP La cave d''Aléria — Rouge',                       'vin_rouge',  3.50, 'Verre 3,5€ · 25cl 5€ · 50cl 8€', NULL, true),

    -- Vins rosés 75cl
    (14, 'Guelfucci « Laurina » — Corte 75cl',                      'vin_rose', 24.00, NULL, NULL, true),
    (15, 'Pero Longo « Harmonie » — Sartène 75cl',                  'vin_rose', 27.00, NULL, NULL, true),
    (16, 'Lucciardi « Signora Catalina » — AOP de Corse 75cl',      'vin_rose', 29.00, NULL, NULL, true),
    -- Rosé 50cl
    (17, 'Pero Longo — Sartène 50cl',                               'vin_rose', 21.00, NULL, NULL, true),
    -- Rosé au verre
    (18, 'Vin AOP Rosé au verre',                                   'vin_rose',  6.50, 'Demandez la suggestion du jour', NULL, true),
    (19, 'Vin IGP La cave d''Aléria — Rosé',                        'vin_rose',  3.50, 'Verre 3,5€ · 25cl 5€ · 50cl 8€', NULL, true),

    -- Vins blancs 75cl
    (20, 'Castellu di Baricci — Sartène 75cl',                      'vin_blanc', 49.00, NULL, NULL, true),
    (21, 'Marquiliani « Blanc de noir » — AOP de Corse 75cl',       'vin_blanc', 39.00, NULL, NULL, true),
    (22, 'Bertolozzi « Inghjò » — Oletta 75cl',                     'vin_blanc', 27.00, NULL, NULL, true),
    (23, 'Suale « Simbiosi » — Patrimonio 75cl',                    'vin_blanc', 52.00, NULL, NULL, true),
    (24, 'Domaine Orsucci — AOP de Corse 75cl',                     'vin_blanc', 28.00, NULL, NULL, true),
    -- Blanc au verre
    (25, 'Vin AOP Blanc au verre',                                  'vin_blanc',  6.50, 'Demandez la suggestion du jour', NULL, true),
    (26, 'Vin IGP La cave d''Aléria — Blanc',                       'vin_blanc',  3.50, 'Verre 3,5€ · 25cl 5€ · 50cl 8€', NULL, true),

    -- Champagne
    (27, 'Baron de Rothschild Brut — la coupe',                     'champagne', 12.00, NULL, NULL, true),
    (28, 'Baron de Rothschild Brut — bouteille 75cl',               'champagne', 90.00, NULL, NULL, true),

    -- Bières
    (29, 'PAOLINA Pression 25cl',                                   'biere',  4.50, NULL, NULL, true),
    (30, 'PAOLINA Pression 50cl',                                   'biere',  8.00, NULL, NULL, true),
    (31, 'Panaché 25cl',                                            'biere',  4.00, NULL, NULL, true),
    (32, 'PAOLINA IPA 33cl',                                        'biere',  5.50, NULL, NULL, true),
    (33, 'Pietra 33cl',                                             'biere',  5.00, NULL, NULL, true),
    (34, 'Pietra Limoncello 33cl',                                  'biere',  5.50, NULL, NULL, true),
    (35, 'Kiara Sunset — Despé Corse',                              'biere',  6.50, NULL, NULL, true),
    (36, 'Paolina sans Alcool',                                     'biere',  6.50, NULL, NULL, true),
    (37, 'Bud 25cl',                                                'biere',  4.00, NULL, NULL, true),

    -- Apéritifs
    (38, 'Muscat Mlle Devichi / Cap Corse rouge ou blanc',          'aperitif',  6.00, NULL, NULL, true),
    (39, 'Muscat Pétillant / Prosseco',                             'aperitif',  6.00, NULL, NULL, true),
    (40, 'Martini Rouge ou Blanc',                                  'aperitif',  5.50, NULL, NULL, true),
    (41, 'Pastis',                                                  'aperitif',  2.50, NULL, NULL, true),
    (42, 'Capo Spritz, Limoncello ou Myrtle Spritz, Apérol Spritz','aperitif', 10.00, NULL, NULL, true),

    -- Softs
    (43, 'Sodas (Coca, Orangina, Ice Tea, Schweppes...)',           'soft',  3.20, NULL, NULL, true),
    (44, 'Limonade corse / Diabolo',                                'soft',  3.50, NULL, NULL, true),
    (45, 'Jus Pago 25cl (Pomme, Pêche, Tomate, Ananas)',           'soft',  3.20, NULL, NULL, true),
    (46, 'Sirop à l''eau (Grenadine, fraise, citron, menthe...)',   'soft',  1.50, NULL, NULL, true),

    -- Eaux
    (47, 'Eau plate Zilia 33cl',                                    'eau_bouteille',  2.50, NULL, NULL, true),
    (48, 'Eau plate Zilia 50cl',                                    'eau_bouteille',  3.80, NULL, NULL, true),
    (49, 'Eau plate Zilia 1L',                                      'eau_bouteille',  4.50, NULL, NULL, true),
    (50, 'Eau gazeuse Orezza 33cl',                                 'eau_bouteille',  3.20, NULL, NULL, true),
    (51, 'Eau gazeuse Orezza 50cl',                                 'eau_bouteille',  4.10, NULL, NULL, true),
    (52, 'Eau gazeuse Orezza 1L',                                   'eau_bouteille',  5.20, NULL, NULL, true),

    -- Digestifs & chauds
    (53, 'Myrte / Limoncello / U Getu',                             'digestif',  6.00, NULL, NULL, true),
    (54, 'Thé / Infusion Kusmi Tea',                                'digestif',  3.00, NULL, NULL, true),
    (55, 'Expresso / Décaféiné',                                    'digestif',  1.90, NULL, NULL, true);

-- 7. Menus restaurant
DELETE FROM menu_resto_plat;
DELETE FROM menu_resto;
INSERT INTO menu_resto (id, titre, prix, description, disponible) VALUES
    (1, 'Menu à la découverte de Corte', 27.00, NULL, true),
    (2, 'Menu Enfant', 12.00, NULL, true);

-- 7b. Composition des menus restaurant (entrée / plat / dessert)
INSERT INTO menu_resto_plat (menu_id, plat_id, cours) VALUES
    -- Menu 1 : À la découverte de Corte
    (1,  1, 'entree'),   -- Soupe jambon
    (1,  2, 'entree'),   -- Terrine sanglier
    (1,  4, 'entree'),   -- Houmous
    (1, 14, 'plat'),     -- Saucisses sanglier
    (1, 15, 'plat'),     -- Truite Népita
    (1, 13, 'plat'),     -- Cannelloni brocciu
    (1, 25, 'dessert'),  -- Fromages cortenais
    (1, 26, 'dessert'),  -- Fiadone
    (1, 27, 'dessert'),  -- Myrte glacée
    -- Menu 2 : Enfant
    (2, 13, 'plat'),     -- Cannelloni brocciu
    (2, 20, 'plat');     -- Gnocchi veau bolognaise

-- 8. Resynchronisation des séquences PostgreSQL
DO $$
DECLARE t text;
BEGIN
    FOREACH t IN ARRAY ARRAY['plats','plat_allergene','menu_plat','boissons','menu_resto'] LOOP
        EXECUTE format(
            'SELECT setval(pg_get_serial_sequence(%L, ''id''), COALESCE(MAX(id), 1)) FROM %I',
            t, t
        );
    END LOOP;
END $$;
