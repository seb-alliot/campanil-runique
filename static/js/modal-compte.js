const modalCompte = document.getElementById('modal-supprimer');
document.getElementById('btn-supprimer-compte').addEventListener('click', () => modalCompte.showModal());
document.getElementById('btn-annuler-suppression').addEventListener('click', () => modalCompte.close());
