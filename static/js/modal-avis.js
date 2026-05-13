const modalAvis = document.getElementById('modal-supprimer-avis');
const formAvis = document.getElementById('form-supprimer-avis');

document.addEventListener('click', function (e) {
    const btn = e.target.closest('.js-supprimer-avis');
    if (!btn) return;
    formAvis.action = '/compte/avis/' + btn.dataset.commande + '/supprimer';
    modalAvis.showModal();
});

document.getElementById('btn-annuler-avis').addEventListener('click', () => modalAvis.close());
