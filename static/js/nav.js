(function () {
  'use strict';

  const burger = document.getElementById('navBurger');
  const menu   = document.getElementById('navMenu');
  if (!burger || !menu) return;

  function openMenu() {
    menu.hidden = false;
    burger.setAttribute('aria-expanded', 'true');
    burger.classList.add('nav-burger--open');
  }

  function closeMenu() {
    menu.hidden = true;
    burger.setAttribute('aria-expanded', 'false');
    burger.classList.remove('nav-burger--open');
  }

  burger.addEventListener('click', () => {
    if (menu.hidden) openMenu(); else closeMenu();
  });

  document.addEventListener('keydown', e => {
    if (e.key === 'Escape' && !menu.hidden) closeMenu();
  });

  document.addEventListener('click', e => {
    if (!menu.hidden && !menu.contains(e.target) && !burger.contains(e.target)) {
      closeMenu();
    }
  });

  menu.addEventListener('click', e => {
    if (e.target.tagName === 'A') closeMenu();
  });
})();
