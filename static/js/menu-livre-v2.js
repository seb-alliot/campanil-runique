(function () {
  'use strict';

  const couverture  = document.getElementById('livreCouverture');
  const livreOuvert = document.getElementById('livreOuvert');
  const btnPrev     = document.getElementById('livrePrev');
  const btnNext     = document.getElementById('livreNext');
  const btnFermer   = document.getElementById('livreFermer');
  const tabs        = document.querySelectorAll('.livre-tab');
  const siteNav     = document.querySelector('nav.nav');
  const sections    = document.querySelectorAll('.spr-section');
  const sprGauche   = document.getElementById('sprGauche');

  const DECORATIF = `<div class="livre-page-int">
    <div class="livre-page-int-ornement">✦</div>
    <p class="livre-page-int-nom">U Campanile</p>
    <p class="livre-page-int-lieu">Corte · Corse</p>
    <div class="livre-page-int-ornement">✦</div>
  </div>`;

  const ANIM_DURATION = 520;

  let current  = 0;
  let flipping = false;

  function updateLeftPage(idx) {
    sprGauche.innerHTML = '';
    if (idx < 0) {
      sprGauche.innerHTML = DECORATIF;
    } else {
      const clone = sections[idx].cloneNode(true);
      clone.removeAttribute('hidden');
      clone.className = 'spr-left-content';
      sprGauche.appendChild(clone);
    }
  }

  function updateNav(idx) {
    current = idx;
    tabs.forEach((t, i) => t.classList.toggle('active', i === current));
    btnPrev.disabled = current === 0;
    btnNext.disabled = current === sections.length - 1;
  }

  function showSection(idx) {
    sections.forEach((s, i) => {
      s.hidden = i !== idx;
      if (i === idx) s.scrollTop = 0;
    });
  }

  function flipTo(target) {
    if (flipping || target === current || target < 0 || target >= sections.length) return;
    flipping = true;

    const forward  = target > current;
    const prev     = current;
    const droite   = document.querySelector('.spr-droite');
    const leaveCls = forward ? 'spr-section--leaving'     : 'spr-section--leaving-bwd';
    const enterCls = forward ? 'spr-section--entering'    : 'spr-section--entering-bwd';

    /* Balayage lumineux — droite pour forward, gauche pour backward */
    const sweep = document.createElement('div');
    sweep.className = 'spr-sweep ' + (forward ? 'fwd' : 'bwd');
    (forward ? droite : sprGauche).appendChild(sweep);

    /* Section courante part */
    const leaving = sections[prev];
    leaving.classList.add(leaveCls);

    /* Section cible arrive (visible mais sous la courante le temps qu'elle parte) */
    const entering = sections[target];
    entering.hidden = false;
    entering.scrollTop = 0;
    entering.classList.add(enterCls);

    setTimeout(() => {
      /* Nettoyage départ */
      leaving.classList.remove(leaveCls);
      leaving.hidden = true;

      /* Nettoyage arrivée */
      entering.classList.remove(enterCls);

      /* Mise à jour page gauche */
      updateLeftPage(forward ? prev : target - 1);

      sweep.remove();
      updateNav(target);
      flipping = false;
    }, ANIM_DURATION + 20);
  }

  function openBook() {
    if (siteNav) siteNav.classList.add('nav--cachee');
    couverture.hidden = true;
    livreOuvert.hidden = false;
    showSection(0);
    updateNav(0);
    sprGauche.innerHTML = DECORATIF;
  }

  function closeBook() {
    couverture.hidden = false;
    livreOuvert.hidden = true;
    if (siteNav) siteNav.classList.remove('nav--cachee');
    current = 0;
    sprGauche.innerHTML = DECORATIF;
  }

  couverture.addEventListener('click', openBook);
  couverture.addEventListener('keydown', e => {
    if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); openBook(); }
  });

  btnPrev.addEventListener('click', () => flipTo(current - 1));
  btnNext.addEventListener('click', () => flipTo(current + 1));
  btnFermer.addEventListener('click', closeBook);
  tabs.forEach(tab => {
    tab.addEventListener('click', () => flipTo(Number(tab.dataset.section)));
  });

})();
