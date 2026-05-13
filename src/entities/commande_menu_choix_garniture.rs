use runique::prelude::*;

model! {
    CommandeMenuChoixGarniture,
    table: "commande_menu_choix_garnitures",
    pk: id => Pk,
    {
        commande_menu_choix_id: int [required, fk(commande_menu_choix.id, cascade)],
        garniture_id:           int [required, fk(garnitures.id, restrict)],
    },
    meta: {
        verbose_name: "Garniture choix menu",
        verbose_name_plural: "Garnitures choix menus",
    }
}
