-- ══════════════════════════════════════════════════════════════
-- Seed U Campanile — données réelles complètes
-- Schéma géré par migrations
-- ══════════════════════════════════════════════════════════════

SET client_encoding = 'UTF8';

-- ── 0. Nettoyage données catalogue (pas les commandes/avis) ──
DELETE FROM menu_resto_plat;
DELETE FROM menu_resto;
DELETE FROM menu_plat;
DELETE FROM plat_garnitures;
DELETE FROM plat_allergene;
DELETE FROM menu_enfant_allergenes;
DELETE FROM menu_enfants;
DELETE FROM formule_jours;
DELETE FROM supplements;
DELETE FROM garnitures;
DELETE FROM boissons;
DELETE FROM plats;
DELETE FROM menus;
DELETE FROM allergenes;

-- ── 1. Allergènes (14 majeurs UE) ───────────────────────────

INSERT INTO allergenes (id, libelle) VALUES
    (1,  'Gluten'),
    (2,  'Crustacés'),
    (3,  'Œufs'),
    (4,  'Poissons'),
    (5,  'Arachides'),
    (6,  'Soja'),
    (7,  'Lait'),
    (8,  'Fruits à coque'),
    (9,  'Céleri'),
    (10, 'Moutarde'),
    (11, 'Graines de sésame'),
    (12, 'Sulfites'),
    (13, 'Lupin'),
    (14, 'Mollusques');

-- ── 2. Plats réels ──────────────────────────────────────────

INSERT INTO plats (id, titre, label, type_plat, prix, description, image, disponible, est_viande, avec_legumes) VALUES
    -- Entrées
    (1,  'Soupe traditionnelle mijotée à l''os de jambon',
         'Soupe jambon',       'entree',     12.00, NULL,                                                                                  NULL, true, false, true),
    (2,  'Notre terrine de sanglier aux baies de myrte',
         'Terrine sanglier',   'entree',     14.00, NULL,                                                                                  NULL, true, false, true),
    (3,  'Planche Charcuterie Corse',
         'Charcuterie',        'entree',     17.00, NULL,                                                                                  NULL, true, false, true),
    (4,  'Houmous, légumes crus, huile de sésame',
         'Houmous',            'entree',     16.00, NULL,                                                                                  NULL, true, false, true),
    (5,  'Mi-cuit foie gras de canard Origine France et ses toasts',
         'Foie gras',          'entree',     22.00, '100 g environ',                                                                       NULL, true, false, true),
    (6,  'Le pain du bandit',
         'Pain bandit',        'entree',     15.00, 'Tartine fromage et jambon corse, figues, pesto, roquette',                            NULL, true, false, true),
    (7,  'Salade de poulpe tiède en persillade, pommes de terre vapeur',
         'Poulpe',             'entree',     14.00, NULL,                                                                                  NULL, true, false, true),
    (8,  'Bowl Végé solo',
         'Bowl Végé S',        'entree',      8.50, 'Riz, tomate, carotte, concombre, avocat, ananas, houmous',                           NULL, true, false, true),
    (9,  'Bowl Végé à partager',
         'Bowl Végé L',        'entree',     17.00, 'Riz, tomate, carotte, concombre, avocat, ananas, houmous',                           NULL, true, false, true),
    (10, 'Bowl Saumon fumé solo',
         'Bowl Saumon S',      'entree',      9.50, 'Riz, avocat, tomate, concombre, carotte, houmous',                                   NULL, true, false, true),
    (11, 'Bowl Saumon fumé à partager',
         'Bowl Saumon L',      'entree',     19.50, 'Riz, avocat, tomate, concombre, carotte, houmous',                                   NULL, true, false, true),
    (12, 'U Spuntinu',
         'U Spuntinu',         'entree',     24.00, 'Planche de charcuterie avec fromages, confiture et terrine de sanglier',             NULL, true, false, true),

    -- Spécialités de chez nous
    (13, 'Cannelloni fraîches broccio-blettes, sauce tomate',
         'Cannelloni brocciu', 'specialite', 17.00, NULL,                                                                                  NULL, true, false, true),
    (14, 'Nos saucisses de sanglier maison à l''usu Figatellu, purée au lait',
         'Saucisses sanglier', 'specialite', 21.00, NULL,                                                                                  NULL, true, false, true),
    (15, 'Truite fraîche poêlée à la Népita, pommes de terre, légumes croquants',
         'Truite Népita',      'specialite', 16.50, NULL,                                                                                  NULL, true, false, true),

    -- Viandes
    (16, 'Gnocchi sauce morilles, foie gras frais poêlé, noisettes torréfiées',
         'Gnocchi morilles',   'viande',     32.00, NULL,                                                                                  NULL, true, false, true),
    (17, 'Émincé de volaille panée à la noisette, légumes croquants, pommes de terre',
         'Volaille noisette',  'viande',     18.50, NULL,                                                                                  NULL, true, false, true),
    (18, '« Corsican pulled porc thaï style », purée au lait, épinards',
         'Pulled porc',        'viande',     22.00, NULL,                                                                                  NULL, true, false, true),
    (19, 'Filet de bœuf charolais, sauce morilles, pommes de terre',
         'Filet bœuf',         'viande',     32.00, 'Option Rossini +7€',                                                                  NULL, true, true,  true),
    (20, 'Gnocchi et veau Corse façon bolognaise, tomme de brebis, châtaignes',
         'Gnocchi veau',       'viande',     24.00, NULL,                                                                                  NULL, true, false, true),

    -- Poissons
    (21, 'Filet de daurade cuit sur peau, émulsion citronnée, petits légumes',
         'Daurade',            'poisson',    27.00, NULL,                                                                                  NULL, true, false, true),
    (22, 'Penne aux gambas du chef, sauce crustacés',
         'Penne gambas',       'poisson',    34.00, NULL,                                                                                  NULL, true, false, true),
    (23, 'Seiche poêlée aux noix de cajou, sauce vierge, pomme de terre, salade',
         'Seiche cajou',       'poisson',    26.00, NULL,                                                                                  NULL, true, false, true),
    (24, 'Healthy Burger Saumon fumé, avocat, crème cheese, pommes de terre, salade',
         'Burger saumon',      'poisson',    21.00, NULL,                                                                                  NULL, true, false, true),

    -- Desserts
    (25, 'Fromages Cortenais, confiture de figue',
         'Fromages',           'dessert',    8.50, NULL,                                                                                   NULL, true, false, true),
    (26, 'Fiadone — Douceur Brocciu-citron',
         'Fiadone',            'dessert',    7.00, NULL,                                                                                   NULL, true, false, true),
    (27, 'Myrte glacée',
         'Myrte glacée',       'dessert',    6.50, 'Sans lactose',                                                                        NULL, true, false, true);

