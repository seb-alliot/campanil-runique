(function () {
    const toggle       = document.getElementById('toggle-livraison');
    const livrFields   = document.getElementById('livraison-fields');
    const retraitField = document.getElementById('retrait-fields');
    const heureRetrait = document.getElementById('heure_retrait');
    const heureLivr    = document.getElementById('heure_livraison');

    const toLocal = function (d) {
        const pad = n => String(n).padStart(2, '0');
        return d.getFullYear() + '-' + pad(d.getMonth() + 1) + '-' + pad(d.getDate())
            + 'T' + pad(d.getHours()) + ':' + pad(d.getMinutes());
    };

    const setMinDatetime = function (input) {
        if (!input) return;
        const soon = new Date();
        soon.setMinutes(soon.getMinutes() + 30);
        const val = toLocal(soon);
        input.min = val;
        if (!input.value) input.value = val;
    };

    if (toggle && livrFields) {
        const update = () => {
            const livr = toggle.checked;
            livrFields.style.display = livr ? '' : 'none';
            if (retraitField) retraitField.style.display = livr ? 'none' : '';
            if (heureRetrait) heureRetrait.required = !livr;
            if (heureLivr)    heureLivr.required    = livr;
            setMinDatetime(livr ? heureLivr : heureRetrait);
        };
        toggle.addEventListener('change', update);
        update();
    } else {
        setMinDatetime(heureRetrait);
    }

    const form = document.querySelector('.panier-checkout form');
    if (form) {
        form.addEventListener('submit', function (e) {
            const livr = toggle && toggle.checked;
            const input = livr ? heureLivr : heureRetrait;
            if (!input || !input.value) return;
            const chosen = new Date(input.value);
            const now = new Date();
            if (isNaN(chosen.getTime()) || (chosen - now) < 30 * 60 * 1000) {
                e.preventDefault();
                input.setCustomValidity("La date doit être dans au moins 30 minutes.");
                input.reportValidity();
            } else {
                input.setCustomValidity('');
            }
        });
        [heureRetrait, heureLivr].forEach(function (inp) {
            if (inp) inp.addEventListener('input', function () { inp.setCustomValidity(''); });
        });
    }
})();

document.addEventListener('click', function (e) {
    const btn = e.target.closest('.js-retirer');
    if (!btn || btn.disabled) return;

    const { platId, boissonId, menuId } = btn.dataset;

    let url;
    let ligneEl = null;
    if (boissonId) {
        url = '/panier/retirer?boisson_id=' + boissonId + '&format=json';
        ligneEl = document.querySelector('.panier-ligne[data-boisson-id="' + boissonId + '"]');
    } else if (menuId) {
        url = '/panier/retirer?menu_id=' + menuId + '&format=json';
        ligneEl = btn.closest('.panier-ligne');
    } else if (platId) {
        url = '/panier/retirer?plat_id=' + platId + '&format=json';
        ligneEl = document.querySelector('.panier-ligne[data-plat-id="' + platId + '"]');
    } else {
        return;
    }

    btn.disabled = true;

    fetch(url, { credentials: 'same-origin' })
        .then(function (r) { return r.json(); })
        .then(function (data) {
            if (ligneEl) ligneEl.remove();

            const totalEl = document.getElementById('panier-total');
            if (totalEl) totalEl.textContent = data.total;

            updateBadge(data.nb);

            if (data.nb === 0) {
                const layout = document.querySelector('.panier-layout');
                if (layout) {
                    layout.innerHTML = '<div class="panier-empty"><p>Votre panier est vide.</p><a href="/carte" class="btn btn-primary">Voir la carte</a></div>';
                }
            }
        })
        .catch(function () { btn.disabled = false; });
});

function updateBadge(n) {
    const badge = document.getElementById('panier-badge');
    if (!badge) return;
    badge.textContent = n;
    badge.classList.toggle('panier-badge--hidden', n === 0);
}
