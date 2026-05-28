# U Campanile — Restaurant corse à Corte

---

## Stack technique

[Repo github](https://github.com/seb-alliot/campanil-runique)

*Techno utilisé*

- [Java script : vanilla]
- [Trello](https://trello.com/b/7rohcKGQ/campanile)
- [Base de donnée : Postgres(relationnel)](https://www.postgresql.org/)
- [Base de donnée non relationnel : Mongo db](https://www.mongodb.com/)
- [Rust](https://rustup.rs/) 1.88+installer en local pour les performances
- [Runique (rust 1.88)](https://img.shields.io/badge/Runique-brightgreen)
- [crate.io](https://runique.io)

runique utilise toutes les techno si dessous

- [Sea-orm](https://www.sea-ql.org/SeaORM/)
- [Moteur de templat = Tera](https://keats.github.io/tera/)
- [Server = axum](https://docs.rs/axum/latest/axum/)

Application web de commande en ligne pour le restaurant U Campanile, développée avec le framework Rust **Runique** (Axum + SeaORM + Tera).

---

## Prérequis

- [Rust](https://rustup.rs/) 1.88+
- PostgreSQL 15+
- MongoDB 7+
- Cargo (inclus avec Rust)
- sea-orm-cli (pour les migrations)

```bash
cargo install sea-orm-cli
```

---

## Installation locale

### 1. Cloner le dépôt

```bash
git clone https://github.com/seb-alliot/campanile.git
cd campanile
```

### 2. Configurer l'environnement

Créer un fichier `.env` à la racine :

```env
DEBUG=false
TZ=Europe/Paris

DB_ENGINE=postgres
DB_HOST=localhost
DB_PORT=5432
DB_NAME=campanile
DB_USER=postgres
DB_PASSWORD=votre_mot_de_passe

DATABASE_URL=postgresql://postgres:votre_mot_de_passe@localhost:5432/campanile

DB_ACQUIRE_TIMEOUT=2000
DB_CONNECT_TIMEOUT=2
DB_IDLE_TIMEOUT=300
DB_MAX_CONNECTIONS=10
DB_MAX_LIFETIME=3600
DB_MIN_CONNECTIONS=2

SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_STARTTLS=true
SMTP_USER=votre_email@gmail.com
SMTP_PASS=votre_app_password
SMTP_FROM=votre_email@gmail.com

SECRET_KEY=une_cle_secrete_longue_et_aleatoire

ENFORCE_HTTPS=false

MONGO_URI=mongodb://localhost:27017
```

### 3. Créer la base de données

```bash
psql -U postgres -c "CREATE DATABASE campanile;"
```

### 4. Appliquer les migrations

```bash
runique migration up
```

### 5. Créer un compte administrateur

```bash
runique create-superuser
```

### 6. Injecter les données de démonstration

```bash
psql -U postgres -d campanile -f seed.sql
```

### 6. Lancer l'application

```bash
cargo run
```

L'application est accessible sur `http://localhost:3000`.

---

## Comptes de démonstration

| Rôle | Email | Mot de passe |
|------|-------|--------------|
| Administrateur | admin@campanile.fr | Admin1234! |
| Employé | employe@campanile.fr | Employe1234! |
| Utilisateur | client@campanile.fr | Client1234! |

---

## Parcours disponibles

### Visiteur (non connecté)
- Consulter la carte et les menus traiteur
- Filtrer les menus (thème, régime, prix, personnes)
- Contacter le restaurant
- Créer un compte

### Utilisateur connecté
- Passer une commande depuis la carte
- Suivre et annuler ses commandes
- Laisser un avis
- Modifier ses informations personnelles

### Employé (panel admin)
- Gérer les menus, plats, entrées, desserts
- Suivre et mettre à jour le statut des commandes
- Valider ou refuser les avis clients

### Administrateur (panel admin)
- Tout ce que peut faire un employé
- Créer et désactiver des comptes employés
- Consulter les statistiques de commandes (graphiques)

Panel admin accessible sur `http://localhost:3000/admin-campanile`.

---

## Structure du projet

```
src/
├── main.rs              # Point d'entrée — RuniqueAppBuilder
├── url.rs               # Routes de l'application
├── views.rs             # Handlers publics
├── admin.rs             # Configuration du panel admin
├── entities/            # Modèles SeaORM
├── backend/             # Logique métier
│   ├── carte/           # Carte du restaurant
│   ├── menus/           # Menus traiteur
│   ├── panier/          # Gestion du panier
│   ├── commande/        # Commandes
│   ├── compte/          # Espace client
│   ├── service/         # Espace employé
│   └── stats/           # Statistiques MongoDB
├── admins/              # Code généré pour l'admin
└── formulaire.rs        # Formulaires

migration/               # Migrations SQL
templates/               # Templates Tera (HTML)
static/                  # CSS, JS, images
seed.sql                 # Données de démonstration
```
