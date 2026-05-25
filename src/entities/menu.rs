use runique::prelude::*;

model! {
    Menu,
    table: "menus",
    pk: id => Pk,
    enums: {
        TypeMenu: [
            MenuResto   = ("menu_resto",   "Menu restaurant"),
            MenuEnfant  = ("menu_enfant",  "Menu enfant"),
            FormuleJour = ("formule_jour", "Formule du jour"),
        ],
    },
    {
        type_menu:     choice  [enum(TypeMenu), required, default: "menu_resto"],
        nom:           text    [required, max_length: 255],
        description:   textarea,
        image:         image   [upload_to: "menus/"],
        prix:          decimal [required],
        ordre:         int     [default: 0],
        entree_libre:  text    [max_length: 500],
        plat_libre:    text    [max_length: 500],
        dessert_libre: text    [max_length: 500],
    },
    meta: {
        ordering: [type_menu, ordre, nom],
        verbose_name: "Menu",
        verbose_name_plural: "Menus",
    }
}
