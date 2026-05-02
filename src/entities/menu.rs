use runique::prelude::*;

model! {
    Menu,
    table: "menus",
    pk: id => Pk,
    {
        titre:             text        [required, max_length: 255],
        description:       textarea    [required],
        prix_par_personne: decimal [required],
        nb_personnes_min:  int         [required],
        theme_id:          int         [required, fk(themes.id, restrict)],
        regime_id:         int         [required, fk(regimes.id, restrict)],
        conditions:        textarea,
        stock:             int         [required, default: 0],
        actif:             bool        [required, default: true],
        created_at:        datetime    [auto_now],
    },
    relations: {
        belongs_to: Theme via theme_id,
        belongs_to: Regime via regime_id,
        has_many: MenuImage,
        many_to_many: Plat through MenuPlat via menu_id,
    },
    meta: {
        ordering: [-created_at],
        verbose_name: "Menu traiteur",
        verbose_name_plural: "Menus traiteur",
    }
}
