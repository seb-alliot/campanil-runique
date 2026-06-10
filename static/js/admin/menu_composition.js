function updateBadge(badgeId, name) {
  document.getElementById(badgeId).textContent =
    document.querySelectorAll('input[name="' + name + '"]:checked').length;
}
document.addEventListener('DOMContentLoaded', function () {
  updateBadge('badge-entree', 'entree_ids');
  updateBadge('badge-plat',   'plat_ids');
  updateBadge('badge-dessert','dessert_ids');
  document.querySelectorAll('input[name="entree_ids"]').forEach(function(el) {
    el.addEventListener('change', function() { updateBadge('badge-entree', 'entree_ids'); });
  });
  document.querySelectorAll('input[name="plat_ids"]').forEach(function(el) {
    el.addEventListener('change', function() { updateBadge('badge-plat', 'plat_ids'); });
  });
  document.querySelectorAll('input[name="dessert_ids"]').forEach(function(el) {
    el.addEventListener('change', function() { updateBadge('badge-dessert', 'dessert_ids'); });
  });
});
