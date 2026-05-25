-- ══════════════════════════════════════════════════════════════
-- Seed U Campanile — données réelles complètes
-- Schéma géré par migrations
-- ══════════════════════════════════════════════════════════════

SET client_encoding = 'UTF8';

-- ── 0. Nettoyage complet ────────────────────────────────────
DELETE FROM commande_menu_choix;
DELETE FROM commande_ligne_garnitures;
DELETE FROM commande_lignes;
DELETE FROM commandes;
DELETE FROM avis_plats;
DELETE FROM avis;
DELETE FROM menu_traiteur_plats;
DELETE FROM menus_traiteur;
DELETE FROM menu_entrees;
DELETE FROM menu_plats;
DELETE FROM menu_desserts;
DELETE FROM entree_allergene;
DELETE FROM dessert_allergene;
DELETE FROM plat_garnitures;
DELETE FROM plat_allergene;
DELETE FROM supplements;
DELETE FROM garnitures;
DELETE FROM boissons;
DELETE FROM entrees;
DELETE FROM desserts;
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

-- ── 2a. Entrées ──────────────────────────────────────────────

INSERT INTO entrees (id, titre, label, prix, description, image, disponible, usage, ordre) VALUES
    (1,  'Soupe traditionnelle mijotée à l''os de jambon',
         'Soupe jambon',       12.00, NULL,                                                                     NULL, true, 'les_deux', 0),
    (2,  'Notre terrine de sanglier aux baies de myrte',
         'Terrine sanglier',   14.00, NULL,                                                                     NULL, true, 'les_deux', 0),
    (3,  'Planche Charcuterie Corse',
         'Charcuterie',        17.00, NULL,                                                                     NULL, true, 'les_deux', 0),
    (4,  'Houmous, légumes crus, huile de sésame',
         'Houmous',            16.00, NULL,                                                                     NULL, true, 'les_deux', 0),
    (5,  'Mi-cuit foie gras de canard Origine France et ses toasts',
         'Foie gras',          22.00, '100 g environ',                                                          NULL, true, 'les_deux', 0),
    (6,  'Le pain du bandit',
         'Pain bandit',        15.00, 'Tartine fromage et jambon corse, figues, pesto, roquette',               NULL, true, 'les_deux', 0),
    (7,  'Salade de poulpe tiède en persillade, pommes de terre vapeur',
         'Poulpe',             14.00, NULL,                                                                     NULL, true, 'les_deux', 0),
    (8,  'Bowl Végé solo',
         'Bowl Végé S',         8.50, 'Riz, tomate, carotte, concombre, avocat, ananas, houmous',              NULL, true, 'les_deux', 0),
    (9,  'Bowl Végé à partager',
         'Bowl Végé L',        17.00, 'Riz, tomate, carotte, concombre, avocat, ananas, houmous',              NULL, true, 'les_deux', 0),
    (10, 'Bowl Saumon fumé solo',
         'Bowl Saumon S',       9.50, 'Riz, avocat, tomate, concombre, carotte, houmous',                      NULL, true, 'les_deux', 0),
    (11, 'Bowl Saumon fumé à partager',
         'Bowl Saumon L',      19.50, 'Riz, avocat, tomate, concombre, carotte, houmous',                      NULL, true, 'les_deux', 0),
    (12, 'U Spuntinu',
         'U Spuntinu',         24.00, 'Planche de charcuterie avec fromages, confiture et terrine de sanglier', NULL, true, 'les_deux', 0);

-- ── 2b. Plats (spécialités, viandes, poissons) ───────────────

