(function () {
    'use strict';

    var POLL_MS = 5000;
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

    function esc(s) {
        return String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    }

    // --- Onglets principaux ---
    var tabs = document.querySelectorAll('.svc-tab');
    var tabCommandes = document.getElementById('tab-commandes');
    var tabStats = document.getElementById('tab-stats');

    var tabStock = document.getElementById('tab-stock');

    function showTab(name) {
        tabs.forEach(function (b) { b.classList.remove('active'); });
        var btn = document.querySelector('.svc-tab[data-tab="' + name + '"]');
        if (btn) btn.classList.add('active');
        if (tabCommandes) tabCommandes.classList.toggle('svc-hidden', name !== 'commandes');
        if (tabStock) tabStock.classList.toggle('svc-hidden', name !== 'stock');
        if (tabStats) tabStats.classList.toggle('svc-hidden', name !== 'stats');
    }

    tabs.forEach(function (btn) {
        btn.addEventListener('click', function () { showTab(btn.dataset.tab); });
    });

    // --- Sous-onglets Midi / Soir ---
    var subtabs = document.querySelectorAll('.svc-subtab');
    var subtabMidi = document.getElementById('subtab-midi');
    var subtabSoir = document.getElementById('subtab-soir');

    function showSubtab(name) {
        subtabs.forEach(function (b) { b.classList.remove('active'); });
        var btn = document.querySelector('.svc-subtab[data-subtab="' + name + '"]');
        if (btn) btn.classList.add('active');
        if (subtabMidi) subtabMidi.classList.toggle('svc-hidden', name !== 'midi');
        if (subtabSoir) subtabSoir.classList.toggle('svc-hidden', name !== 'soir');
    }

    subtabs.forEach(function (btn) {
        btn.addEventListener('click', function () { showSubtab(btn.dataset.subtab); });
    });

    // --- Rendu cartes ---
    function renderCard(c, readonly) {
        var cls = c.statut_valeur.replace(/_/g, '-');
        var dis = readonly ? ' disabled' : '';
        var actions = '';
        if (c.statut_valeur === 'en_attente') {
            actions = '<button class="svc-btn svc-btn--accepter" data-next="accepte"' + dis + '>Accepter</button>'
                    + '<button class="svc-btn svc-btn--annuler" data-next="annule"' + dis + '>Annuler</button>';
        } else if (c.statut_valeur === 'accepte') {
            actions = '<button class="svc-btn svc-btn--preparer" data-next="en_preparation"' + dis + '>En préparation</button>'
                    + '<button class="svc-btn svc-btn--annuler" data-next="annule"' + dis + '>Annuler</button>';
        } else if (c.statut_valeur === 'en_preparation') {
            actions = '<button class="svc-btn svc-btn--pret" data-next="pret"' + dis + '>Prêt</button>';
        } else if (c.statut_valeur === 'pret') {
            actions = c.type_retrait === 'livraison'
                ? '<button class="svc-btn svc-btn--livrer" data-next="en_cours_livraison"' + dis + '>En livraison</button>'
                : '<button class="svc-btn svc-btn--terminer" data-next="termine"' + dis + '>Terminé</button>';
        } else if (c.statut_valeur === 'en_cours_livraison') {
            actions = '<button class="svc-btn svc-btn--terminer" data-next="livre"' + dis + '>Livré</button>';
        }

        var lignes = c.lignes.map(function (l) {
            var top = '<div class="svc-ligne-top">'
                + '<span class="svc-qte">×' + l.quantite + '</span>'
                + '<span class="svc-titre">' + esc(l.titre) + '</span>';
            if (l.cuisson) top += '<span class="svc-detail">' + esc(l.cuisson) + '</span>';
            if (l.sans_sel) top += '<span class="svc-detail svc-detail--alert">sans sel</span>';
            top += '</div>';

            var compo = '';
            if ((l.garnitures && l.garnitures.length) || l.avec_legumes) {
                compo = '<ul class="svc-composition">';
                if (l.garnitures) l.garnitures.forEach(function (g) { compo += '<li>' + esc(g) + '</li>'; });
                if (l.avec_legumes) compo += '<li>+ légumes</li>';
                compo += '</ul>';
            }

            var note = l.note ? '<p class="svc-note">' + esc(l.note) + '</p>' : '';

            var menuChoix = '';
            if (l.menu_choix && l.menu_choix.length) {
                menuChoix = '<ul class="svc-menu-choix">';
                l.menu_choix.forEach(function (ch) {
                    menuChoix += '<li class="svc-menu-cours">'
                        + '<span class="svc-menu-cours-label">' + esc(ch.cours_label) + '</span>'
                        + '<span class="svc-menu-cours-titre">' + esc(ch.titre) + '</span>';
                    if (ch.cuisson) menuChoix += '<span class="svc-detail">' + esc(ch.cuisson) + '</span>';
                    if (ch.sans_sel) menuChoix += '<span class="svc-detail svc-detail--alert">sans sel</span>';
                    if ((ch.garnitures && ch.garnitures.length) || ch.avec_legumes) {
                        menuChoix += '<ul class="svc-composition">';
                        if (ch.garnitures) ch.garnitures.forEach(function (g) { menuChoix += '<li>' + esc(g) + '</li>'; });
                        if (ch.avec_legumes) menuChoix += '<li>+ légumes</li>';
                        menuChoix += '</ul>';
                    }
                    if (ch.note) menuChoix += '<p class="svc-note">' + esc(ch.note) + '</p>';
                    menuChoix += '</li>';
                });
                menuChoix += '</ul>';
            }

            return '<li class="svc-ligne">' + top + compo + note + menuChoix + '</li>';
        }).join('');

        var adresse = c.adresse_livraison ? '<p class="svc-adresse">' + esc(c.adresse_livraison) + '</p>' : '';

        return '<article class="svc-card svc-card--' + cls + '" data-numero="' + esc(c.numero) + '">'
            + '<div class="svc-card-header">'
            + '<span class="svc-heure">' + esc(c.heure) + '</span>'
            + '<span class="svc-numero">#' + esc(c.numero) + '</span>'
            + '<span class="svc-badge svc-badge--' + cls + '">' + esc(c.statut) + '</span>'
            + '</div>'
            + '<div class="svc-card-body">'
            + '<p class="svc-client">' + esc(c.client) + '</p>'
            + adresse
            + '<ul class="svc-lignes">' + lignes + '</ul>'
            + '</div>'
            + '<div class="svc-card-footer">'
            + '<span class="svc-prix">' + esc(c.prix_total) + ' €</span>'
            + '<span class="svc-paiement">' + esc(c.mode_paiement) + '</span>'
            + '<div class="svc-actions" data-numero="' + esc(c.numero) + '" data-statut="' + esc(c.statut_valeur) + '">'
            + actions
            + '</div>'
            + '</div>'
            + '</article>';
    }

    function renderGrille(groupeId, grilleId, commandes, readonly) {
        var groupe = document.getElementById(groupeId);
        var grille = document.getElementById(grilleId);
        if (!groupe || !grille) return;
        grille.innerHTML = commandes.map(function (c) { return renderCard(c, readonly); }).join('');
        groupe.hidden = commandes.length === 0;
    }

    function renderService(prefix, data, readonly) {
        var ra = data[prefix + '_retraits_attente']     || [];
        var rac = data[prefix + '_retraits_accepte']    || [];
        var rpr = data[prefix + '_retraits_preparation']|| [];
        var rpt = data[prefix + '_retraits_pret']       || [];
        var la = data[prefix + '_livraisons_attente']     || [];
        var lac = data[prefix + '_livraisons_accepte']    || [];
        var lpr = data[prefix + '_livraisons_preparation']|| [];
        var lpt = data[prefix + '_livraisons_pret']       || [];

        var P = prefix.charAt(0).toUpperCase() + prefix.slice(1);

        renderGrille('groupe' + P + 'RetraitAttente',     'grille' + P + 'RetraitAttente',     ra,  readonly);
        renderGrille('groupe' + P + 'RetraitAccepte',     'grille' + P + 'RetraitAccepte',     rac, readonly);
        renderGrille('groupe' + P + 'RetraitPreparation', 'grille' + P + 'RetraitPreparation', rpr, readonly);
        renderGrille('groupe' + P + 'RetraitPret',        'grille' + P + 'RetraitPret',        rpt, readonly);
        renderGrille('groupe' + P + 'LivraisonAttente',     'grille' + P + 'LivraisonAttente',     la,  readonly);
        renderGrille('groupe' + P + 'LivraisonAccepte',     'grille' + P + 'LivraisonAccepte',     lac, readonly);
        renderGrille('groupe' + P + 'LivraisonPreparation', 'grille' + P + 'LivraisonPreparation', lpr, readonly);
        renderGrille('groupe' + P + 'LivraisonPret',        'grille' + P + 'LivraisonPret',        lpt, readonly);

        var nbRetrait   = ra.length + rac.length + rpr.length + rpt.length;
        var nbLivraison = la.length + lac.length + lpr.length + lpt.length;

        var cntR = document.getElementById('count' + P + 'Retrait');
        if (cntR) cntR.textContent = nbRetrait;
        var cntL = document.getElementById('count' + P + 'Livraison');
        if (cntL) cntL.textContent = nbLivraison;

        var videR = document.getElementById('vide' + P + 'Retrait');
        if (videR) videR.hidden = nbRetrait > 0;
        var videL = document.getElementById('vide' + P + 'Livraison');
        if (videL) videL.hidden = nbLivraison > 0;

        return nbRetrait + nbLivraison;
    }

    function poll() {
        fetch('/service/commandes.json', { credentials: 'same-origin' })
            .then(function (r) { return r.json(); })
            .then(function (data) {
                var nbMidi = renderService('midi', data, false);
                var nbSoir = renderService('soir', data, !!data.midi_actif);

                var cntMidi = document.getElementById('tabCountMidi');
                if (cntMidi) cntMidi.textContent = nbMidi;
                var cntSoir = document.getElementById('tabCountSoir');
                if (cntSoir) cntSoir.textContent = nbSoir;

                var sync = document.getElementById('svcSync');
                if (sync) {
                    var now = new Date();
                    sync.textContent = 'Mis à jour ' + now.toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
                }
            })
            .catch(function () {});
    }

    poll();
    timer = setInterval(poll, POLL_MS);

    // --- Stock ---
    var stkLoaded = false;

    function loadStock() {
        fetch('/service/stock.json', { credentials: 'same-origin' })
            .then(function (r) { return r.json(); })
            .then(function (data) { renderStock(data.menus || []); })
            .catch(function () {});
    }

    function renderStock(menus) {
        var body = document.getElementById('stkBody');
        if (!body) return;
        if (!menus.length) {
            body.innerHTML = '<tr><td colspan="3" class="stk-loading">Aucun menu traiteur.</td></tr>';
            return;
        }
        body.innerHTML = menus.map(function (m) {
            return '<tr data-id="' + m.id + '">'
                + '<td class="stk-nom">' + esc(m.titre) + (!m.actif ? ' <span class="stk-inactif">inactif</span>' : '') + '</td>'
                + '<td class="stk-col-num"><span class="stk-val' + (m.stock === 0 ? ' stk-val--zero' : '') + '">' + m.stock + '</span></td>'
                + '<td class="stk-col-actions">'
                + '<button class="stk-btn stk-btn--minus" data-id="' + m.id + '" data-delta="-1">−</button>'
                + '<input class="stk-input" type="number" min="0" value="' + m.stock + '" data-id="' + m.id + '">'
                + '<button class="stk-btn stk-btn--plus" data-id="' + m.id + '" data-delta="1">+</button>'
                + '</td>'
                + '</tr>';
        }).join('');
    }

    function stkPost(id, payload) {
        var body = new URLSearchParams(payload);
        body.append('csrf_token', csrf());
        fetch('/service/menus/' + id + '/stock', {
            method: 'POST',
            credentials: 'same-origin',
            headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
            body: body.toString(),
        })
            .then(function (r) { return r.json(); })
            .then(function (data) {
                if (!data.ok) return;
                var row = document.querySelector('#stkBody tr[data-id="' + id + '"]');
                if (!row) return;
                var val = row.querySelector('.stk-val');
                var input = row.querySelector('.stk-input');
                if (val) {
                    val.textContent = data.stock;
                    val.classList.toggle('stk-val--zero', data.stock === 0);
                }
                if (input) input.value = data.stock;
            })
            .catch(function () {});
    }

    document.addEventListener('click', function (e) {
        var deltaBtn = e.target.closest('.stk-btn[data-delta]');
        if (deltaBtn) {
            stkPost(deltaBtn.dataset.id, { delta: deltaBtn.dataset.delta });
            return;
        }
    });

    document.addEventListener('change', function (e) {
        var input = e.target.closest('.stk-input');
        if (input) {
            var val = parseInt(input.value, 10);
            if (!isNaN(val) && val >= 0) {
                stkPost(input.dataset.id, { stock: val });
            }
        }
    });

    tabs.forEach(function (btn) {
        btn.addEventListener('click', function () {
            if (btn.dataset.tab === 'stock' && !stkLoaded) {
                stkLoaded = true;
                loadStock();
            }
        });
    });

    var stkRefresh = document.getElementById('stkRefresh');
    if (stkRefresh) {
        stkRefresh.addEventListener('click', function () { loadStock(); });
    }

    // --- Actions commandes ---
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
        body.append('csrf_token', csrf());

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

// Stats
(function () {
    var chartJours = null;
    var chartTypes = null;

    var periodeSelect = document.getElementById('periodeSelect');
    if (periodeSelect) {
        periodeSelect.addEventListener('change', function () { loadStats(); });
    }

    function loadStats() {
        if (typeof Chart === 'undefined') return;
        var periode = periodeSelect ? periodeSelect.value : '7';
        fetch('/service/stats.json?periode=' + periode, { credentials: 'same-origin' })
            .then(function (r) { return r.json(); })
            .then(function (d) { renderStats(d); })
            .catch(function () {});
    }

    function renderStats(d) {
        setText('sNbCommandes', d.nb_commandes);
        setText('sNbCarte',     d.nb_carte);
        setText('sNbTraiteur',  d.nb_traiteur);
        setText('sCaTotal',     (Math.round(d.ca_total * 100) / 100).toFixed(2) + ' €');

        var labels = d.jours.map(function (j) { return j.date; });
        var ca     = d.jours.map(function (j) { return j.ca; });
        var nb     = d.jours.map(function (j) { return j.nb; });
        var joursEl = document.getElementById('chartJours');
        if (joursEl) {
            if (chartJours) { chartJours.destroy(); }
            chartJours = new Chart(joursEl, {
                type: 'bar',
                data: {
                    labels: labels,
                    datasets: [
                        { label: 'CA (€)', data: ca, backgroundColor: 'rgba(59,130,246,.7)', yAxisID: 'yCA', order: 2 },
                        { label: 'Commandes', data: nb, type: 'line', borderColor: '#f59e0b', backgroundColor: 'transparent', pointBackgroundColor: '#f59e0b', tension: .3, yAxisID: 'yNb', order: 1 },
                    ],
                },
                options: {
                    responsive: true,
                    plugins: { legend: { labels: { color: '#94a3b8' } } },
                    scales: {
                        yCA: { position: 'left',  ticks: { color: '#94a3b8' }, grid: { color: '#334155' }, title: { display: true, text: 'CA €', color: '#94a3b8' } },
                        yNb: { position: 'right', ticks: { color: '#94a3b8' }, grid: { drawOnChartArea: false }, title: { display: true, text: 'Nb', color: '#94a3b8' } },
                        x:   { ticks: { color: '#94a3b8' }, grid: { color: '#334155' } },
                    },
                },
            });
        }

        var typesEl = document.getElementById('chartTypes');
        if (typesEl) {
            if (chartTypes) { chartTypes.destroy(); }
            chartTypes = new Chart(typesEl, {
                type: 'doughnut',
                data: {
                    labels: ['Carte', 'Traiteur'],
                    datasets: [{ data: [d.nb_carte, d.nb_traiteur], backgroundColor: ['#3b82f6', '#f59e0b'] }],
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    plugins: { legend: { position: 'bottom', labels: { color: '#94a3b8' } } },
                },
            });
        }

        var tbodyPlats = document.getElementById('tbodyPlats');
        if (tbodyPlats) {
            tbodyPlats.innerHTML = d.top_plats.length
                ? d.top_plats.map(function (p) {
                    return '<tr><td>' + esc(p.titre) + '</td><td>' + p.vues + '</td></tr>';
                }).join('')
                : '<tr><td colspan="2" class="s-muted">Aucune donnée.</td></tr>';
        }

        var filtresWrap = document.getElementById('filtresWrap');
        if (filtresWrap) {
            if (!d.top_filtres.length) {
                filtresWrap.innerHTML = '<p class="s-muted">Aucune donnée.</p>';
            } else {
                var LABELS = { theme: 'Thème', regime: 'Régime', prix_min: 'Prix min.', prix_max: 'Prix max.', nb_personnes: 'Nb personnes' };
                var groups = {};
                d.top_filtres.forEach(function (f) {
                    if (!groups[f.filtre]) groups[f.filtre] = [];
                    groups[f.filtre].push(f);
                });
                filtresWrap.innerHTML = Object.keys(groups).map(function (key) {
                    var label = LABELS[key] || key.replace(/_/g, ' ');
                    var rows = groups[key].map(function (f) {
                        return '<tr><td>' + esc(f.valeur) + '</td><td>' + f.count + '</td></tr>';
                    }).join('');
                    return '<div class="s-filter-group">'
                        + '<p class="s-filter-group-label">' + esc(label) + '</p>'
                        + '<table class="s-table"><thead><tr><th>Valeur</th><th>Utilisations</th></tr></thead>'
                        + '<tbody>' + rows + '</tbody></table></div>';
                }).join('');
            }
        }
    }

    function setText(id, val) {
        var el = document.getElementById(id);
        if (el) el.textContent = val;
    }

    function esc(s) {
        return String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    }

    // Chargement lazy au premier affichage des stats
    var statsLoaded = false;
    document.querySelectorAll('.svc-tab').forEach(function (btn) {
        btn.addEventListener('click', function () {
            if (btn.dataset.tab === 'stats' && !statsLoaded) {
                statsLoaded = true;
                loadStats();
            }
        });
    });
})();
