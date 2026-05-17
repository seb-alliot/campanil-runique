use runique::prelude::*;

model! {
    MenuResto,
    table: "menu_resto",
    pk: id => Pk,
    {
        nom:         text    [required, max_length: 255],
        prix:        decimal [required],
        description: textarea,
        disponible:  bool    [required, default: true],
        ordre:       int     [default: 0],
        dessert:     text    [max_length: 255],
    },
    relations: {
        many_to_many: Plat through MenuRestoPlat via menu_id,
    },
    meta: {
        ordering: [ordre, nom],
        verbose_name: "Menu restaurant",
        verbose_name_plural: "Menus restaurant",
    }
}