INSERT INTO plats (id, titre, label, type_plat, prix, description, image, disponible, est_viande, usage) VALUES
    (13, 'Cannelloni fraîches broccio-blettes, sauce tomate',
         'Cannelloni brocciu', 'specialite', 17.00, NULL,                                                      NULL, true, false, 'les_deux'),
    (14, 'Nos saucisses de sanglier maison à l''usu Figatellu, purée au lait',
         'Saucisses sanglier', 'specialite', 21.00, NULL,                                                      NULL, true, false, 'les_deux'),
    (15, 'Truite fraîche poêlée à la Népita, pommes de terre, légumes croquants',
         'Truite Népita',      'specialite', 16.50, NULL,                                                      NULL, true, false, 'les_deux'),
    (16, 'Gnocchi sauce morilles, foie gras frais poêlé, noisettes torréfiées',
         'Gnocchi morilles',   'viande',     32.00, NULL,                                                      NULL, true, false, 'les_deux'),
    (17, 'Émincé de volaille panée à la noisette, légumes croquants, pommes de terre',
         'Volaille noisette',  'viande',     18.50, NULL,                                                      NULL, true, false, 'les_deux'),
    (18, '« Corsican pulled porc thaï style », purée au lait, épinards',
         'Pulled porc',        'viande',     22.00, NULL,                                                      NULL, true, false, 'les_deux'),
    (19, 'Filet de bœuf charolais, sauce morilles, pommes de terre',
         'Filet bœuf',         'viande',     32.00, 'Option Rossini +7€',                                      NULL, true, true,  'les_deux'),
    (20, 'Gnocchi et veau Corse façon bolognaise, tomme de brebis, châtaignes',
         'Gnocchi veau',       'viande',     24.00, NULL,                                                      NULL, true, false, 'les_deux'),
    (21, 'Filet de daurade cuit sur peau, émulsion citronnée, petits légumes',
         'Daurade',            'poisson',    27.00, NULL,                                                      NULL, true, false, 'les_deux'),
    (22, 'Penne aux gambas du chef, sauce crustacés',
         'Penne gambas',       'poisson',    34.00, NULL,                                                      NULL, true, false, 'les_deux'),
    (23, 'Seiche poêlée aux noix de cajou, sauce vierge, pomme de terre, salade',
         'Seiche cajou',       'poisson',    26.00, NULL,                                                      NULL, true, false, 'les_deux'),
    (24, 'Healthy Burger Saumon fumé, avocat, crème cheese, pommes de terre, salade',
         'Burger saumon',      'poisson',    21.00, NULL,                                                      NULL, true, false, 'les_deux');

-- ── 2c. Desserts ─────────────────────────────────────────────

INSERT INTO desserts (id, titre, label, prix, description, image, disponible, usage, ordre) VALUES
    (25, 'Fromages Cortenais, confiture de figue',  'Fromages',   8.50, NULL,          NULL, true, 'les_deux', 0),
    (26, 'Fiadone — Douceur Brocciu-citron',        'Fiadone',    7.00, NULL,          NULL, true, 'les_deux', 0),
    (27, 'Myrte glacée',                            'Myrte glacée',6.50,'Sans lactose',NULL, true, 'les_deux', 0);

-- ── 3a. Allergènes des entrées ───────────────────────────────

INSERT INTO entree_allergene (id, entree_id, allergene_id) VALUES
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
    (15, 12, 12), (16, 12, 7);

-- ── 3b. Allergènes des plats ─────────────────────────────────

INSERT INTO plat_allergene (id, plat_id, allergene_id) VALUES
    -- Cannelloni brocciu : gluten, lait, œufs
    (1, 13, 1),  (2, 13, 7),  (3, 13, 3),
    -- Saucisses sanglier : sulfites
    (4, 14, 12),
    -- Truite Népita : poissons
    (5, 15, 4),
    -- Gnocchi morilles : gluten, lait, fruits à coque (noisettes)
    (6, 16, 1),  (7, 16, 7),  (8, 16, 8),
    -- Volaille noisette : fruits à coque
    (9, 17, 8),
    -- Pulled porc : lait (purée)
    (10, 18, 7),
    -- Gnocchi veau : gluten, lait (tomme)
    (11, 20, 1),  (12, 20, 7),
    -- Daurade : poissons
    (13, 21, 4),
    -- Penne gambas : gluten, crustacés
    (14, 22, 1),  (15, 22, 2),
    -- Seiche cajou : mollusques, fruits à coque
    (16, 23, 14), (17, 23, 8),
    -- Burger saumon : poissons, lait (crème cheese)
    (18, 24, 4),  (19, 24, 7);

