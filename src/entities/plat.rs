use runique::prelude::*;

model! {
    Plat,
    table: "plats",
    pk: id => Pk,
    enums: {
        TypePlat: [
            Specialite = ("specialite", "Spécialité de chez nous"),
            Viande     = ("viande",     "Viande"),
            Poisson    = ("poisson",    "Poisson"),
            Plat       = ("plat",       "Plat"),
        ],
        UsagePlat: [
            Carte   = ("carte",    "Carte uniquement"),
            Menu    = ("menu",     "Menu uniquement"),
            LesDeux = ("les_deux", "Carte et menu"),
        ],
    },
    {
        titre:        text    [required, max_length: 255],
        label:        text    [max_length: 80],
        type_plat:    choice  [enum(TypePlat), required],
        description:  textarea,
        image:        image   [upload_to: "plats/"],
        prix:         decimal [required],
        disponible:   bool    [required, default: true],
        est_viande:   bool    [required, default: false],
        usage:        choice  [enum(UsagePlat), required, default: "les_deux"],
        ordre:        int     [default: 0],
    },
    relations: {
        many_to_many: Allergene through PlatAllergene via plat_id,
        many_to_many: Garniture through PlatGarniture via plat_id,
    },
    meta: {
        ordering: [type_plat, ordre, titre],
        verbose_name: "Plat",
        verbose_name_plural: "Plats",
    }
}
