use runique::prelude::*;

model! {
    CommandeMenuChoix,
    table: "commande_menu_choix",
    pk: id => Pk,
    {
        commande_plat_id: int  [required, fk(commande_plats.id, cascade)],
        cours:            text [required, max_length: 10],
        plat_id:          int  [required, fk(plats.id, restrict)],
        cuisson:          text [max_length: 20],
        avec_legumes:     bool [default: false],
        sans_sel:         bool [default: false],
        note:             text [max_length: 500],
    },
    meta: {
        verbose_name: "Choix menu commandé",
        verbose_name_plural: "Choix menus commandés",
    }
}
