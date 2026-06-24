(function () {
    const cards = Array.from(document.querySelectorAll('.menu-card'));
    const count = document.getElementById('menus-count');
    const empty     = document.querySelector('.menus-empty');
    const noResults = document.getElementById('menus-no-results');

    const fPersonnes = document.getElementById('f-personnes');
    const fPrixMin   = document.getElementById('f-prix-min');
    const fPrixMax   = document.getElementById('f-prix-max');
    const fTheme     = document.getElementById('f-theme');
    const fRegime    = document.getElementById('f-regime');
    const fReset     = document.getElementById('f-reset');

    let trackTimer = null;
    const trackFilters = (nbPersonnes, prixMin, prixMax, themeVal, regimeVal) => {
        const params = new URLSearchParams();
        if (!isNaN(nbPersonnes)) params.set('nb_personnes', nbPersonnes);
        if (!isNaN(prixMin))     params.set('prix_min', prixMin);
        if (!isNaN(prixMax))     params.set('prix_max', prixMax);
        if (themeVal)            params.set('theme', themeVal);
        if (regimeVal)           params.set('regime', regimeVal);
        if ([...params].length === 0) return;

        clearTimeout(trackTimer);
        trackTimer = setTimeout(() => {
            fetch('/menus/track?' + params.toString(), { method: 'GET' }).catch(() => {});
        }, 800);
    };

    const applyFilters = () => {
        const nbPersonnes = fPersonnes ? parseInt(fPersonnes.value, 10) : NaN;
        const prixMin     = fPrixMin ? parseFloat(fPrixMin.value) : NaN;
        const prixMax     = fPrixMax ? parseFloat(fPrixMax.value) : NaN;
        const themeVal    = fTheme ? fTheme.value : '';
        const regimeVal   = fRegime ? fRegime.value : '';

        trackFilters(nbPersonnes, prixMin, prixMax, themeVal, regimeVal);

        let visible = 0;
        cards.forEach((card) => {
            const cardPrix      = parseFloat(card.dataset.prix);
            const cardPersonnes = parseInt(card.dataset.personnes, 10);

            const okPersonnes = isNaN(nbPersonnes) || cardPersonnes <= nbPersonnes;
            const okPrixMin   = isNaN(prixMin) || cardPrix >= prixMin;
            const okPrixMax   = isNaN(prixMax) || cardPrix <= prixMax;
            const okTheme     = themeVal === '' || card.dataset.theme === themeVal;
            const okRegime    = regimeVal === '' || card.dataset.regime === regimeVal;

            const ok = okPersonnes && okPrixMin && okPrixMax && okTheme && okRegime;
            card.style.display = ok ? '' : 'none';
            if (ok) visible++;
        });

        if (count) {
            count.style.display = visible === 0 ? 'none' : '';
            count.textContent = visible + ' menu' + (visible > 1 ? 's' : '') + ' disponible' + (visible > 1 ? 's' : '');
        }
        if (noResults) {
            noResults.style.display = visible === 0 ? 'block' : 'none';
        }
    };

    [fPersonnes, fPrixMin, fPrixMax, fTheme, fRegime].forEach((el) => {
        if (el) el.addEventListener('input', applyFilters);
    });

    if (fReset) {
        fReset.addEventListener('click', () => {
            if (fPersonnes) fPersonnes.value = '';
            if (fPrixMin)   fPrixMin.value = '';
            if (fPrixMax)   fPrixMax.value = '';
            if (fTheme)     fTheme.value = '';
            if (fRegime)    fRegime.value = '';
            applyFilters();
        });
    }
})();
