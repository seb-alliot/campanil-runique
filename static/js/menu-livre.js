(function () {
  'use strict';

  const couverture  = document.getElementById('livreCouverture');
  const livreOuvert = document.getElementById('livreOuvert');
  const btnPrev     = document.getElementById('livrePrev');
  const btnNext     = document.getElementById('livreNext');
  const btnFermer   = document.getElementById('livreFermer');
  const tabs        = document.querySelectorAll('.livre-tab');
  const siteNav     = document.querySelector('nav.nav');
  const sprGauche   = document.getElementById('sprGauche');
  const sprDroite   = document.querySelector('.spr-droite');

  const sections          = Array.from(document.querySelectorAll('.spr-section'));
  const btnBoissonsParent = document.getElementById('livreBoissonsParent');
  const boissonsRow       = document.getElementById('livreBoissonsRow');
  const BOISSONS_START    = 5;

  const DECORATIF = `<div class="livre-page-int">
    <div class="livre-page-int-ornement">✦</div>
    <p class="livre-page-int-nom">U Campanile</p>
    <p class="livre-page-int-lieu">Corte · Corse</p>
    <div class="livre-page-int-ornement">✦</div>
  </div>`;

  const ANIM_DURATION = 800;

  let current  = 0;
  let flipping = false;

  /* Remet toutes les sections dans sprDroite (état neutre) */
  function resetSections() {
    sections.forEach(s => {
      s.hidden = true;
      s.classList.remove('spr-left-mode');
      if (s.parentNode !== sprDroite) sprDroite.appendChild(s);
    });
  }

  /* Efface sprGauche sans toucher aux sections qui y seraient encore */
  function clearGauche() {
    sections.forEach(s => {
      if (s.parentNode === sprGauche) sprDroite.appendChild(s);
    });
    Array.from(sprGauche.childNodes).forEach(node => node.remove());
  }

  /* Place les sections aux bons endroits après un flip */
  function renderPages(rightIdx) {
    const leftIdx = rightIdx - 1;

    resetSections();
    clearGauche();

    if (leftIdx < 0) {
      sprGauche.innerHTML = DECORATIF;
    } else {
      const leftSection = sections[leftIdx];
      leftSection.hidden = false;
      sprGauche.appendChild(leftSection);
    }

    sections[rightIdx].hidden = false;
    sections[rightIdx].scrollTop = 0;
  }

  function updateNav(idx) {
    current = idx;
    const isBoisson = current >= BOISSONS_START;
    tabs.forEach((t, i) => t.classList.toggle('active', i === current));
    if (btnBoissonsParent) btnBoissonsParent.classList.toggle('active', isBoisson);
    if (boissonsRow) boissonsRow.classList.toggle('visible', isBoisson);
    btnPrev.disabled = current === 0;
    btnNext.disabled = current === sections.length - 1;
  }

  function flipTo(target) {
    if (flipping || target === current || target < 0 || target >= sections.length) return;
    flipping = true;

    const forward  = target > current;
    const leaveCls = forward ? 'spr-section--leaving'  : 'spr-section--leaving-bwd';
    const enterCls = forward ? 'spr-section--entering' : 'spr-section--entering-bwd';

    /* Pendant l'animation les deux sections doivent être dans sprDroite */
    const leaving  = sections[current];
    const entering = sections[target];

    if (leaving.parentNode !== sprDroite) sprDroite.appendChild(leaving);
    leaving.hidden = false;
    leaving.classList.remove('spr-left-mode');

    if (forward) {
      /* Avant : entering vient de sprGauche, on l'amène dans sprDroite pour l'animation */
      if (entering.parentNode !== sprDroite) sprDroite.appendChild(entering);
      entering.hidden = false;
      entering.scrollTop = 0;
      entering.classList.remove('spr-left-mode');
      entering.classList.add(enterCls);
    } else {
      /* Arrière : entering reste dans sprGauche — il est déjà visible, pas besoin de le déplacer */
      entering.hidden = false;
      entering.scrollTop = 0;
    }

    /* Balayage de page */
    const sweep = document.createElement('div');
    sweep.className = 'spr-sweep ' + (forward ? 'fwd' : 'bwd');
    (forward ? sprDroite : sprGauche).appendChild(sweep);

    leaving.classList.add(leaveCls);

    setTimeout(() => {
      leaving.classList.remove(leaveCls);
      if (forward) entering.classList.remove(enterCls);
      sweep.remove();
      renderPages(target);
      updateNav(target);
      flipping = false;
    }, ANIM_DURATION + 20);
  }

  function openBook() {
    if (siteNav) siteNav.classList.add('nav--cachee');
    couverture.hidden = true;
    livreOuvert.hidden = false;
    renderPages(0);
    updateNav(0);
  }

  function closeBook() {
    couverture.hidden = false;
    livreOuvert.hidden = true;
    if (siteNav) siteNav.classList.remove('nav--cachee');
    current = 0;
    clearGauche();
    resetSections();
    sprGauche.innerHTML = DECORATIF;
  }

  window.addEventListener('pageshow', e => {
    if (e.persisted && !livreOuvert.hidden) {
      renderPages(current);
    }
  });

  couverture.addEventListener('click', openBook);
  couverture.addEventListener('keydown', e => {
    if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); openBook(); }
  });

  if (btnBoissonsParent) {
    btnBoissonsParent.addEventListener('click', () => flipTo(BOISSONS_START));
  }

  btnPrev.addEventListener('click', () => flipTo(current - 1));
  btnNext.addEventListener('click', () => flipTo(current + 1));
  btnFermer.addEventListener('click', closeBook);
  tabs.forEach(tab => {
    tab.addEventListener('click', () => flipTo(Number(tab.dataset.section)));
  });

})();