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
        ouverture_midi:   time,
        fermeture_midi:   time,
        ouverture_soir:   time,
        fermeture_soir:   time,
        ferme:            bool   [required, default: false],
        note:             text,
    }
}
