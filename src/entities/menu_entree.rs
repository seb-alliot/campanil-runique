use runique::prelude::*;

model! {
    MenuEntree,
    table: "menu_entrees",
    pk: id => Pk,
    {
        menu_id:   int [required, fk(menus.id, cascade)],
        entree_id: int [required, fk(entrees.id, cascade)],
    },
}
