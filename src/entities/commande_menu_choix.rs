use runique::prelude::*;

model! {
    CommandeMenuChoix,
    table: "commande_menu_choix",
    pk: id => Pk,
    {
        commande_ligne_id: int [required, fk(commande_lignes.id, cascade)],
        cours:             text [required, max_length: 20],
        plat_id:           int  [required, fk(plats.id, restrict)],
    },
    meta: {
        verbose_name: "Choix de menu",
        verbose_name_plural: "Choix de menus",
    }
}
