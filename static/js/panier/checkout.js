(function () {
    const adresseEl = document.getElementById('adresse_livraison');
    const cpEl      = document.getElementById('cp_livraison');
    const villeEl   = document.getElementById('ville_livraison');
    const labelEl   = document.getElementById('prix-livraison-label');
    const infoEl    = document.getElementById('livraison-prix-info');
    if (!adresseEl) return;

    let timer = null;

    function updatePrix() {
        const adresse = adresseEl.value.trim();
        const cp      = cpEl.value.trim();
        const ville   = villeEl.value.trim();
        if (!adresse || !ville) { infoEl.textContent = ''; return; }

        infoEl.textContent = 'Calcul en cours…';
        const params = new URLSearchParams({ adresse, cp, ville });
        const hiddenEl = document.getElementById('prix_livraison_hidden');
        fetch('/panier/livraison-prix?' + params)
            .then(r => r.json())
            .then(data => {
                if (data.prix) {
                    labelEl.textContent = data.prix;
                    infoEl.textContent  = 'Frais de livraison estimés : ' + data.prix + ' € (selon distance)';
                    if (hiddenEl) hiddenEl.value = data.prix;
                } else {
                    infoEl.textContent = data.erreur || 'Adresse introuvable';
                }
            })
            .catch(() => { infoEl.textContent = ''; });
    }

    [adresseEl, cpEl, villeEl].forEach(el => {
        el.addEventListener('input', () => {
            clearTimeout(timer);
            timer = setTimeout(updatePrix, 700);
        });
    });

    if (adresseEl.value.trim() && villeEl.value.trim()) {
        updatePrix();
    }
})();

(function () {
    const pad = n => String(n).padStart(2, '0');
    const toLocal = d => `${d.getFullYear()}-${pad(d.getMonth()+1)}-${pad(d.getDate())}T${pad(d.getHours())}:${pad(d.getMinutes())}`;
    const now = new Date();
    const min = toLocal(now);
    const def = toLocal(new Date(now.getTime() + 60 * 60 * 1000));
    const el = document.getElementById('heure_retrait');
    if (el) { el.min = min; el.value = def; }
})();
