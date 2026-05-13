(function () {
    'use strict';

    var POLL_MS = 30000;
    var timer = null;

    function clock() {
        var el = document.getElementById('svcClock');
        if (!el) return;
        var now = new Date();
        el.textContent = now.toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
    }
    setInterval(clock, 1000);
    clock();

    function csrf() {
        var m = document.querySelector('meta[name="csrf-token"]');
        return m ? m.getAttribute('content') : '';
    }

    function statutClass(valeur) {
        return valeur.replace(/_/g, '-');
    }

    function renderCard(c) {
        var actions = '';
        if (c.statut_valeur === 'en_attente') {
            actions = '<button class="svc-btn svc-btn--accepter" data-next="accepte">Accepter</button>'
                    + '<button class="svc-btn svc-btn--annuler" data-next="annule">Annuler</button>';
        } else if (c.statut_valeur === 'accepte') {
            actions = '<button class="svc-btn svc-btn--preparer" data-next="en_preparation">En préparation</button>'
                    + '<button class="svc-btn svc-btn--annuler" data-next="annule">Annuler</button>';
        } else if (c.statut_valeur === 'en_preparation') {
            actions = '<button class="svc-btn svc-btn--pret" data-next="pret">Prêt</button>';
        } else if (c.statut_valeur === 'pret') {
            actions = c.avec_livraison
                ? '<button class="svc-btn svc-btn--livrer" data-next="en_cours_livraison">En livraison</button>'
                : '<button class="svc-btn svc-btn--terminer" data-next="termine">Terminé</button>';
        } else if (c.statut_valeur === 'en_cours_livraison') {
            actions = '<button class="svc-btn svc-btn--terminer" data-next="livre">Livré</button>';
        }

        var lignes = c.lignes.map(function (l) {
            var detail = '';
            if (l.cuisson) detail += '<span class="svc-detail">' + l.cuisson + '</span>';
            if (l.note) detail += '<span class="svc-detail svc-note">' + l.note + '</span>';
            return '<li class="svc-ligne"><span class="svc-qte">×' + l.quantite + '</span>'
                 + '<span class="svc-titre">' + l.titre + '</span>' + detail + '</li>';
        }).join('');

        var adresse = c.adresse_livraison ? '<p class="svc-adresse">' + c.adresse_livraison + '</p>' : '';
        var cls = statutClass(c.statut_valeur);

        return '<article class="svc-card svc-card--' + cls + '" data-numero="' + c.numero + '">'
            + '<div class="svc-card-header">'
            + '<span class="svc-heure">' + c.heure + '</span>'
            + '<span class="svc-numero">#' + c.numero + '</span>'
            + '<span class="svc-badge svc-badge--' + cls + '">' + c.statut + '</span>'
            + '</div>'
            + '<div class="svc-card-body">'
            + '<p class="svc-client">' + c.client + '</p>'
            + adresse
            + '<ul class="svc-lignes">' + lignes + '</ul>'
            + '</div>'
            + '<div class="svc-card-footer">'
            + '<span class="svc-prix">' + c.prix_total + ' €</span>'
            + '<span class="svc-paiement">' + c.mode_paiement + '</span>'
            + '<div class="svc-actions" data-numero="' + c.numero + '" data-statut="' + c.statut_valeur + '">'
            + actions
            + '</div>'
            + '</div>'
            + '</article>';
    }

    function renderGrille(groupeId, grilleId, commandes) {
        var groupe = document.getElementById(groupeId);
        var grille = document.getElementById(grilleId);
        if (!groupe || !grille) return;
        grille.innerHTML = commandes.map(renderCard).join('');
        groupe.hidden = commandes.length === 0;
    }

    function renderSection(prefix, attente, accepte, preparation, pret, videId, countId) {
        var total = attente.length + accepte.length + preparation.length + pret.length;
        var countEl = document.getElementById(countId);
        if (countEl) countEl.textContent = total;
        var vide = document.getElementById(videId);
        if (vide) vide.hidden = total > 0;

        renderGrille('groupe' + prefix + 'Attente',     'grille' + prefix + 'Attente',     attente);
        renderGrille('groupe' + prefix + 'Accepte',     'grille' + prefix + 'Accepte',     accepte);
        renderGrille('groupe' + prefix + 'Preparation', 'grille' + prefix + 'Preparation', preparation);
        renderGrille('groupe' + prefix + 'Pret',        'grille' + prefix + 'Pret',        pret);
    }

    function poll() {
        fetch('/service/commandes.json', { credentials: 'same-origin' })
            .then(function (r) { return r.json(); })
            .then(function (data) {
                renderSection('Retrait',
                    data.retraits_attente      || [],
                    data.retraits_accepte      || [],
                    data.retraits_preparation  || [],
                    data.retraits_pret         || [],
                    'videRetrait', 'countRetrait');
                renderSection('Livraison',
                    data.livraisons_attente      || [],
                    data.livraisons_accepte      || [],
                    data.livraisons_preparation  || [],
                    data.livraisons_pret         || [],
                    'videLivraison', 'countLivraison');
                var sync = document.getElementById('svcSync');
                if (sync) {
                    var now = new Date();
                    sync.textContent = 'Mis à jour ' + now.toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
                }
            })
            .catch(function () {});
    }

    function startPoll() {
        if (timer) clearInterval(timer);
        timer = setInterval(poll, POLL_MS);
    }
    startPoll();

    document.addEventListener('click', function (e) {
        var btn = e.target.closest('.svc-btn[data-next]');
        if (!btn) return;

        var actions = btn.closest('.svc-actions');
        if (!actions) return;
        var numero = actions.dataset.numero;
        var next = btn.dataset.next;

        btn.disabled = true;
        var label = btn.textContent;
        btn.textContent = '…';

        var body = new URLSearchParams();
        body.append('statut', next);
        body.append('csrftoken', csrf());

        fetch('/service/commandes/' + numero + '/statut', {
            method: 'POST',
            credentials: 'same-origin',
            headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
            body: body.toString(),
        })
            .then(function (r) { return r.json(); })
            .then(function (data) {
                if (data.ok) {
                    poll();
                } else {
                    btn.textContent = label;
                    btn.disabled = false;
                }
            })
            .catch(function () {
                btn.textContent = label;
                btn.disabled = false;
            });
    });
})();
