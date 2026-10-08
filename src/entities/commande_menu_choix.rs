use runique::prelude::*;

model! {
    CommandeMenuChoix,
    table: "commande_menu_choix",
    pk: id => Pk,
    {
        commande_ligne_id: int [required],
        cours:             text [required, max_length: 20],
        plat_id:           int  [nullable],
        entree_id:         int  [nullable],
        dessert_id:        int  [nullable],
    },
    relations: {
        belongs_to: CommandeLigne via commande_ligne_id [cascade],
        belongs_to: Plat via plat_id [restrict],
        belongs_to: Entree via entree_id [restrict],
        belongs_to: Dessert via dessert_id [restrict],
    },
    meta: {
        verbose_name: "Choix de menu",
        verbose_name_plural: "Choix de menus",
    }
}
