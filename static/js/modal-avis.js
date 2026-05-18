const modalAvis = document.getElementById('modal-supprimer-avis');
const formAvis = document.getElementById('form-supprimer-avis');

document.addEventListener('click', function (e) {
    const btn = e.target.closest('.js-supprimer-avis');
    if (!btn) return;
    formAvis.action = '/compte/avis/' + btn.dataset.commande + '/supprimer';
    modalAvis.showModal();
});

document.getElementById('btn-annuler-avis').addEventListener('click', () => modalAvis.close());

/* ── Soumission avis plat (AJAX) ── */
document.addEventListener('submit', function (e) {
    const form = e.target.closest('.plat-avis-card .avis-form');
    if (!form) return;
    e.preventDefault();

    const btn = form.querySelector('button[type="submit"]');
    if (btn) btn.disabled = true;

    const body = new URLSearchParams(new FormData(form));

    fetch(form.action, {
        method: 'POST',
        credentials: 'same-origin',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
            'X-CSRF-Token': document.querySelector('meta[name="csrf-token"]')?.content || '',
        },
        body: body.toString(),
    })
        .then(r => r.json())
        .then(data => {
            if (data.ok) {
                const card = form.closest('.plat-avis-body');
                const platId = form.action.split('/').pop();
                const details = form.closest('details');
                if (details) details.remove();
                if (card) {
                    const badge = document.createElement('span');
                    badge.className = 'avis-badge avis-attente';
                    badge.textContent = 'En attente de modération — ' + data.note + '★';
                    card.appendChild(badge);

                    const btnSuppr = document.createElement('button');
                    btnSuppr.type = 'button';
                    btnSuppr.className = 'btn btn-sm btn-danger js-supprimer-avis-plat avis-plat-suppr-form';
                    btnSuppr.dataset.platId = platId;
                    btnSuppr.textContent = 'Supprimer mon avis';
                    card.appendChild(btnSuppr);
                }
            } else {
                if (btn) btn.disabled = false;
            }
        })
        .catch(() => { if (btn) btn.disabled = false; });
});

/* ── Suppression avis plat (AJAX) ── */
const modalAvisPlat   = document.getElementById('modal-supprimer-avis-plat');
const btnConfirmerPlat = document.getElementById('btn-confirmer-avis-plat');
const btnAnnulerPlat   = document.getElementById('btn-annuler-avis-plat');
let pendingPlatId = null;
let pendingPlatBtn = null;

document.addEventListener('click', function (e) {
    const btn = e.target.closest('.js-supprimer-avis-plat');
    if (!btn) return;
    pendingPlatId = btn.dataset.platId;
    pendingPlatBtn = btn;
    modalAvisPlat.showModal();
});

btnAnnulerPlat.addEventListener('click', () => { modalAvisPlat.close(); pendingPlatId = null; pendingPlatBtn = null; });

btnConfirmerPlat.addEventListener('click', function () {
    if (!pendingPlatId) return;
    btnConfirmerPlat.disabled = true;

    fetch('/compte/avis-plat/' + pendingPlatId + '/supprimer', {
        method: 'POST',
        credentials: 'same-origin',
        headers: { 'X-CSRF-Token': document.querySelector('meta[name="csrf-token"]')?.content || '' },
    })
        .then(r => r.json())
        .then(data => {
            modalAvisPlat.close();
            if (data.ok && pendingPlatBtn) {
                const body = pendingPlatBtn.closest('.plat-avis-body');
                if (body) {
                    body.querySelectorAll('.avis-badge, .js-supprimer-avis-plat').forEach(el => el.remove());
                    const msg = document.createElement('span');
                    msg.className = 'avis-badge avis-attente';
                    msg.textContent = 'Avis supprimé';
                    body.appendChild(msg);
                }
            }
            btnConfirmerPlat.disabled = false;
            pendingPlatId = null;
            pendingPlatBtn = null;
        })
        .catch(() => { btnConfirmerPlat.disabled = false; });
});