-- ── 3. Allergènes des plats ─────────────────────────────────

INSERT INTO plat_allergene (id, plat_id, allergene_id) VALUES
    -- Soupe jambon : gluten, céleri
    (1,  1, 1),  (2,  1, 9),
    -- Terrine sanglier : sulfites
    (3,  2, 12),
    -- Charcuterie : sulfites
    (4,  3, 12),
    -- Houmous : sésame
    (5,  4, 11),
    -- Foie gras : sulfites, gluten (toasts)
    (6,  5, 12), (7,  5, 1),
    -- Pain du bandit : gluten, lait
    (8,  6, 1),  (9,  6, 7),
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
    (17, 13, 1),  (18, 13, 7),  (19, 13, 3),
    -- Saucisses sanglier : sulfites
    (20, 14, 12),
    -- Truite Népita : poissons
    (21, 15, 4),
    -- Gnocchi morilles : gluten, lait, fruits à coque (noisettes)
    (22, 16, 1),  (23, 16, 7),  (24, 16, 8),
    -- Volaille noisette : fruits à coque
    (25, 17, 8),
    -- Pulled porc : lait (purée)
    (26, 18, 7),
    -- Gnocchi veau : gluten, lait (tomme)
    (27, 20, 1),  (28, 20, 7),
    -- Daurade : poissons
    (29, 21, 4),
    -- Penne gambas : gluten, crustacés
    (30, 22, 1),  (31, 22, 2),
    -- Seiche cajou : mollusques, fruits à coque
    (32, 23, 14), (33, 23, 8),
    -- Burger saumon : poissons, lait (crème cheese)
    (34, 24, 4),  (35, 24, 7),
    -- Fromages cortenais : lait
    (36, 25, 7),
    -- Fiadone : lait, œufs
    (37, 26, 7),  (38, 26, 3);

