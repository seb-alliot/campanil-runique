use runique::prelude::*;

model! {
    Horaire,
    table: "horaires",
    pk: id => Pk,
    enums: {
        Jour: [
            Lundi    = ("lundi",    "Lundi"),
            Mardi    = ("mardi",    "Mardi"),
            Mercredi = ("mercredi", "Mercredi"),
            Jeudi    = ("jeudi",    "Jeudi"),
            Vendredi = ("vendredi", "Vendredi"),
            Samedi   = ("samedi",   "Samedi"),
            Dimanche = ("dimanche", "Dimanche"),
        ],
    },
    {
        jour:             choice [enum(Jour), required, unique],
        ouverture_midi:   time [nullable],
        fermeture_midi:   time [nullable],
        ouverture_soir:   time [nullable],
        fermeture_soir:   time [nullable],
        ferme:            bool   [required, default: false],
        note:             text [nullable],
    }
}
