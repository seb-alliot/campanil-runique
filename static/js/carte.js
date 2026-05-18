const csrfToken = document.querySelector('meta[name="csrf-token"]')?.content || '';

function ajouterAuPanier(url, btn, labelEl) {
    const label = labelEl ? labelEl.textContent : '';
    if (btn) { btn.disabled = true; if (labelEl) labelEl.textContent = 'Ajouté !'; }

    fetch(url, {
        method: 'POST',
        credentials: 'same-origin',
        headers: { 'X-CSRFToken': csrfToken },
    })
        .then(function (r) { return r.json(); })
        .then(function (data) {
            updateBadge(data.nb);
            setTimeout(function () {
                if (labelEl) labelEl.textContent = label;
                if (btn) btn.disabled = false;
                const panel = btn && btn.closest('.plat-picker');
                if (panel) panel.hidden = true;
            }, 1200);
        })
        .catch(function () {
            if (labelEl) labelEl.textContent = label;
            if (btn) btn.disabled = false;
        });
}

document.addEventListener('change', function (e) {
    const radio = e.target.closest('.js-cours-radio');
    if (!radio) return;
    const coursGroupe = radio.closest('.menu-cours-groupe');
    if (!coursGroupe) return;
    const platId = radio.value;
    coursGroupe.querySelectorAll('.menu-plat-picker').forEach(function (p) { p.hidden = true; });
    coursGroupe.querySelectorAll('.js-modifier-plat-menu').forEach(function (b) {
        b.hidden = b.dataset.platId !== platId;
    });
    coursGroupe.classList.remove('menu-cours-groupe--erreur');
});

document.addEventListener('click', function (e) {
    const confirmer = e.target.closest('.js-menu-confirmer');
    if (confirmer && !confirmer.disabled) {
        const menuId = confirmer.dataset.menuId;
        const picker = confirmer.closest('.menu-picker');
        const coursGroupes = picker.querySelectorAll('.menu-cours-groupe');
        let params = 'menu_id=' + menuId + '&quantite=1&format=json';
        let valid = true;

        coursGroupes.forEach(function (groupe) {
            const cours = groupe.dataset.cours;
            const radio = groupe.querySelector('input[type="radio"]:checked');
            if (!radio) {
                groupe.classList.add('menu-cours-groupe--erreur');
                valid = false;
                return;
            }
            const platId = radio.value;
            params += '&' + cours + '_plat_id=' + platId;

            const platPicker = groupe.querySelector('.menu-plat-picker[data-plat-id="' + platId + '"]');
            if (platPicker) {
                const cuissonR = platPicker.querySelector('input[name*="_cuiss_"]:checked');
                if (cuissonR) params += '&' + cours + '_cuisson=' + cuissonR.value;
                const garns = [...platPicker.querySelectorAll('input.js-garniture:checked')].map(function (i) { return i.value; });
                if (garns.length) params += '&' + cours + '_garniture_ids=' + garns.join(',');
                const legumes = platPicker.querySelector('.js-avec-legumes');
                if (legumes && legumes.checked) params += '&' + cours + '_avec_legumes=1';
                const sel = platPicker.querySelector('.js-sans-sel');
                if (sel && sel.checked) params += '&' + cours + '_sans_sel=1';
            }
        });

        if (!valid) return;

        ajouterAuPanier('/panier/ajouter?' + params, confirmer, confirmer);
        return;
    }

    const modBtn = e.target.closest('.js-modifier');
    if (modBtn) {
        const panel = document.querySelector('.plat-picker[data-plat-id="' + modBtn.dataset.platId + '"]');
        if (panel) {
            panel.hidden = !panel.hidden;
            if (!panel.hidden) closePhotoModalIfOpen();
        }
        return;
    }

    const modMenuBtn = e.target.closest('.js-modifier-plat-menu');
    if (modMenuBtn) {
        const { platId, cours } = modMenuBtn.dataset;
        const coursGroupe = modMenuBtn.closest('.menu-cours-groupe');
        if (coursGroupe) {
            const panel = coursGroupe.querySelector('.menu-plat-picker[data-plat-id="' + platId + '"][data-cours="' + cours + '"]');
            if (panel) {
                panel.hidden = !panel.hidden;
                if (!panel.hidden) closePhotoModalIfOpen();
            }
        }
        return;
    }

    const btn = e.target.closest('.js-ajouter');
    if (!btn || btn.disabled) return;

    const {platId, boissonId, menuId, supplementId} = btn.dataset;

    let url;
    if (supplementId) {
        url = '/panier/ajouter?supplement_id=' + supplementId + '&quantite=1&format=json';
    } else if (menuId) {
        url = '/panier/ajouter?menu_id=' + menuId + '&quantite=1&format=json';
    } else if (boissonId) {
        url = '/panier/ajouter?boisson_id=' + boissonId + '&quantite=1&format=json';
    } else if (platId) {
        const picker = btn.closest('.plat-picker');
        let cuisson = '', garnitureId = '', avecLegumes = '0', sansSel = '0';
        if (picker) {
            const cuissonR = picker.querySelector('input[name^="cuiss_"]:checked');
            if (cuissonR) cuisson = cuissonR.value;
            const garnChecked = [...picker.querySelectorAll('input.js-garniture:checked')].map(i => i.value);
            if (garnChecked.length) garnitureId = garnChecked.join(',');
            const legumesC = picker.querySelector('.js-avec-legumes');
            if (legumesC && legumesC.checked) avecLegumes = '1';
            const selC = picker.querySelector('.js-sans-sel');
            if (selC && selC.checked) sansSel = '1';
        }
        const noteInput = document.querySelector('.plat-note-input[data-plat-id="' + platId + '"]');
        const note = noteInput ? noteInput.value.trim() : '';
        url = '/panier/ajouter?plat_id=' + platId + '&quantite=1&format=json'
            + (cuisson ? '&cuisson=' + cuisson : '')
            + (garnitureId ? '&garniture_ids=' + garnitureId : '')
            + (avecLegumes === '1' ? '&avec_legumes=1' : '')
            + (sansSel === '1' ? '&sans_sel=1' : '')
            + (note ? '&note=' + encodeURIComponent(note) : '');
    } else {
        return;
    }

    ajouterAuPanier(url, btn, btn);
});

function closePhotoModalIfOpen() {
    const modal = document.getElementById('photoModal');
    if (modal && !modal.hidden) {
        modal.hidden = true;
        document.body.style.overflow = '';
    }
}

function updateBadge(n) {
    const badge = document.getElementById('panier-badge');
    if (!badge) return;
    badge.textContent = n;
    badge.classList.toggle('panier-badge--hidden', n === 0);
}
