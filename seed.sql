-- Seed campanile — données de démonstration
-- Compatible SQLite et PostgreSQL

-- Thèmes
INSERT INTO themes (id, libelle) VALUES
    (1, 'Mariage'),
    (2, 'Baptême'),
    (3, 'Anniversaire'),
    (4, 'Repas d''entreprise'),
    (5, 'Communion');

-- Régimes alimentaires
INSERT INTO regimes (id, libelle) VALUES
    (1, 'Omnivore'),
    (2, 'Végétarien'),
    (3, 'Sans gluten'),
    (4, 'Vegan');

-- Allergènes (14 majeurs UE)
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

-- Plats (type_plat : 'entree' | 'plat' | 'dessert')
INSERT INTO plats (id, titre, type_plat, prix, description, image, disponible, est_viande) VALUES
    -- Entrées
    (1,  'Charcuterie corse',              'entree',  12.00, 'Sélection de figatellu, lonzu et coppa du pays',                                   NULL, true, true),
    (2,  'Beignets de courgettes',         'entree',   8.50, 'Beignets légers, servis avec une sauce au yaourt et menthe',                       NULL, true, false),
    (3,  'Salade de poulpe à l''huile',    'entree',  11.00, 'Poulpe de roche, huile d''olive corse, citron et persil',                          NULL, true, false),
    (4,  'Soupe corse au lard et légumes', 'entree',   9.00, 'Recette traditionnelle mijotée avec haricots, blettes et lard paysan',             NULL, true, true),
    (5,  'Carpaccio de bœuf corse',        'entree',  13.50, 'Bœuf de montagne tranché fin, huile d''olive, copeaux de brocciu sec et roquette', NULL, true, true),
    (6,  'Velouté de châtaigne',           'entree',   9.50, 'Crème de châtaigne corse, noisettes torréfiées, huile de noix',                   NULL, true, false),

    -- Plats principaux
    (7,  'Veau aux olives corses',          'plat',   24.00, 'Épaule de veau braisée longuement avec des olives de Balagne et des herbes du maquis', NULL, true, true),
    (8,  'Stufatu de bœuf',                 'plat',   22.00, 'Ragoût de bœuf à la corse, mijotés au vin rouge et aux tomates',                      NULL, true, true),
    (9,  'Truite de montagne à la plancha', 'plat',   20.00, 'Truite des rivières corses, beurre citronné, légumes grillés de saison',              NULL, true, false),
    (10, 'Raviolis au brocciu et menthe',   'plat',   18.00, 'Pâtes fraîches maison farcies au brocciu AOC, sauce tomate aux herbes',              NULL, true, false),
    (11, 'Agneau de lait rôti',             'plat',   26.00, 'Agneau corse rôti au four, pommes de terre et romarin du maquis',                     NULL, true, true),
    (12, 'Poêlée de légumes du maquis',     'plat',   16.00, 'Courgettes, poivrons, aubergines, herbes aromatiques, huile d''olive AOP',            NULL, true, false),

    -- Desserts
    (13, 'Fiadone au brocciu',         'dessert',  7.50, 'Gâteau corse au fromage frais brocciu AOC et zeste de citron',                   NULL, true, false),
    (14, 'Canistrelli aux amandes',    'dessert',  5.00, 'Biscuits secs corses à l''anis et aux amandes, croquants et parfumés',           NULL, true, false),
    (15, 'Fraîcheur d''agrumes',       'dessert',  6.50, 'Suprêmes de clémentines de Corse, sorbet citron, écorces confites',              NULL, true, false),
    (16, 'Crème de châtaigne',         'dessert',  6.00, 'Mousse légère à la farine de châtaigne corse, chantilly et éclats de marrons',  NULL, true, false),
    (17, 'Assiette de fromages corses','dessert',  8.50, 'Sélection de brocciu frais, tomme et niolo, accompagnée de confiture de figue', NULL, true, false);

-- Allergènes des plats
INSERT INTO plat_allergene (id, plat_id, allergene_id) VALUES
    -- Charcuterie corse : sulfites
    (1,  1, 12),
    -- Beignets de courgettes : gluten, œufs
    (2,  2, 1),
    (3,  2, 3),
    -- Salade de poulpe : mollusques
    (4,  3, 14),
    -- Soupe corse : gluten, céleri
    (5,  4, 1),
    (6,  4, 9),
    -- Carpaccio : lait (brocciu)
    (7,  5, 7),
    -- Velouté châtaigne : fruits à coque (noisettes)
    (8,  6, 8),
    -- Veau aux olives : sulfites (vin)
    (9,  7, 12),
    -- Stufatu : gluten, sulfites
    (10, 8, 1),
    (11, 8, 12),
    -- Truite : poissons
    (12, 9, 4),
    -- Raviolis : gluten, œufs, lait
    (13, 10, 1),
    (14, 10, 3),
    (15, 10, 7),
    -- Agneau rôti : sulfites
    (16, 11, 12),
    -- Fiadone : lait, œufs
    (17, 13, 7),
    (18, 13, 3),
    -- Canistrelli : gluten, fruits à coque (amandes), œufs
    (19, 14, 1),
    (20, 14, 8),
    (21, 14, 3),
    -- Châtaigne : fruits à coque
    (22, 16, 8),
    -- Fromages corses : lait
    (23, 17, 7);

