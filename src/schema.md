table restaurant


plat

id
titre
label                      sous-titre court (ex: "Spécialité maison")
type_plat                  (Entree | Specialite | Viande | Poisson | Plat | Dessert)
description
allergene => M2M allergene
garniture => M2M garniture  (avec est_defaut sur la jointure)
est_viande                 (bool)
avec_legumes               (bool, default true)
prix_supplement_legumes    (decimal, default 0)
image
prix
ordre                      (tri dans la carte)
disponible


menu_resto

id
nom
description
entrees => M2M plat (type=Entree)
plats => M2M plat (type=Viande | Poisson | Plat | Specialite)
desserts => M2M plat (type=Dessert)
prix
disponible
ordre


menu_enfant

id
titre
description
plat                       texte libre
dessert                    texte libre
allergene => M2M allergene
prix
image
actif


formule_jour               ← plat du jour, hors carte, champs libres

id
titre                      ex: "Magret de canard aux cèpes"
description
prix
dessert                    texte libre


garniture

id
libelle                    ex: "Frites", "Riz", "Salade"
type_garniture             (Feculent | Legumes)
disponible


allergene

id
libelle                    ex: "Gluten", "Lactose"


boisson

id
titre
type_boisson               (VinRouge | VinRose | VinBlanc | Champagne | Biere | Aperitif | Soft | Eau | Digestif)
description
prix
image
ordre
disponible


supplement

id
garniture                  => fk garniture (nullable)
titre                      nom affiché (ex: "Fromage râpé", "Sauce piquante") — saisie manuelle
libelle                    description courte (nullable)
prix
disponible


commande

id
user                       => fk user
numero
heure_commande
type_retrait               (sur_place | livraison)
adresse_livraison
ville_livraison
cp_livraison
heure_retrait
statut
prix_total
mode_paiement              (Especes | CarteBancaire | EnLigne)
prix_livraison
stripe_payment_intent_id   (nullable)
motif_annulation           (nullable)
date_annulation            (nullable)
created_at
updated_at


commande_ligne             ← une ligne par article commandé

id
commande                   => fk commande
type                       (plat | menu_resto | formule_jour | menu_enfant | boisson | supplement)
plat_id                    => fk plat (nullable)
menu_resto_id              => fk menu_resto (nullable)
formule_jour_id            => fk formule_jour (nullable)
menu_enfant_id             => fk menu_enfant (nullable)
boisson_id                 => fk boisson (nullable)
supplement_id              => fk supplement (nullable)
garnitures                 => M2M garniture via commande_ligne_garniture (choix pour ce plat/menu)
cuisson                    (Bleu | Saignant | APoint | BienCuit — nullable)
avec_legumes               (bool, default false)
sans_sel                   (bool, default false)
note                       texte libre
quantite
prix_unitaire              snapshot du prix au moment de la commande


commande_statut            ← historique des changements de statut

id
commande                   => fk commande
statut
note                       texte libre (optionnel)
created_at


avis                       ← avis sur la commande / le resto

id
user                       => fk user
commande                   => fk commande (unique — un avis par commande)
note                       (1 à 5)
commentaire
statut                     (EnAttente | Valide | Refuse)
created_at


avis_plat                  ← avis sur un plat, libre avec modération

id
user                       => fk user (nullable — anonyme possible ?)
plat                       => fk plat
note                       (1 à 5)
commentaire
statut                     (EnAttente | Valide | Refuse)
created_at


---
Fonctionnalités annexes (hors schéma principal)

menu_traiteur              ← commandes traiteur, devis, événements
  id, titre, description, prix_par_personne, nb_personnes_min
  theme : enum (Mariage | Bapteme | Anniversaire | Entreprise | Autre)
  regime : enum (Standard | Vegetarien | SansGluten | Autre)
  conditions, stock, actif, created_at

devis_traiteur             ← formulaire devis (menu_id nullable, nom, email, téléphone, date, nb_personnes, message, statut)


---
Entités conservées hors carte

horaire                    ← jour, ouverture_midi, fermeture_midi, ouverture_soir, fermeture_soir, ferme, note
info_resto                 ← nom, adresse, téléphone, email, facebook, instagram, tripadvisor, google_maps, description, prix_livraison
contact                    ← titre, description, email, created_at
user_profil                ← extension eihwaz_users : téléphone, adresse, ville, cp, pays


---
Décisions validées

- type_plat : 6 types fins conservés (Entree, Specialite, Viande, Poisson, Plat, Dessert)
- garniture.type_garniture : Feculent | Legumes
- plat_garniture.est_defaut : garniture proposée par défaut pour un plat
- formule_jour : une seule active, uniquement midi (possibilité de l'activer le soir)
- menu_enfant : choix multiples (plusieurs versions)
- menu_resto.desserts : M2M vers plat (pas texte libre)
- menu_traiteur : fonctionnalité annexe, séparée
- regime/theme : conservés, rattachés au traiteur
- commande_statut : conservé (historique)
- commande : champs traiteur retirés (séparation carte/traiteur)
- avis.statut : enum EnAttente | Valide | Refuse (plus riche qu'un simple bool)
- avis_plat : libre avec modération admin
- avis anonyme (avis_plat) : à décider
- page /avis : avis plats validés, regroupés par plat avec note moyenne — argument commercial
- fiche plat (detail/zoom) : affiche note moyenne + lien "X avis — voir les retours" → /avis
- stripe_payment_intent_id : sur commande (paiement en ligne)
- panier : session uniquement (persisté via eihwaz_sessions DB fallback) — pas de table panier
- ajout au panier sans connexion : modal bloquant avec href="/connexion?next=/carte" (middleware open redirect actif)