-- ── 4. Garnitures ───────────────────────────────────────────

INSERT INTO garnitures (id, libelle, type_garniture, disponible) VALUES
    -- Féculents
    (1, 'Purée au lait',          'feculent', true),
    (2, 'Pommes de terre vapeur', 'feculent', true),
    (3, 'Frites maison',          'feculent', true),
    (4, 'Riz',                    'feculent', true),
    (5, 'Gnocchi maison',         'feculent', true),
    -- Légumes
    (6, 'Légumes croquants',      'legumes',  true),
    (7, 'Épinards',               'legumes',  true),
    (8, 'Salade verte',           'legumes',  true);

INSERT INTO plat_garnitures (id, plat_id, garniture_id, est_defaut) VALUES
    -- Filet bœuf (19) : pommes de terre (défaut), purée, frites
    (1, 19, 2, true),
    (2, 19, 1, false),
    (3, 19, 3, false),
    -- Daurade (21) : légumes croquants (défaut), riz
    (4, 21, 6, true),
    (5, 21, 4, false),
    -- Seiche cajou (23) : pommes de terre (défaut), riz
    (6, 23, 2, true),
    (7, 23, 4, false),
    -- Burger saumon (24) : frites (défaut)
    (8, 24, 3, true);

-- ── 5. Suppléments ──────────────────────────────────────────
-- garniture_id nullable : lié à une garniture existante OU titre libre

INSERT INTO supplements (id, garniture_id, titre, libelle, prix, disponible) VALUES
    (1, NULL, 'Option Rossini',  'Foie gras frais poêlé',       7.00, true),
    (2, 6,    NULL,              'Légumes grillés en supplément', 3.50, true),
    (3, 1,    NULL,              'Purée maison en supplément',    3.50, true),
    (4, 3,    NULL,              'Frites maison en supplément',   3.50, true),
    (5, NULL, 'Pain artisanal',  NULL,                            2.00, true),
    (6, NULL, 'Sauce du chef',   NULL,                            2.50, true);

-- ── 6. Boissons réelles ─────────────────────────────────────

