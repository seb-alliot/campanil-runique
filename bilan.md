# Bilan de projet — U Campanile

## Ce qui a été réalisé

U Campanile est une application web de commande en ligne pour un restaurant corse, développée avec la stack **Rust + Runique + PostgreSQL + MongoDB**. Le projet couvre l'ensemble du périmètre fonctionnel prévu :

- Consultation de la carte et des menus traiteur avec filtres (thème, régime, prix, nombre de personnes)
- Commande en ligne depuis la carte, suivi et annulation
- Espace client : compte, historique, avis
- Panel d'administration : gestion des plats, menus, suppléments, commandes, employés
- Statistiques de consultation (MongoDB) : vues par plat, filtres menus, événements commandes
- Formulaires de contact, devis et commande traiteur
- Déploiement sur VPS avec HTTPS, reverse proxy Nginx et certificat Let's Encrypt

---

## Points forts

**Runique plus complet que prévu.** Les fonctionnalités transversales — sessions, CSRF, CSP, rate limiting, anti-bot, panel admin généré, formulaires avec validation, migrations SQL, envoi d'emails, SRI — étaient disponibles nativement. Cela a permis de se concentrer sur la logique métier sans réimplémenter l'infrastructure de sécurité.

**Double base de données cohérente.** L'utilisation de PostgreSQL pour les données métier et MongoDB pour les statistiques s'est avérée pertinente : chaque moteur est utilisé là où il est le plus efficace, sans chercher à tout faire tenir dans un seul système.

**Rust en production.** Le borrow checker impose une rigueur plus grande qu'un langage à garbage collector, mais les garanties obtenues en contrepartie sont réelles : pas de race condition, erreurs détectées à la compilation plutôt qu'à l'exécution. Les seules fuites mémoire potentielles identifiées proviennent de `tower-sessions` (accumulation de sessions anonymes) — corrigées par le `CleaningMemoryStore` de Runique via un watermark mémoire.

---

## Difficultés rencontrées

### Rust — courbe d'apprentissage

La principale difficulté du projet a été Rust lui-même. Le système d'ownership et le borrow checker imposent une façon de penser la mémoire qui n'a pas d'équivalent dans les langages courants. Les premières semaines ont nécessité de revoir des habitudes de développement acquises sur d'autres stacks. Une fois ces concepts assimilés, le développement est devenu fluide, mais la phase d'adaptation a été réelle.

### Modélisation M2M plat ↔ supplément

La relation entre les plats et leurs suppléments a demandé plusieurs itérations avant d'aboutir à une structure stable. La question centrale était de savoir si les suppléments étaient propres à chaque plat ou partagés, et comment modéliser les contraintes (gratuité, ordre d'affichage, disponibilité). Cette réflexion sur la structure des données en amont aurait pu éviter des migrations correctives.

Un facteur aggravant est lié à SeaORM : contrairement à Django qui génère automatiquement la table de jonction d'une relation `ManyToManyField`, SeaORM ne gère pas les M2M nativement. Chaque relation M2M doit être déclarée comme une entité à part entière, avec sa propre migration et ses propres relations. Ce choix de conception impose de matérialiser explicitement ce que d'autres ORM abstraient — ce qui est cohérent avec la philosophie de Rust (explicite plutôt qu'implicite), mais représente un coût de modélisation réel.

### Déploiement multi-projets — ACME et Nginx

Runique intègre un client ACME (Let's Encrypt) qui gère nativement le TLS. Ce mode fonctionne parfaitement pour un projet seul sur un VPS. En revanche, plusieurs projets cohabitant sur le même serveur ne peuvent pas partager les ports 80 et 443. Il a fallu reconfigurer le déploiement : Nginx prend en charge le TLS via Certbot, et Runique tourne sur un port interne. La limite de 5 certificats émis par domaine sur 7 jours glissants imposée par Let's Encrypt a également nécessité une validation soigneuse de la configuration réseau avant toute activation.

---

## Runique comme terrain d'épreuve

Campanile a été développé sur Runique, un framework Rust créé en parallèle. L'objectif était précisément d'éprouver le framework sur un cas métier réel. Cette démarche a généré des évolutions et correctifs dans Runique (macros, relations M2M SeaORM, middleware anti-bot, niveaux de log) qui n'auraient pas été identifiés sans les contraintes concrètes du projet. Les frictions rencontrées sont des retours de terrain, pas des blocages — elles ont enrichi le framework plutôt qu'alourdi le développement de Campanile.

---

## Axes d'amélioration

- Finaliser le tunnel de commande (paiement Stripe)
- Compléter la logique saisonnière : la période d'ouverture est configurable mais ne bloque pas encore les commandes hors saison