-- Menus traiteur
INSERT INTO menus (id, titre, description, prix_par_personne, nb_personnes_min, theme_id, regime_id, conditions, stock, actif, created_at) VALUES
    (1,
     'Menu Prestige Mariage',
     'Notre menu d''exception pour célébrer votre union en Corse. Un parcours gastronomique autour des plus belles saveurs de l''île, de la charcuterie aux desserts traditionnels.',
     65.00, 30, 1, 1,
     'Acompte de 30 % à la commande. Annulation possible jusqu''à 30 jours avant l''événement. Prestation comprise : dressage, service à table, débarrassage.',
     8, true, '2026-01-10 09:00:00'),

    (2,
     'Menu Découverte Corse',
     'Un menu complet pour faire découvrir à vos convives les saveurs authentiques de la cuisine corse. Idéal pour les repas d''entreprise et les groupes.',
     42.00, 15, 4, 1,
     'Réservation au minimum 7 jours à l''avance. Service à table inclus.',
     12, true, '2026-01-15 10:00:00'),

    (3,
     'Menu Végétarien du Maquis',
     'Une cuisine végétarienne généreuse et parfumée, qui puise dans les richesses végétales de la Corse : brocciu, châtaigne, légumes du maquis.',
     38.00, 10, 3, 2,
     'Réservation 5 jours à l''avance minimum.',
     5, true, '2026-02-01 11:00:00'),

    (4,
     'Menu Baptême & Communion',
     'Menu festif adapté aux célébrations familiales, avec des plats appréciés de tous, des enfants aux grands-parents.',
     48.00, 20, 2, 1,
     'Gâteau de célébration non inclus. Prestataire recommandé sur demande.',
     6, true, '2026-02-10 09:30:00'),

    (5,
     'Menu Sans Gluten',
     'Tous les plats de ce menu sont préparés sans gluten, pour que chacun puisse profiter du repas sans contrainte.',
     45.00, 10, 3, 3,
     'Cuisine préparée dans un espace dédié pour éviter les contaminations croisées. Merci de nous prévenir de toute allergie complémentaire.',
     4, true, '2026-03-01 10:00:00');

-- Composition des menus (menu_plat)
INSERT INTO menu_plat (id, menu_id, plat_id) VALUES
    -- Menu 1 : Prestige Mariage
    (1,  1,  1),  -- Charcuterie corse
    (2,  1,  5),  -- Carpaccio bœuf
    (3,  1,  7),  -- Veau aux olives
    (4,  1, 11),  -- Agneau rôti
    (5,  1, 13),  -- Fiadone
    (6,  1, 17),  -- Fromages corses

    -- Menu 2 : Découverte Corse
    (7,  2,  1),  -- Charcuterie corse
    (8,  2,  4),  -- Soupe corse
    (9,  2,  8),  -- Stufatu de bœuf
    (10, 2, 13),  -- Fiadone
    (11, 2, 14),  -- Canistrelli

    -- Menu 3 : Végétarien
    (12, 3,  2),  -- Beignets courgettes
    (13, 3,  6),  -- Velouté châtaigne
    (14, 3, 10),  -- Raviolis brocciu
    (15, 3, 12),  -- Poêlée légumes
    (16, 3, 15),  -- Fraîcheur agrumes
    (17, 3, 16),  -- Crème châtaigne

    -- Menu 4 : Baptême & Communion
    (18, 4,  2),  -- Beignets courgettes
    (19, 4,  1),  -- Charcuterie corse
    (20, 4,  7),  -- Veau aux olives
    (21, 4, 10),  -- Raviolis brocciu
    (22, 4, 13),  -- Fiadone
    (23, 4, 15),  -- Fraîcheur agrumes

    -- Menu 5 : Sans Gluten
    (24, 5,  1),  -- Charcuterie corse (sans gluten)
    (25, 5,  3),  -- Salade poulpe (sans gluten)
    (26, 5,  7),  -- Veau aux olives (sans gluten)
    (27, 5,  9),  -- Truite (sans gluten)
    (28, 5, 15),  -- Fraîcheur agrumes (sans gluten)
    (29, 5, 13);  -- Fiadone (sans gluten)

-- Resynchronisation des séquences PostgreSQL après inserts avec IDs explicites
DO $$
DECLARE
    t text;
BEGIN
    FOREACH t IN ARRAY ARRAY[
        'themes', 'regimes', 'allergenes', 'plats', 'plat_allergene',
        'menus', 'menu_plat', 'menu_images', 'horaires', 'contacts',
        'info_resto', 'commandes', 'commande_plats', 'commande_statuts',
        'avis', 'boissons', 'devis_traiteur'
    ]
    LOOP
        EXECUTE format(
            'SELECT setval(pg_get_serial_sequence(%L, ''id''), COALESCE(MAX(id), 1)) FROM %I',
            t, t
        );
    END LOOP;
END $$;

-- Fix FK CASCADE pour suppression des plats avec allergènes/menus liés
ALTER TABLE plat_allergene
    DROP CONSTRAINT IF EXISTS plat_allergene_plat_id_plats_fkey,
    ADD CONSTRAINT plat_allergene_plat_id_plats_fkey
        FOREIGN KEY (plat_id) REFERENCES plats(id) ON DELETE CASCADE;

ALTER TABLE menu_plat
    DROP CONSTRAINT IF EXISTS menu_plat_plat_id_plats_fkey,
    ADD CONSTRAINT menu_plat_plat_id_plats_fkey
        FOREIGN KEY (plat_id) REFERENCES plats(id) ON DELETE CASCADE;