-- ── 3c. Allergènes des desserts ──────────────────────────────

INSERT INTO dessert_allergene (id, dessert_id, allergene_id) VALUES
    -- Fromages cortenais : lait
    (1, 25, 7),
    -- Fiadone : lait, œufs
    (2, 26, 7),  (3, 26, 3);

-- ── 4. Garnitures ───────────────────────────────────────────

INSERT INTO garnitures (id, libelle, type_garniture, disponible) VALUES
    (1, 'Purée au lait',          'feculent', true),
    (2, 'Pommes de terre vapeur', 'feculent', true),
    (3, 'Frites maison',          'feculent', true),
    (4, 'Riz',                    'feculent', true),
    (5, 'Gnocchi maison',         'feculent', true),
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

INSERT INTO supplements (id, garniture_id, titre, libelle, prix, disponible) VALUES
    (1, NULL, 'Option Rossini',  'Foie gras frais poêlé',        7.00, true),
    (2, 6,    NULL,              'Légumes grillés en supplément', 3.50, true),
    (3, 1,    NULL,              'Purée maison en supplément',    3.50, true),
    (4, 3,    NULL,              'Frites maison en supplément',   3.50, true),
    (5, NULL, 'Pain artisanal',  NULL,                            2.00, true),
    (6, NULL, 'Sauce du chef',   NULL,                            2.50, true);

-- ── 6. Boissons réelles ─────────────────────────────────────