INSERT INTO boissons (id, titre, type_boisson, prix, description, image, disponible) VALUES
    -- Vins rouges 75cl
    (1,  'Alzipratu « Pumonte » — Calvi 75cl',                   'vin_rouge',     42.00, NULL, NULL, true),
    (2,  'Pero longo « Le lion de Roccapina » — Sartène 75cl',   'vin_rouge',     46.00, NULL, NULL, true),
    (3,  'Giacometti « Batolaccio » — Patrimonio 75cl',          'vin_rouge',     31.00, NULL, NULL, true),
    (4,  'Ornasca — Ajaccio 75cl',                               'vin_rouge',     29.00, NULL, NULL, true),
    (5,  'Guelfucci « Sciacarellu » — Corte 75cl',               'vin_rouge',     34.00, NULL, NULL, true),
    (6,  'Devichi « Mlle D » — Patrimonio 75cl',                 'vin_rouge',     27.00, NULL, NULL, true),
    (7,  'San Armettu « Myrtus » — AOP Sartène 75cl',            'vin_rouge',     49.00, NULL, NULL, true),
    (8,  'Landry « Cuvée Léa » — AOP Calvi 75cl',                'vin_rouge',     54.00, NULL, NULL, true),
    (9,  'Peretti della Rocca « Jules » — Figari 75cl',          'vin_rouge',     44.00, NULL, NULL, true),
    -- Vins rouges 50cl et verre
    (10, 'Pero longo — Sartène 50cl',                            'vin_rouge',     21.50, NULL, NULL, true),
    (11, 'Ornasca — Ajaccio 50cl',                               'vin_rouge',     21.00, NULL, NULL, true),
    (12, 'Vin AOP Rouge au verre',                               'vin_rouge',      6.50, 'Demandez la suggestion du jour', NULL, true),
    (13, 'Vin IGP La cave d''Aléria — Rouge',                    'vin_rouge',      3.50, 'Verre 3,5€ · 25cl 5€ · 50cl 8€', NULL, true),
    -- Vins rosés
    (14, 'Guelfucci « Laurina » — Corte 75cl',                   'vin_rose',      24.00, NULL, NULL, true),
    (15, 'Pero Longo « Harmonie » — Sartène 75cl',               'vin_rose',      27.00, NULL, NULL, true),
    (16, 'Lucciardi « Signora Catalina » — AOP de Corse 75cl',   'vin_rose',      29.00, NULL, NULL, true),
    (17, 'Pero Longo — Sartène 50cl',                            'vin_rose',      21.00, NULL, NULL, true),
    (18, 'Vin AOP Rosé au verre',                                'vin_rose',       6.50, 'Demandez la suggestion du jour', NULL, true),
    (19, 'Vin IGP La cave d''Aléria — Rosé',                     'vin_rose',       3.50, 'Verre 3,5€ · 25cl 5€ · 50cl 8€', NULL, true),
    -- Vins blancs
    (20, 'Castellu di Baricci — Sartène 75cl',                   'vin_blanc',     49.00, NULL, NULL, true),
    (21, 'Marquiliani « Blanc de noir » — AOP de Corse 75cl',    'vin_blanc',     39.00, NULL, NULL, true),
    (22, 'Bertolozzi « Inghjò » — Oletta 75cl',                  'vin_blanc',     27.00, NULL, NULL, true),
    (23, 'Suale « Simbiosi » — Patrimonio 75cl',                 'vin_blanc',     52.00, NULL, NULL, true),
    (24, 'Domaine Orsucci — AOP de Corse 75cl',                  'vin_blanc',     28.00, NULL, NULL, true),
    (25, 'Vin AOP Blanc au verre',                               'vin_blanc',      6.50, 'Demandez la suggestion du jour', NULL, true),
    (26, 'Vin IGP La cave d''Aléria — Blanc',                    'vin_blanc',      3.50, 'Verre 3,5€ · 25cl 5€ · 50cl 8€', NULL, true),
    -- Champagne
    (27, 'Baron de Rothschild Brut — la coupe',                  'champagne',     12.00, NULL, NULL, true),
    (28, 'Baron de Rothschild Brut — bouteille 75cl',            'champagne',     90.00, NULL, NULL, true),
    -- Bières
    (29, 'PAOLINA Pression 25cl',                                'biere',          4.50, NULL, NULL, true),
    (30, 'PAOLINA Pression 50cl',                                'biere',          8.00, NULL, NULL, true),
    (31, 'Panaché 25cl',                                         'biere',          4.00, NULL, NULL, true),
    (32, 'PAOLINA IPA 33cl',                                     'biere',          5.50, NULL, NULL, true),
    (33, 'Pietra 33cl',                                          'biere',          5.00, NULL, NULL, true),
    (34, 'Pietra Limoncello 33cl',                               'biere',          5.50, NULL, NULL, true),
    (35, 'Kiara Sunset — Despé Corse',                           'biere',          6.50, NULL, NULL, true),
    (36, 'Paolina sans Alcool',                                  'biere',          6.50, NULL, NULL, true),
    (37, 'Bud 25cl',                                             'biere',          4.00, NULL, NULL, true),
    -- Apéritifs
    (38, 'Muscat Mlle Devichi / Cap Corse rouge ou blanc',       'aperitif',       6.00, NULL, NULL, true),
    (39, 'Muscat Pétillant / Prosecco',                          'aperitif',       6.00, NULL, NULL, true),
    (40, 'Martini Rouge ou Blanc',                               'aperitif',       5.50, NULL, NULL, true),
    (41, 'Pastis',                                               'aperitif',       2.50, NULL, NULL, true),
    (42, 'Capo Spritz, Limoncello, Myrtle Spritz ou Apérol',    'aperitif',      10.00, NULL, NULL, true),
    -- Softs
    (43, 'Sodas (Coca, Orangina, Ice Tea, Schweppes...)',        'soft',           3.20, NULL, NULL, true),
    (44, 'Limonade corse / Diabolo',                             'soft',           3.50, NULL, NULL, true),
    (45, 'Jus Pago 25cl (Pomme, Pêche, Tomate, Ananas)',        'soft',           3.20, NULL, NULL, true),
    (46, 'Sirop à l''eau (Grenadine, fraise, citron, menthe...)', 'soft',          1.50, NULL, NULL, true),
    -- Eaux
    (47, 'Eau plate Zilia 33cl',                                 'eau_bouteille',  2.50, NULL, NULL, true),
    (48, 'Eau plate Zilia 50cl',                                 'eau_bouteille',  3.80, NULL, NULL, true),
    (49, 'Eau plate Zilia 1L',                                   'eau_bouteille',  4.50, NULL, NULL, true),
    (50, 'Eau gazeuse Orezza 33cl',                              'eau_bouteille',  3.20, NULL, NULL, true),
    (51, 'Eau gazeuse Orezza 50cl',                              'eau_bouteille',  4.10, NULL, NULL, true),
    (52, 'Eau gazeuse Orezza 1L',                                'eau_bouteille',  5.20, NULL, NULL, true),
    -- Digestifs & chauds
    (53, 'Myrte / Limoncello / U Getu',                         'digestif',       6.00, NULL, NULL, true),
    (54, 'Thé / Infusion Kusmi Tea',                             'digestif',       3.00, NULL, NULL, true),
    (55, 'Expresso / Décaféiné',                                 'digestif',       1.90, NULL, NULL, true);

