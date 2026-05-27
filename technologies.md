# Technologies choisies — U Campanile

## Technologies

- Rust
- Runique
- CSS (vanilla)
- JavaScript (vanilla)
- PostgreSQL
- MongoDB

## Dépendances notables

- Let's Encrypt (ACME)
- SeaORM
- Tera
- Axum
- Serde

---

## Pourquoi ces choix ?

### Pourquoi Rust ?

J'ai choisi Rust pour ses performances et sa sécurité mémoire. C'est un langage compilé qui ne possède pas de garbage collector — la gestion mémoire est vérifiée à la compilation par le système d'ownership. Cela garantit qu'il n'y a pas de fuite mémoire ni de comportement indéfini à l'exécution.

Rust est souvent perçu comme difficile à apprendre, notamment à cause de son système d'emprunt (borrow checker) qui impose des règles strictes sur la durée de vie des références. Cependant, une fois ces concepts assimilés, il permet d'écrire du code robuste et performant sans avoir à gérer manuellement la mémoire comme en C.

Ce projet m'a permis de consolider ma pratique de Rust dans un contexte réel, avec des contraintes métier concrètes.

### Pourquoi Runique ?

Runique est un framework web Rust que j'ai développé moi-même, inspiré de Django. Ce projet est l'occasion de l'éprouver sur un cas réel avec des contraintes métier concrètes : commandes en ligne, panel d'administration, uploads, envoi d'emails, statistiques.

Il repose sur Axum (serveur HTTP), SeaORM (ORM) et Tera (moteur de templates), et fournit nativement :

- Un système de modèles avec migrations SQL automatiques via `derive_form!{}`
- Un panel d'administration généré automatiquement via `admin!{}`
- Un système de formulaires avec validation et rendu HTML
- La gestion des sessions, CSRF, CSP, rate limiting
- La distribution des fichiers statiques et médias
- Le HTTPS automatique via ACME (Let's Encrypt)

Utiliser Runique sur Campanile me permet de détecter les manques, les bugs et les frictions du framework en conditions réelles, et de l'améliorer en conséquence.

### Pourquoi CSS vanilla ?

J'ai choisi CSS pur sans framework (pas de Bootstrap, pas de Tailwind) pour garder un contrôle total sur le rendu et ne pas alourdir le projet avec des dépendances front inutilisées. Cela implique d'écrire plus de code, mais le résultat est plus léger et mieux maîtrisé.

### Pourquoi JavaScript vanilla ?

JavaScript est indispensable pour apporter du dynamisme à l'interface : mise à jour du panier sans rechargement, gestion de la page service en temps réel, graphiques de statistiques. J'ai délibérément évité les frameworks front (React, Vue) pour rester sur du JavaScript natif, suffisant pour les besoins du projet et sans dépendance supplémentaire à maintenir.

### Pourquoi PostgreSQL ?

PostgreSQL est une base de données relationnelle robuste, adaptée aux données structurées du projet : commandes, plats, menus, utilisateurs. Elle supporte les types énumérés natifs, les contraintes d'intégrité et les transactions, ce qui correspond bien aux besoins d'une application de commande en ligne.

C'est également la base de données la mieux supportée par SeaORM et Runique, ce qui en fait le choix naturel pour ce projet.

SeaORM sert de "traducteur" entre le code Rust et PostgreSQL, de la même façon que Psycopg2 le fait entre Python/Django et PostgreSQL. Il convertit les requêtes Rust en SQL et gère les migrations via les fichiers générés par Runique.

### Pourquoi MongoDB ?

MongoDB est une base de données non relationnelle utilisée uniquement pour les statistiques (vues de plats, filtres menus, événements de commandes). Ce type de données est par nature peu structuré et volumineuse — MongoDB est plus adapté que PostgreSQL pour les stocker et les agréger efficacement via ses pipelines d'agrégation.

---

## Les dépendances notables

### Pourquoi avoir recours à des dépendances ?

Les dépendances Rust (crates) ajoutent des fonctionnalités que le langage ou le framework ne fournissent pas nativement, ou simplifient des tâches complexes. En Rust, toutes les dépendances sont déclarées dans `Cargo.toml` et compilées dans le binaire final.

### Let's Encrypt (ACME)

Let's Encrypt est une autorité de certification (CA) qui délivre gratuitement des certificats SSL/TLS pour activer le protocole HTTPS.

Cela permet :
- de chiffrer les échanges entre le navigateur et le serveur,
- d'améliorer le référencement (SEO),
- d'éviter les alertes "Site non sécurisé" dans les navigateurs.

Runique intègre un client ACME qui émet et renouvelle le certificat automatiquement au démarrage, sans configuration externe. Lorsque plusieurs projets cohabitent sur le même serveur, c'est Certbot + Nginx qui prend en charge cette responsabilité à la place.

### SeaORM

SeaORM est l'ORM (Object-Relational Mapper) utilisé par Runique pour interagir avec PostgreSQL. Il joue le même rôle que Psycopg2 pour Django : convertir les opérations Rust en requêtes SQL et gérer les migrations de schéma.

### Tera

Tera est le moteur de templates utilisé par Runique pour générer les pages HTML. La syntaxe est proche de Jinja2 (utilisé par Django), avec les variables `{{ }}`, les blocs `{% %}` et l'héritage de templates. L'autoescape est actif par défaut sur les fichiers `.html`, ce qui protège contre les injections XSS.

### Axum

Axum est le framework HTTP bas niveau sur lequel repose Runique. Il gère le routage des requêtes et la sérialisation des réponses. Runique encapsule Axum de manière à ce que le développeur n'ait pas à l'utiliser directement dans la plupart des cas.

La gestion des sessions repose sur `tower-sessions`. La CSRF et la CSP sont implémentées par Runique sous forme de middlewares Axum. Le rate limiting est également un middleware Runique, configurable par route.

### Serde

Serde est la bibliothèque de sérialisation/désérialisation de Rust. Elle permet de convertir des structures Rust en JSON (et inversement) pour les réponses API, et de lire les fichiers de configuration. Elle est omniprésente dans l'écosystème Rust.

---

## Conclusion

Ce projet m'a permis de travailler avec une stack moderne et performante dans un contexte métier réel. Rust et Runique imposent une rigueur plus importante que Python/Django lors de la phase de développement, mais offrent en contrepartie des garanties fortes sur la correction du code, la sécurité mémoire et les performances à l'exécution. L'utilisation de deux bases de données complémentaires (PostgreSQL pour les données métier, MongoDB pour les statistiques) reflète le choix du bon outil selon la nature des données à gérer.
