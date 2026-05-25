use runique::prelude::*;

model! {
    MenuTraiteur,
    table: "menus_traiteur",
    pk: id => Pk,
    enums: {
        ThemeMenu: [
            Mariage      = ("mariage",      "Mariage"),
            Bapteme      = ("bapteme",       "Baptême"),
            Anniversaire = ("anniversaire",  "Anniversaire"),
            Autre        = ("autre",         "Autre"),
        ],
        RegimeMenu: [
            Standard   = ("standard",    "Standard"),
            Vegetarien = ("vegetarien",  "Végétarien"),
            SansGluten = ("sans_gluten", "Sans gluten"),
            Halal      = ("halal",       "Halal"),
            Casher     = ("casher",      "Casher"),
        ],
    },
    {
        titre:             text     [required, max_length: 255],
        description:       textarea [required],
        prix_par_personne: decimal  [required],
        nb_personnes_min:  int      [required],
        theme:             choice   [enum(ThemeMenu), required, default: "autre"],
        regime:            choice   [enum(RegimeMenu), required, default: "standard"],
        conditions:        textarea,
        stock:             int      [required, default: 0],
        actif:             bool     [required, default: true],
        created_at:        datetime [auto_now],
    },
    meta: {
        ordering: [-created_at],
        verbose_name: "Menu traiteur",
        verbose_name_plural: "Menus traiteur",
    }
}
