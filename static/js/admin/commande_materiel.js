(function () {
    'use strict';

    function csrfToken() {
        const el = document.querySelector('input[name="csrf_token"]');
        return el ? el.value : '';
    }

    function commandeId() {
        const parts = window.location.pathname.split('/');
        for (let i = 0; i < parts.length - 1; i++) {
            if (parts[i] === 'commandes' && /^\d+$/.test(parts[i + 1])) {
                return parts[i + 1];
            }
        }
        return null;
    }

    function postAction(url, onSuccess) {
        fetch(url, {
            method: 'POST',
            headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
            body: 'csrf_token=' + encodeURIComponent(csrfToken()),
            credentials: 'same-origin',
        })
            .then(function (r) { return r.json(); })
            .then(onSuccess)
            .catch(function () { alert('Erreur réseau.'); });
    }

    function init() {
        const statutEl = document.querySelector('select[name="statut"]');
        const pretEl = document.querySelector('input[name="pret_materiel"]');
        const penaliteEl = document.querySelector('input[name="penalite_envoyee"]');
        if (!statutEl || !pretEl) return;

        const id = commandeId();
        if (!id) return;

        const estLivre = statutEl.value === 'livre';
        const pretRendu = pretEl.value === 'true';
        const penaliteAppliquee = penaliteEl && penaliteEl.value === 'true';
        const container = statutEl.closest('.form-group') || statutEl.parentNode;

        // Bouton "Matériel rendu"
        if (estLivre && !pretRendu) {
            const btnRendu = document.createElement('button');
            btnRendu.type = 'button';
            btnRendu.className = 'btn btn-warning';
            btnRendu.style.marginTop = '1rem';
            btnRendu.textContent = 'Matériel rendu';
            container.appendChild(btnRendu);

            btnRendu.addEventListener('click', function () {
                btnRendu.disabled = true;
                postAction('/api/admin/commandes/' + id + '/marquer-rendu', function (data) {
                    if (data.ok) {
                        pretEl.value = 'true';
                        btnRendu.textContent = 'Matériel marqué rendu';
                        btnRendu.classList.remove('btn-warning');
                        btnRendu.classList.add('btn-success');
                        const bp = container.querySelector('.btn-penalite');
                        if (bp) bp.remove();
                    } else {
                        btnRendu.disabled = false;
                        alert('Erreur : ' + (data.error || 'inconnue'));
                    }
                });
            });
        }

        // Bouton "Appliquer la pénalité"
        if (estLivre && !pretRendu && !penaliteAppliquee) {
            const btnPenalite = document.createElement('button');
            btnPenalite.type = 'button';
            btnPenalite.className = 'btn btn-danger btn-penalite';
            btnPenalite.style.marginTop = '0.5rem';
            btnPenalite.style.marginLeft = '0.5rem';
            btnPenalite.textContent = 'Appliquer la pénalité';
            container.appendChild(btnPenalite);

            btnPenalite.addEventListener('click', function () {
                if (!confirm('Appliquer la pénalité de non-retour du matériel et envoyer le mail au client ?')) return;
                btnPenalite.disabled = true;
                postAction('/api/admin/commandes/' + id + '/appliquer-penalite', function (data) {
                    if (data.ok) {
                        if (penaliteEl) penaliteEl.value = 'true';
                        btnPenalite.textContent = 'Pénalité appliquée (' + (data.montant || '') + ')';
                        btnPenalite.classList.remove('btn-danger');
                        btnPenalite.classList.add('btn-success');
                    } else {
                        btnPenalite.disabled = false;
                        alert('Erreur : ' + (data.error || 'inconnue'));
                    }
                });
            });
        }
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }
})();
