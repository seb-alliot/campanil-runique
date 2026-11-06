(function () {
    'use strict';

    const zone = document.getElementById('materiel-actions');
    if (!zone) return;
    const id = zone.dataset.commandeId;

    function jetonCsrf() {
        const champ = document.querySelector('input[name="csrf_token"]');
        return champ ? champ.value : '';
    }

    function envoyer(action, bouton, reussi) {
        bouton.disabled = true;
        fetch('/api/admin/commandes/' + id + '/' + action, {
            method: 'POST',
            headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
            body: 'csrf_token=' + encodeURIComponent(jetonCsrf()),
            credentials: 'same-origin',
        })
            .then(function (reponse) { return reponse.json(); })
            .then(function (data) {
                if (data.ok) {
                    reussi(data);
                } else {
                    bouton.disabled = false;
                    alert('Erreur : ' + (data.error || 'inconnue'));
                }
            })
            .catch(function () {
                bouton.disabled = false;
                alert('Erreur réseau.');
            });
    }

    const rendu = document.getElementById('btn-materiel-rendu');
    const penalite = document.getElementById('btn-penalite');

    rendu.addEventListener('click', function () {
        envoyer('marquer-rendu', rendu, function () {
            rendu.textContent = 'Matériel marqué rendu';
            if (penalite) penalite.remove();
        });
    });

    if (penalite) {
        penalite.addEventListener('click', function () {
            if (!confirm('Appliquer la pénalité de non-retour du matériel et envoyer le mail au client ?')) return;
            envoyer('appliquer-penalite', penalite, function (data) {
                penalite.textContent = 'Pénalité appliquée (' + (data.montant || '') + ')';
            });
        });
    }
})();