INSERT INTO boissons (id, titre, type_boisson, prix, description, image, disponible) VALUES
    (1,  'Alzipratu « Pumonte » — Calvi 75cl',                   'vin_rouge',     42.00, NULL, NULL, true),
    (2,  'Pero longo « Le lion de Roccapina » — Sartène 75cl',   'vin_rouge',     46.00, NULL, NULL, true),
    (3,  'Giacometti « Batolaccio » — Patrimonio 75cl',          'vin_rouge',     31.00, NULL, NULL, true),
    (4,  'Ornasca — Ajaccio 75cl',                               'vin_rouge',     29.00, NULL, NULL, true),
    (5,  'Guelfucci « Sciacarellu » — Corte 75cl',               'vin_rouge',     34.00, NULL, NULL, true),
    (6,  'Devichi « Mlle D » — Patrimonio 75cl',                 'vin_rouge',     27.00, NULL, NULL, true),
    (7,  'San Armettu « Myrtus » — AOP Sartène 75cl',            'vin_rouge',     49.00, NULL, NULL, true),
    (8,  'Landry « Cuvée Léa » — AOP Calvi 75cl',                'vin_rouge',     54.00, NULL, NULL, true),
    (9,  'Peretti della Rocca « Jules » — Figari 75cl',          'vin_rouge',     44.00, NULL, NULL, true),
    (10, 'Pero longo — Sartène 50cl',                            'vin_rouge',     21.50, NULL, NULL, true),
    (11, 'Ornasca — Ajaccio 50cl',                               'vin_rouge',     21.00, NULL, NULL, true),
    (12, 'Vin AOP Rouge au verre',                               'vin_rouge',      6.50, 'Demandez la suggestion du jour', NULL, true),
    (13, 'Vin IGP La cave d''Aléria — Rouge',                    'vin_rouge',      3.50, 'Verre 3,5€ · 25cl 5€ · 50cl 8€', NULL, true),
    (14, 'Guelfucci « Laurina » — Corte 75cl',                   'vin_rose',      24.00, NULL, NULL, true),
    (15, 'Pero Longo « Harmonie » — Sartène 75cl',               'vin_rose',      27.00, NULL, NULL, true),
    (16, 'Lucciardi « Signora Catalina » — AOP de Corse 75cl',   'vin_rose',      29.00, NULL, NULL, true),
    (17, 'Pero Longo — Sartène 50cl',                            'vin_rose',      21.00, NULL, NULL, true),
    (18, 'Vin AOP Rosé au verre',                                'vin_rose',       6.50, 'Demandez la suggestion du jour', NULL, true),
    (19, 'Vin IGP La cave d''Aléria — Rosé',                     'vin_rose',       3.50, 'Verre 3,5€ · 25cl 5€ · 50cl 8€', NULL, true),
    (20, 'Castellu di Baricci — Sartène 75cl',                   'vin_blanc',     49.00, NULL, NULL, true),
    (21, 'Marquiliani « Blanc de noir » — AOP de Corse 75cl',    'vin_blanc',     39.00, NULL, NULL, true),
    (22, 'Bertolozzi « Inghjò » — Oletta 75cl',                  'vin_blanc',     27.00, NULL, NULL, true),
    (23, 'Suale « Simbiosi » — Patrimonio 75cl',                 'vin_blanc',     52.00, NULL, NULL, true),
    (24, 'Domaine Orsucci — AOP de Corse 75cl',                  'vin_blanc',     28.00, NULL, NULL, true),
    (25, 'Vin AOP Blanc au verre',                               'vin_blanc',      6.50, 'Demandez la suggestion du jour', NULL, true),
    (26, 'Vin IGP La cave d''Aléria — Blanc',                    'vin_blanc',      3.50, 'Verre 3,5€ · 25cl 5€ · 50cl 8€', NULL, true),
    (27, 'Baron de Rothschild Brut — la coupe',                  'champagne',     12.00, NULL, NULL, true),
    (28, 'Baron de Rothschild Brut — bouteille 75cl',            'champagne',     90.00, NULL, NULL, true),
    (29, 'PAOLINA Pression 25cl',                                'biere',          4.50, NULL, NULL, true),
    (30, 'PAOLINA Pression 50cl',                                'biere',          8.00, NULL, NULL, true),
    (31, 'Panaché 25cl',                                         'biere',          4.00, NULL, NULL, true),
    (32, 'PAOLINA IPA 33cl',                                     'biere',          5.50, NULL, NULL, true),
    (33, 'Pietra 33cl',                                          'biere',          5.00, NULL, NULL, true),
    (34, 'Pietra Limoncello 33cl',                               'biere',          5.50, NULL, NULL, true),
    (35, 'Kiara Sunset — Despé Corse',                           'biere',          6.50, NULL, NULL, true),
    (36, 'Paolina sans Alcool',                                  'biere',          6.50, NULL, NULL, true),
    (37, 'Bud 25cl',                                             'biere',          4.00, NULL, NULL, true),
    (38, 'Muscat Mlle Devichi / Cap Corse rouge ou blanc',       'aperitif',       6.00, NULL, NULL, true),
    (39, 'Muscat Pétillant / Prosecco',                          'aperitif',       6.00, NULL, NULL, true),
    (40, 'Martini Rouge ou Blanc',                               'aperitif',       5.50, NULL, NULL, true),
    (41, 'Pastis',                                               'aperitif',       2.50, NULL, NULL, true),
    (42, 'Capo Spritz, Limoncello, Myrtle Spritz ou Apérol',    'aperitif',      10.00, NULL, NULL, true),
    (43, 'Sodas (Coca, Orangina, Ice Tea, Schweppes...)',        'soft',           3.20, NULL, NULL, true),
    (44, 'Limonade corse / Diabolo',                             'soft',           3.50, NULL, NULL, true),
    (45, 'Jus Pago 25cl (Pomme, Pêche, Tomate, Ananas)',        'soft',           3.20, NULL, NULL, true),
    (46, 'Sirop à l''eau (Grenadine, fraise, citron, menthe...)', 'soft',          1.50, NULL, NULL, true),
    (47, 'Eau plate Zilia 33cl',                                 'eau_bouteille',  2.50, NULL, NULL, true),
    (48, 'Eau plate Zilia 50cl',                                 'eau_bouteille',  3.80, NULL, NULL, true),
    (49, 'Eau plate Zilia 1L',                                   'eau_bouteille',  4.50, NULL, NULL, true),
    (50, 'Eau gazeuse Orezza 33cl',                              'eau_bouteille',  3.20, NULL, NULL, true),
    (51, 'Eau gazeuse Orezza 50cl',                              'eau_bouteille',  4.10, NULL, NULL, true),
    (52, 'Eau gazeuse Orezza 1L',                                'eau_bouteille',  5.20, NULL, NULL, true),
    (53, 'Myrte / Limoncello / U Getu',                         'digestif',       6.00, NULL, NULL, true),
    (54, 'Thé / Infusion Kusmi Tea',                             'digestif',       3.00, NULL, NULL, true),
    (55, 'Expresso / Décaféiné',                                 'digestif',       1.90, NULL, NULL, true);

