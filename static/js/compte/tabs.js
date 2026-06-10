const savedY = sessionStorage.getItem('scrollY');
if (savedY) { window.scrollTo(0, parseInt(savedY)); sessionStorage.removeItem('scrollY'); }

const tabsWrapper = document.querySelector('.compte-tabs-wrapper');
if (tabsWrapper) {
    const btnPrev = document.createElement('button');
    btnPrev.className = 'compte-tabs-arrow compte-tabs-arrow--prev';
    btnPrev.setAttribute('aria-label', 'Précédent');
    btnPrev.textContent = '‹';
    const btnNext = document.createElement('button');
    btnNext.className = 'compte-tabs-arrow compte-tabs-arrow--next';
    btnNext.setAttribute('aria-label', 'Suivant');
    btnNext.textContent = '›';
    tabsWrapper.appendChild(btnPrev);
    tabsWrapper.appendChild(btnNext);

    const goTo = delta => {
        const tabs = [...document.querySelectorAll('.compte-tab')];
        const idx = tabs.findIndex(a => a.classList.contains('active'));
        const target = tabs[idx + delta];
        if (target) target.click();
    };

    btnPrev.addEventListener('click', () => goTo(-1));
    btnNext.addEventListener('click', () => goTo(1));
}

const updateArrowState = () => {
    const tabs = [...document.querySelectorAll('.compte-tab')];
    const idx = tabs.findIndex(a => a.classList.contains('active'));
    const prev = document.querySelector('.compte-tabs-arrow--prev');
    const next = document.querySelector('.compte-tabs-arrow--next');
    if (prev) prev.disabled = idx <= 0;
    if (next) next.disabled = idx >= tabs.length - 1;
};

function activateTab(hash) {
    const tab = (hash || '#tab-commandes').replace('#tab-', '');
    document.querySelectorAll('.compte-tab').forEach(a => {
        a.classList.toggle('active', a.getAttribute('href') === '#tab-' + tab);
    });
    document.querySelectorAll('.compte-section[id^="tab-"]').forEach(s => {
        s.classList.toggle('tab-active', s.id === 'tab-' + tab);
    });
    updateArrowState();
}

document.querySelectorAll('.compte-tab[href^="#tab-"]').forEach(a => {
    a.addEventListener('click', e => {
        e.preventDefault();
        const href = a.getAttribute('href');
        history.pushState(null, '', href);
        activateTab(href);
    });
});

window.addEventListener('popstate', () => activateTab(window.location.hash));

function getInitialTab() {
    if (window.location.hash.startsWith('#tab-')) return window.location.hash;
    const param = new URLSearchParams(window.location.search).get('tab');
    if (param) return '#tab-' + param;
    return '#tab-commandes';
}
activateTab(getInitialTab());

document.querySelectorAll('.local-time[data-utc]').forEach(el => {
    const iso = el.dataset.utc;
    if (!iso) return;
    const d = new Date(iso);
    if (isNaN(d)) return;
    el.textContent = d.toLocaleString('fr-FR', {
        timeZone: 'Europe/Paris',
        day: '2-digit', month: '2-digit', year: 'numeric',
        hour: '2-digit', minute: '2-digit'
    });
});
