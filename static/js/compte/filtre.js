(function () {
    const liste = document.querySelector('.commandes-list');
    const qs = new URLSearchParams(window.location.search);
    let statutCourant  = qs.get('statut')  || '';
    let serviceCourant = qs.get('service') || '';

    function esc(s) {
        return String(s ?? '').replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');
    }

    const renderLignes = lignes => lignes.map(l =>
        `<li><span class="commande-ligne-titre">${esc(l.titre)}</span>${l.cuisson ? `<span class="commande-ligne-cuisson">— ${l.cuisson}</span>` : ''}<span class="commande-ligne-qte">× ${l.quantite}</span><span class="commande-ligne-prix">${l.prix_unitaire} €</span></li>`
    ).join('');

    const renderCard = c => `
        <div class="commande-card" data-statut="${c.statut_css}">
            <div class="commande-card-header">
                <span class="commande-numero">N° ${c.numero}</span>
                <span class="commande-badge statut-${c.statut_css}">${c.statut_label}</span>
            </div>
            <div class="commande-meta">
                <span>${c.type_retrait === 'livraison' ? 'Livraison' : 'Sur place'}</span>
                <span>${c.mode_paiement}</span>
                <time class="commande-date">${c.date}</time>
                ${c.date_annulation ? `<span class="commande-annulation-date">Annulée le ${c.date_annulation}</span>` : ''}
            </div>
            ${c.lignes.length ? `<ul class="commande-lignes">${renderLignes(c.lignes)}</ul>` : ''}
            ${c.type_retrait === 'livraison'
                ? `<div class="commande-livraison"><span>Livraison</span>${c.adresse_livraison ? `<span>${esc(c.adresse_livraison)}${c.cp_livraison ? ', ' + esc(c.cp_livraison) : ''}${c.ville_livraison ? ' ' + esc(c.ville_livraison) : ''}</span>` : ''}${c.heure_retrait ? `<span>à ${c.heure_retrait}</span>` : ''}</div>`
                : c.heure_retrait ? `<div class="commande-retrait">Retrait prévu à ${c.heure_retrait}</div>` : ''}
            <div class="commande-card-footer">
                <strong class="commande-total">Total : ${c.prix_total} €</strong>
            </div>
        </div>`;

    const renderPagination = (page, totalPages) => {
        if (totalPages <= 1) return '';
        const prev = page > 1
            ? `<a href="#" class="btn btn-sm" data-page="${page - 1}">&larr; Précédent</a>`
            : `<span class="btn btn-sm disabled">&larr; Précédent</span>`;
        const next = page < totalPages
            ? `<a href="#" class="btn btn-sm" data-page="${page + 1}">Suivant &rarr;</a>`
            : `<span class="btn btn-sm disabled">Suivant &rarr;</span>`;
        return `<nav class="pagination js-pagination">${prev}<span class="pagination-info">Page ${page} / ${totalPages}</span>${next}</nav>`;
    };

    const charger = (statut, service, page) => {
        if (!liste) return;
        liste.style.transition = 'opacity .15s';
        liste.style.opacity = '0.4';

        fetch(`/compte/commandes.json?statut=${statut}&service=${service}&page=${page}`, { credentials: 'same-origin' })
            .then(r => r.json())
            .then(data => {
                document.querySelector('.pagination')?.remove();
                if (!data.commandes.length) {
                    liste.innerHTML = '<p class="compte-empty-filtre">Aucune commande pour ce filtre.</p>';
                } else {
                    liste.innerHTML = data.commandes.map(renderCard).join('');
                    const nav = renderPagination(data.page, data.total_pages);
                    if (nav) liste.insertAdjacentHTML('afterend', nav);
                }
                liste.style.opacity = '1';
            })
            .catch(() => {
                liste.innerHTML = '<p class="compte-empty-filtre">Erreur de chargement.</p>';
                liste.style.opacity = '1';
            });
    };

    document.addEventListener('click', e => {
        const btnStatut = e.target.closest('.js-filtre-statut');
        if (btnStatut) {
            const {filtre} = btnStatut.dataset;
            document.querySelectorAll('.js-filtre-statut').forEach(b => {
                b.classList.toggle('btn-primary', b === btnStatut);
                b.classList.toggle('btn-secondary', b !== btnStatut);
            });
            const map = {'en-cours': 'en_cours', 'termine': 'termine', 'annule': 'annule'};
            const { [filtre]: statut = '' } = map;
            statutCourant = statut;
            charger(statutCourant, serviceCourant, 1);
            return;
        }

        const btnService = e.target.closest('.js-filtre-service');
        if (btnService) {
            serviceCourant = btnService.dataset.service;
            document.querySelectorAll('.js-filtre-service').forEach(b => {
                b.classList.toggle('btn-primary', b === btnService);
                b.classList.toggle('btn-secondary', b !== btnService);
            });
            charger(statutCourant, serviceCourant, 1);
            return;
        }

        const pagLink = e.target.closest('.pagination a[data-page]');
        if (pagLink) {
            e.preventDefault();
            charger(statutCourant, serviceCourant, parseInt(pagLink.dataset.page));
        }
    });
})();
