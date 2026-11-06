use crate::backend::admin_relations::ecrire_liens_plat;
use crate::entities::{commande, garniture, plat, plat_garniture};
use runique::prelude::*;
use runique::runique_test::{TestFailure, runique_test};
use sea_orm::DbErr;

fn compte(rang: Rang) -> CurrentUser {
    CurrentUser {
        id: 999_999,
        username: "runique_test".to_string(),
        rang,
        groupes: vec![],
        droits: vec![],
    }
}

/// La porte refuse de modifier une commande sans le droit, et un rang admin sans
/// droit reçu n'a rien de plus qu'un membre. Le superutilisateur passe.
#[tokio::test]
async fn modifier_une_commande_demande_le_droit() -> Result<(), TestFailure> {
    runique_test::<ADb>(super::ENV, async |db| {
        for (rang, attendu) in [
            (Rang::Lambda, false),
            (Rang::Admin, false),
            (Rang::Superuser, true),
        ] {
            let user = compte(rang);
            let ctx = GateCtx { db, user: &user };
            let accepte = authorize_row::<Update, commande::Entity>(&ctx, 1)
                .await
                .is_ok();
            if accepte != attendu {
                return Err(DbErr::Custom(format!(
                    "rang {rang:?} : accepté = {accepte}, attendu {attendu}"
                )));
            }
        }
        Ok(())
    })
    .await
}

async fn nouvelle_garniture(db: &ADb, libelle: &str) -> Result<i32, DbErr> {
    Ok(garniture::ActiveModel {
        libelle: Set(libelle.to_string()),
        type_garniture: Set(garniture::TypeGarniture::Sauce),
        disponible: Set(true),
        ..Default::default()
    }
    .insert(db)
    .await?
    .id)
}

/// Une garniture gardée conserve son `est_defaut`, une décochée disparaît, une
/// nouvelle s'ajoute.
#[tokio::test]
async fn la_synchronisation_garde_ce_qui_reste_coche() -> Result<(), TestFailure> {
    runique_test::<ADb>(super::ENV, async |db| {
        let plat_id = plat::ActiveModel {
            titre: Set("runique_test plat".to_string()),
            type_plat: Set(plat::TypePlat::Plat),
            prix: Set(Decimal::new(100, 2)),
            disponible: Set(true),
            est_viande: Set(false),
            ordre: Set(0),
            ..Default::default()
        }
        .insert(db)
        .await?
        .id;
        let gardee = nouvelle_garniture(db, "runique_test gardée").await?;
        let retiree = nouvelle_garniture(db, "runique_test retirée").await?;
        let ajoutee = nouvelle_garniture(db, "runique_test ajoutée").await?;
        for (garniture_id, est_defaut) in [(gardee, true), (retiree, false)] {
            plat_garniture::ActiveModel {
                plat_id: Set(plat_id),
                garniture_id: Set(garniture_id),
                est_defaut: Set(est_defaut),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }

        ecrire_liens_plat(&**db, plat_id, &[], &[gardee, ajoutee], &[]).await?;

        let mut liens = search!(plat_garniture::Entity => PlatId eq plat_id,)
            .all(db)
            .await?
            .into_iter()
            .map(|l| (l.garniture_id, l.est_defaut))
            .collect::<Vec<_>>();
        liens.sort();
        let mut attendu = vec![(gardee, true), (ajoutee, false)];
        attendu.sort();
        if liens != attendu {
            return Err(DbErr::Custom(format!(
                "liens {liens:?}, attendu {attendu:?}"
            )));
        }
        Ok(())
    })
    .await
}