-- ── 7. Menus traiteur ───────────────────────────────────────

INSERT INTO menus_traiteur (id, titre, description, prix_par_personne, nb_personnes_min, theme, regime, conditions, stock, actif, created_at) VALUES
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

INSERT INTO menu_traiteur_plats (id, menu_traiteur_id, plat_id) VALUES
    -- Menu 1 : Prestige Mariage
    (1,  1, 19), (2,  1, 21), (3,  1, 13),
    -- Menu 2 : Découverte Corse
    (4,  2, 14), (5,  2, 15),
    -- Menu 3 : Végétarien du Maquis
    (6,  3, 13),
    -- Menu 4 : Baptême & Communion
    (7,  4, 19), (8,  4, 13),
    -- Menu 5 : Sans Gluten
    (9,  5, 21), (10, 5, 18);

-- ── 8. Menus unifiés ────────────────────────────────────────

INSERT INTO menus (id, type_menu, nom, description, prix, ordre, entree_libre, plat_libre, dessert_libre) VALUES
    (1, 'menu_resto',   'Menu à la découverte de Corte',
        'Entrée + Plat + Dessert au choix parmi notre sélection du jour',
        27.00, 1, NULL, NULL, NULL),
    (2, 'menu_enfant',  'Menu Pinocchio',
        'Le menu des petits gourmands',
        12.00, 1, NULL, 'Spaghetti bolognaise maison', 'Glace 2 boules au choix'),
    (3, 'formule_jour', 'Magret de canard aux cèpes',
        'Servi avec pommes de terre sarladaises',
        18.00, 1, NULL, NULL, 'Fiadone du jour');

INSERT INTO menu_entrees (id, menu_id, entree_id) VALUES
    (1, 1, 1),
    (2, 1, 2),
    (3, 1, 4);

INSERT INTO menu_plats (id, menu_id, plat_id) VALUES
    (1, 1, 14),
    (2, 1, 15),
    (3, 1, 13);

INSERT INTO menu_desserts (id, menu_id, dessert_id) VALUES
    (1, 1, 25),
    (2, 1, 26),
    (3, 1, 27);

-- ── 8. Info restaurant & horaires ───────────────────────────

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

-- ── 9. Resynchronisation des séquences PostgreSQL ───────────

DO $$
DECLARE t text;
BEGIN
    FOREACH t IN ARRAY ARRAY[
        'allergenes',
        'entrees', 'entree_allergene',
        'plats', 'plat_allergene', 'garnitures', 'plat_garnitures',
        'desserts', 'dessert_allergene',
        'supplements', 'boissons',
        'menus_traiteur', 'menu_traiteur_plats',
        'menus', 'menu_entrees', 'menu_plats', 'menu_desserts',
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
