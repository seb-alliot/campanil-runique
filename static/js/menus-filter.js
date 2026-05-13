(function () {
    const cards = Array.from(document.querySelectorAll('.menu-card'));
    const count = document.getElementById('menus-count');
    const empty = document.querySelector('.menus-empty');

    const fPersonnes = document.getElementById('f-personnes');
    const fPrixMin   = document.getElementById('f-prix-min');
    const fPrixMax   = document.getElementById('f-prix-max');
    const fTheme     = document.getElementById('f-theme');
    const fRegime    = document.getElementById('f-regime');
    const fReset     = document.getElementById('f-reset');

    const applyFilters = () => {
        const personnes = fPersonnes ? parseInt(fPersonnes.value) || null : null;
        const prixMin   = fPrixMin   ? parseFloat(fPrixMin.value) || null : null;
        const prixMax   = fPrixMax   ? parseFloat(fPrixMax.value) || null : null;
        const themeId   = fTheme  ? fTheme.value  : '';
        const regimeId  = fRegime ? fRegime.value : '';

        let visible = 0;
        cards.forEach((card) => {
            const { prix, theme, regime, personnes: pers } = card.dataset;

            const ok =
                (personnes === null || parseInt(pers)    <= personnes) &&
                (prixMin   === null || parseFloat(prix)  >= prixMin)   &&
                (prixMax   === null || parseFloat(prix)  <= prixMax)   &&
                (themeId   === ''   || theme  === themeId)             &&
                (regimeId  === ''   || regime === regimeId);

            card.style.display = ok ? '' : 'none';
            if (ok) visible++;
        });

        if (count) {
            count.textContent = visible + ' menu' + (visible > 1 ? 's' : '') + ' disponible' + (visible > 1 ? 's' : '');
        }
        if (empty) {
            empty.style.display = visible === 0 ? '' : 'none';
        }
    };

    let debounceTimer;
    const debounced = () => {
        clearTimeout(debounceTimer);
        debounceTimer = setTimeout(applyFilters, 300);
    };

    [fPersonnes, fPrixMin, fPrixMax].forEach((el) => {
        if (el) el.addEventListener('input', debounced);
    });
    [fTheme, fRegime].forEach((el) => {
        if (el) el.addEventListener('change', applyFilters);
    });

    if (fReset) {
        fReset.addEventListener('click', () => {
            if (fPersonnes) fPersonnes.value = '';
            if (fPrixMin)   fPrixMin.value   = '';
            if (fPrixMax)   fPrixMax.value   = '';
            if (fTheme)     fTheme.value     = '';
            if (fRegime)    fRegime.value    = '';
            applyFilters();
        });
    }
})();
