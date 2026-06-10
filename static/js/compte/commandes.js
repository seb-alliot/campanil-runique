const csrfToken = document.querySelector('meta[name="csrf-token"]').content;

document.addEventListener('click', async function (e) {
    const btn = e.target.closest('.js-supprimer-ligne');
    if (!btn || btn.disabled) return;
    const { numero, ligneId, titre } = btn.dataset;
    const ok = await runique_confirm(`Supprimer "${titre}" de la commande ?`, { okLabel: 'Supprimer', cancelLabel: 'Annuler' });
    if (!ok) return;
    btn.disabled = true;
    const body = new URLSearchParams();
    body.append('csrf_token', csrfToken);
    fetch(`/commande/${numero}/ligne/${ligneId}/supprimer`, {
        method: 'POST',
        credentials: 'same-origin',
        headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
        body: body.toString(),
    })
    .then(r => r.json())
    .then(data => {
        if (!data.ok) { btn.disabled = false; return; }
        const li = btn.closest('li');
        li?.remove();
        const card = btn.closest('.commande-card');
        if (card && data.nouveau_total) {
            const total = card.querySelector('.commande-total');
            if (total) total.textContent = `Total : ${data.nouveau_total} €`;
        }
    })
    .catch(() => { btn.disabled = false; });
});

document.addEventListener('click', async function (e) {
    const btn = e.target.closest('.js-annuler');
    if (!btn || btn.disabled) return;
    const ok = await runique_confirm('Annuler cette commande ?', { okLabel: 'Annuler la commande', cancelLabel: 'Garder' });
    if (!ok) return;
    const numero = btn.dataset.numero;
    btn.disabled = true;
    const body = new URLSearchParams();
    body.append('csrf_token', csrfToken);
    fetch('/commande/' + numero + '/annuler', {
        method: 'POST',
        credentials: 'same-origin',
        headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
        body: body.toString(),
    })
    .then(function (r) {
        if (r.redirected || r.ok) {
            sessionStorage.setItem('scrollY', window.scrollY);
            location.reload();
        } else {
            btn.disabled = false;
        }
    })
    .catch(function () { btn.disabled = false; });
});
