# Déploiement sur VPS

## Vue d'ensemble

Runique/Axum gère nativement le HTTPS via le protocole **ACME (Let's Encrypt)** et distribue directement les fichiers statiques. Aucun reverse proxy n'est requis pour un projet seul sur un VPS.

```
Internet → Runique (443/80) → PostgreSQL + MongoDB
```

---

## ACME / Let's Encrypt intégré

Runique intègre un client ACME qui émet et renouvelle automatiquement les certificats Let's Encrypt au démarrage de l'application.

### Fonctionnement

Le protocole ACME demande à Let's Encrypt de vérifier que tu contrôles le domaine en répondant à un challenge HTTP sur `/.well-known/acme-challenge/`. Runique répond à ce challenge directement, sans configuration externe.

### Limite importante : 5 certificats par semaine par domaine

Let's Encrypt impose une limite de **5 émissions de certificat par domaine sur 7 jours glissants**. Dépasser cette limite bloque les émissions pendant plusieurs jours.

Valide d'abord toute ta configuration réseau (DNS, ports ouverts) avant d'activer `ACME_ENABLED=true` pour ne pas brûler des tentatives inutilement.

### Limite sans reverse proxy : un seul projet par VPS

Runique écoute directement sur les ports **80 et 443**. Un seul processus peut occuper ces ports à la fois — donc un seul projet Runique par VPS sans reverse proxy.

Si tu as plusieurs projets sur le même serveur, il faut passer par Nginx (voir section dédiée).

---

## Déploiement sans reverse proxy (projet seul)

### Structure Docker

```
campanile/
├── docker-compose.yml
└── .env
```

### docker-compose.yml

```yaml
services:
  app:
    image: ghcr.io/seb-alliot/campanile:latest
    restart: unless-stopped
    env_file: .env
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./media:/app/media
      - ./certs:/app/certs  # certificats ACME persistés
    depends_on:
      - postgres
      - mongo

  postgres:
    image: postgres:16-alpine
    restart: unless-stopped
    environment:
      POSTGRES_DB: campanile
      POSTGRES_USER: ${DB_USER}
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data

  mongo:
    image: mongo:7
    restart: unless-stopped
    volumes:
      - mongo_data:/data/db

volumes:
  postgres_data:
  mongo_data:
```

### .env production

```env
DEBUG=false
TZ=Europe/Paris
ENFORCE_HTTPS=true
ACME_ENABLED=true
ACME_DOMAIN=ton-domaine.fr
ACME_EMAIL=contact@ton-domaine.fr
ACME_CERTS_DIR=./certs

DB_ENGINE=postgres
DB_HOST=postgres
DB_PORT=5432
DB_NAME=campanile
DB_USER=campanile
DB_PASSWORD=mot_de_passe_fort

DATABASE_URL=postgresql://campanile:mot_de_passe_fort@postgres:5432/campanile

MONGO_URI=mongodb://mongo:27017

SECRET_KEY=cle_secrete_longue_et_aleatoire_minimum_32_chars

SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_STARTTLS=true
SMTP_USER=contact@ton-domaine.fr
SMTP_PASS=app_password
SMTP_FROM=contact@ton-domaine.fr
```

---

## Déploiement classique sans Docker (configuration actuelle — test-itsuki.fr)

C'est le mode de déploiement en place sur le VPS. Nginx fait le reverse proxy car plusieurs projets cohabitent sur le même serveur.

### 1. Prérequis sur le VPS

```bash
apt update && apt install -y nginx postgresql mongodb-org
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
cargo install runique
```

### 2. Cloner et compiler

```bash
git clone https://github.com/seb-alliot/campanile.git /opt/campanile
cd /opt/campanile
cargo build --release
```

### 3. Configurer l'environnement

```bash
cp .env.example .env
# Editer .env — DEBUG=false, ENFORCE_HTTPS=true, DB_HOST=localhost, MONGO_URI=mongodb://localhost:27017
```

### 4. Créer la base de données

```bash
sudo -u postgres psql -c "CREATE USER campanile WITH PASSWORD 'mot_de_passe';"
sudo -u postgres psql -c "CREATE DATABASE campanile OWNER campanile;"
```

### 5. Appliquer les migrations et le seed

```bash
cd /opt/campanile
runique migration up
psql -U campanile -d campanile -f seed.sql
runique create-superuser
```

### 6. Service systemd

Créer `/etc/systemd/system/campanile.service` :

```ini
[Unit]
Description=U Campanile
After=network.target postgresql.service

[Service]
User=www-data
WorkingDirectory=/opt/campanile
EnvironmentFile=/opt/campanile/.env
ExecStart=/opt/campanile/target/release/campanile
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

```bash
systemctl daemon-reload
systemctl enable campanile
systemctl start campanile
```

L'application tourne sur le port 3000 en interne. Configurable dans le .env .

### 7. Nginx — reverse proxy

Créer `/etc/nginx/sites-available/campanile` :

```nginx
server {
    listen 80;
    server_name test-itsuki.fr;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl;
    server_name test-itsuki.fr;

    ssl_certificate     /etc/letsencrypt/live/test-itsuki.fr/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/test-itsuki.fr/privkey.pem;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

```bash
ln -s /etc/nginx/sites-available/campanile /etc/nginx/sites-enabled/
nginx -t && systemctl reload nginx
```

### 8. Certificat TLS via Certbot

Dans ce cas Nginx gère le TLS — le ACME intégré de Runique n'est pas utilisé. C'est Certbot qui émet le certificat :

```bash
apt install -y certbot python3-certbot-nginx
certbot --nginx -d test-itsuki.fr
```

Le renouvellement automatique est géré par le timer systemd installé avec Certbot.

À ce stade le site est fonctionnel sur **[https://test-itsuki.fr](https://test-itsuki.fr)**.

### Mise à jour (classique)

```bash
cd /opt/campanile
git pull
cargo build --release
runique migration up
systemctl restart campanile
```

---

## Dockerfile

```dockerfile
# Étape 1 : compilation
FROM rust:1.88 AS builder
WORKDIR /app
RUN cargo install runique
COPY . .
RUN cargo build --release

# Étape 2 : image finale minimale
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/campanile .
COPY --from=builder /usr/local/cargo/bin/runique /usr/local/bin/runique
COPY templates/ templates/
COPY static/ static/
CMD ["./campanile"]
```

La CLI `runique` est copiée dans l'image finale pour pouvoir exécuter les migrations et créer le superutilisateur via `docker compose exec`.

---

## Déploiement initial

### 1. Préparer le VPS

```bash
apt update && apt install -y docker.io docker-compose-plugin
```

### 2. Cloner le projet

```bash
git clone https://github.com/seb-alliot/campanile.git /opt/campanile
cd /opt/campanile
```

### 3. Configurer l'environnement

```bash
cp .env.example .env
# Editer .env avec les vraies valeurs
```

### 4. Lancer les services

```bash
docker compose up -d
```

Runique émet le certificat Let's Encrypt automatiquement au premier démarrage. Le domaine doit pointer sur l'IP du VPS avant de lancer.

> La feature `acme` doit être activée dans `Cargo.toml` de Runique, sinon `ACME_ENABLED=true` affiche un avertissement et est ignoré.

### 5. Appliquer les migrations et le seed

```bash
docker compose exec app runique migration up
docker compose exec -T postgres psql -U $DB_USER -d campanile < seed.sql
```

### 6. Créer le superutilisateur

```bash
docker compose exec app runique create-superuser
```

---

## Mise à jour de l'application (Docker)

```bash
cd /opt/campanile
git pull
docker compose pull app
docker compose up -d app
docker compose exec app runique migration up
```

---

## Nginx — uniquement si plusieurs projets sur le même VPS

Si d'autres projets tournent sur le même serveur, Nginx prend les ports 80/443 et route vers chaque application sur un port interne différent. Dans ce cas, Runique tourne sur un port non-privilégié (ex. 3000) et **c'est Nginx qui gère le TLS** — le ACME intégré de Runique n'est plus utilisé.

```
Internet → Nginx (443/80) → Runique campanile (:3000)
                           → Autre projet (:3001)
```

Ce cas de figure est effectif car j'ai deux projet sur le vps, runique.io(la documentation actuel de runique et test-itsuki.fr (campanile avant ditribution officiel))
