use runique::prelude::*;

model! {
    MenuResto,
    table: "menu_resto",
    pk: id => Pk,
    {
        titre:       text     [required, max_length: 255],
        prix:        decimal  [required],
        description: textarea,
        disponible:  bool     [required, default: true],
    },
    meta: {
        ordering: [titre],
        verbose_name: "Menu restaurant",
        verbose_name_plural: "Menus restaurant",
    }
}