-- ── 7. Formule du jour (exemple) ────────────────────────────

INSERT INTO formule_jours (id, titre, description, prix, dessert) VALUES
    (1, 'Magret de canard aux cèpes', 'Servi avec pommes de terre sarladaises', 18.00, 'Fiadone du jour');

-- ── 8. Menu enfant ──────────────────────────────────────────

INSERT INTO menu_enfants (id, titre, description, plat, dessert, prix, actif) VALUES
    (1, 'Menu Pinocchio', 'Le menu des petits gourmands', 'Spaghetti bolognaise maison', 'Glace 2 boules au choix', 12.00, true);

-- ── 9. Menus traiteur ───────────────────────────────────────

INSERT INTO menus (id, titre, description, prix_par_personne, nb_personnes_min, theme, regime, conditions, stock, actif, created_at) VALUES
    (1, 'Menu Prestige Mariage',
        'Notre menu d''exception pour célébrer votre union en Corse. Un parcours gastronomique autour des plus belles saveurs de l''île.',
        65.00, 30, 'mariage', 'standard',
        'Acompte de 30 % à la commande. Annulation possible jusqu''à 30 jours avant l''événement. Prestation comprise : dressage, service à table, débarrassage.',
        8, true, '2026-01-10 09:00:00'),
    (2, 'Menu Découverte Corse',
        'Un menu complet pour faire découvrir à vos convives les saveurs authentiques de la cuisine corse. Idéal pour les repas d''entreprise et les groupes.',
        42.00, 15, 'autre', 'standard',
        'Réservation au minimum 7 jours à l''avance. Service à table inclus.',
        12, true, '2026-01-15 10:00:00'),
    (3, 'Menu Végétarien du Maquis',
        'Une cuisine végétarienne généreuse et parfumée, qui puise dans les richesses végétales de la Corse : brocciu, châtaigne, légumes du maquis.',
        38.00, 10, 'anniversaire', 'vegetarien',
        'Réservation 5 jours à l''avance minimum.',
        5, true, '2026-02-01 11:00:00'),
    (4, 'Menu Baptême & Communion',
        'Menu festif adapté aux célébrations familiales, avec des plats appréciés de tous, des enfants aux grands-parents.',
        48.00, 20, 'bapteme', 'standard',
        'Gâteau de célébration non inclus. Prestataire recommandé sur demande.',
        6, true, '2026-02-10 09:30:00'),
    (5, 'Menu Sans Gluten',
        'Tous les plats de ce menu sont préparés sans gluten, pour que chacun puisse profiter du repas sans contrainte.',
        45.00, 10, 'anniversaire', 'sans_gluten',
        'Cuisine préparée dans un espace dédié pour éviter les contaminations croisées. Merci de nous prévenir de toute allergie complémentaire.',
        4, true, '2026-03-01 10:00:00');

