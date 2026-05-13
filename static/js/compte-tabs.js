const savedY = sessionStorage.getItem('scrollY');
if (savedY) { window.scrollTo(0, parseInt(savedY)); sessionStorage.removeItem('scrollY'); }

document.querySelectorAll('.compte-tab').forEach(btn => {
    btn.addEventListener('click', () => {
        const tab = btn.dataset.tab;
        document.querySelectorAll('.compte-tab').forEach(b => b.classList.remove('active'));
        btn.classList.add('active');
        document.getElementById('tab-commandes').hidden = (tab !== 'commandes');
        document.getElementById('tab-profil').hidden = (tab !== 'profil');
        const url = new URL(window.location);
        url.searchParams.set('tab', tab);
        history.replaceState(null, '', url);
    });
});

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
