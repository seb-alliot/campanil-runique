document.getElementById('select-statut').addEventListener('change', function() {
    document.getElementById('annulation-fields').style.display =
        this.value === 'annule' ? 'block' : 'none';
});