INSERT INTO menu_plat (id, menu_id, plat_id) VALUES
    -- Menu 1 : Prestige Mariage
    (1,  1,  3), (2,  1,  5), (3,  1, 12),
    (4,  1, 19), (5,  1, 21), (6,  1, 13),
    (7,  1, 25), (8,  1, 26),
    -- Menu 2 : Découverte Corse
    (9,  2,  1), (10, 2,  2),
    (11, 2, 14), (12, 2, 15),
    (13, 2, 26), (14, 2, 27),
    -- Menu 3 : Végétarien du Maquis
    (15, 3,  4), (16, 3,  8),
    (17, 3, 13),
    (18, 3, 26), (19, 3, 27),
    -- Menu 4 : Baptême & Communion
    (20, 4,  3), (21, 4,  6),
    (22, 4, 19), (23, 4, 13),
    (24, 4, 25), (25, 4, 26),
    -- Menu 5 : Sans Gluten
    (26, 5,  4), (27, 5,  7),
    (28, 5, 21), (29, 5, 18),
    (30, 5, 27);

-- ── 10. Menus restaurant ────────────────────────────────────

INSERT INTO menu_resto (id, nom, prix, description, disponible) VALUES
    (1, 'Menu à la découverte de Corte', 27.00, 'Entrée + Plat + Dessert au choix parmi notre sélection du jour', true);

INSERT INTO menu_resto_plat (menu_id, plat_id, cours) VALUES
    (1,  1, 'entree'),
    (1,  2, 'entree'),
    (1,  4, 'entree'),
    (1, 14, 'plat'),
    (1, 15, 'plat'),
    (1, 13, 'plat'),
    (1, 25, 'dessert'),
    (1, 26, 'dessert'),
    (1, 27, 'dessert');

-- ── 11. Info restaurant & horaires ──────────────────────────

INSERT INTO info_resto (id, nom, adresse, telephone, email, description)
VALUES (1, 'U Campanile', 'Corte, 20250 Haute-Corse', '04 95 61 00 00', 'contact@ucampanile.fr',
        'Restaurant de cuisine corse traditionnelle à Corte, Haute-Corse.')
ON CONFLICT (id) DO NOTHING;

INSERT INTO horaires (jour, ouverture_midi, fermeture_midi, ouverture_soir, fermeture_soir, note, ferme) VALUES
    ('lundi',    NULL,       NULL,       NULL,       NULL,       NULL, true),
    ('mardi',    '12:00:00', '14:00:00', '19:00:00', '21:30:00', NULL, false),
    ('mercredi', '12:00:00', '14:00:00', '19:00:00', '21:30:00', NULL, false),
    ('jeudi',    '12:00:00', '14:00:00', '19:00:00', '21:30:00', NULL, false),
    ('vendredi', '12:00:00', '14:00:00', '19:00:00', '22:00:00', NULL, false),
    ('samedi',   '12:00:00', '14:00:00', '19:00:00', '22:00:00', NULL, false),
    ('dimanche', '12:00:00', '14:30:00', NULL,       NULL,       NULL, false)
ON CONFLICT (jour) DO NOTHING;

-- ── 12. Resynchronisation des séquences PostgreSQL ──────────

DO $$
DECLARE t text;
BEGIN
    FOREACH t IN ARRAY ARRAY[
        'allergenes',
        'plats', 'plat_allergene', 'garnitures', 'plat_garnitures',
        'supplements', 'boissons',
        'formule_jours', 'menu_enfants', 'menu_enfant_allergenes',
        'menus', 'menu_plat',
        'menu_resto', 'menu_resto_plat',
        'horaires', 'info_resto',
        'commandes', 'commande_lignes', 'commande_ligne_garnitures', 'commande_statuts',
        'avis', 'avis_plats', 'devis_traiteur', 'contacts', 'eihwaz_users'
    ] LOOP
        EXECUTE format(
            'SELECT setval(pg_get_serial_sequence(%L, ''id''), COALESCE(MAX(id), 1)) FROM %I',
            t, t
        );
    END LOOP;
END $$;
