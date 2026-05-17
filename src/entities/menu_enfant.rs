use runique::prelude::*;

model! {
    MenuEnfant,
    table: "menu_enfants",
    pk: id => Pk,
    {
        titre:       text    [required, max_length: 255],
        description: textarea,
        plat:        text    [required, max_length: 255],
        dessert:     text    [max_length: 255],
        prix:        decimal [required],
        image:       image   [upload_to: "menus/enfant/"],
        actif:       bool    [required, default: true],
    },
    relations: {
        many_to_many: Allergene through MenuEnfantAllergene via menu_enfant_id,
    },
    meta: {
        ordering: [titre],
        verbose_name: "Menu enfant",
        verbose_name_plural: "Menus enfant",
    }
}
