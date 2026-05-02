use runique::prelude::*;

model! {
    CommandeStatut,
    table: "commande_statuts",
    pk: id => Pk,
    {
        commande_id: int      [required, fk(commandes.id, cascade)],
        statut:      text     [required, max_length: 50],
        note:        textarea,
        created_at:  datetime [auto_now],
    },
    relations: {
        belongs_to: Commande via commande_id,
    },
    meta: {
        ordering: [created_at],
    }
}
