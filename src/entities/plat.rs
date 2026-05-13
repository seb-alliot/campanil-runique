use runique::prelude::*;

model! {
    Plat,
    table: "plats",
    pk: id => Pk,
    enums: {
        TypePlat: [
            Entree     = ("entree",     "Entrée"),
            Specialite = ("specialite", "Spécialité de chez nous"),
            Viande     = ("viande",     "Viande"),
            Poisson    = ("poisson",    "Poisson"),
            Plat       = ("plat",       "Plat"),
            Dessert    = ("dessert",    "Dessert"),
        ],
    },
    {
        titre:       text   [required, max_length: 255],
        label:       text   [max_length: 80],
        type_plat:   choice [enum(TypePlat), required],
        prix:        decimal [required],
        description: textarea,
        image:       image  [upload_to: "plats/"],
        disponible:              bool    [required, default: true],
        est_viande:              bool    [required, default: false],
        avec_legumes:            bool    [required, default: true],
        prix_supplement_legumes: decimal [default: 0],
    },
    relations: {
        many_to_many: Allergene through PlatAllergene via plat_id,
    },
    meta: {
        ordering: [type_plat, titre],
        verbose_name: "Plat",
        verbose_name_plural: "Plats",
    }
}
