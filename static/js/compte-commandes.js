const csrfToken = document.querySelector('meta[name="csrf-token"]').content;

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
