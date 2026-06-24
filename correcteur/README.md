# Lancer Campanile en local (correction / prise en main)

Environnement **autonome** : app + PostgreSQL + MongoDB, sans rien installer
d'autre que Docker.

## Démarrage

```bash
cd correcteur
cp .env.example .env      # placeholders de dev, à ajuster si besoin
docker compose up --build
```

L'application est ensuite disponible sur <http://localhost:3000>.

## Migrations (base initiale)

Au premier lancement, la base est vide. Applique le schéma :

```bash
docker compose exec app ./migrate
```

> Adapter la commande selon le binaire de migration (ex. `./migrate up`).

## Pourquoi ce dossier séparé ?

La **production** (VPS) utilise un compose différent : `network_mode: host` avec
des instances PostgreSQL et MongoDB **mutualisées sur l'hôte** (choix assumé pour
faire tourner plusieurs projets sur la même machine, une seule base à sauvegarder).

Ce compose-ci **embarque** les bases (réseau bridge) pour qu'un tiers obtienne un
environnement complet en une commande, sans dépendre de l'infrastructure du VPS.
Il est isolé dans `correcteur/` pour ne pas interférer avec le déploiement.