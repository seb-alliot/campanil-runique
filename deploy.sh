#!/usr/bin/env bash
# Déploie Campanile sur le VPS : ./deploy.sh
# Le build tourne en tâche de fond et survit à la fermeture du terminal.
# Les migrations restent manuelles : ce script n'en lance aucune.
set -euo pipefail

cd "$(dirname "$(readlink -f "$0")")"

if pgrep -f 'docker compose build' >/dev/null; then
    echo "Un build tourne déjà. Suivi : tail -f deploy.log" >&2
    exit 1
fi

sudo git pull --ff-only origin main

# Le Dockerfile copie Cargo.lock : il doit désigner le même Runique que la CLI et
# ses fichiers statiques, installés depuis la branche main.
cargo update -p runique

# --no-cache : sinon Docker peut réutiliser l'étape qui installe Runique et garder
# ses anciens fichiers statiques.
setsid nohup bash -c '
    echo "=== Début : $(date)"
    docker compose build --no-cache && docker compose up -d
    code=$?
    echo "=== Fin : $(date), code de sortie $code"
' >>deploy.log 2>&1 </dev/null &

echo "Build lancé en tâche de fond. Suivi : tail -f deploy.log"
