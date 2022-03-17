use uuid::Uuid;

use crate::model::{Human, NewHuman};

pub(crate) struct Db {
    tx: crate::Tx,
}

impl Db {
    pub(crate) fn new(tx: crate::Tx) -> Self {
        Self { tx }
    }

    pub(crate) async fn list_humans(&mut self) -> Result<Vec<Human>, sqlx::Error> {
        sqlx::query_as!(
            Human,
            "
            SELECT id, name, appears_in AS \"appears_in: _\", home_planet
            FROM humans
            ",
        )
        .fetch_all(&mut self.tx)
        .await
    }

    pub(crate) async fn get_human(&mut self, id: &Uuid) -> Result<Human, sqlx::Error> {
        sqlx::query_as!(
            Human,
            "
            SELECT id, name, appears_in AS \"appears_in: _\", home_planet
            FROM humans
            WHERE id = $1
            ",
            id
        )
        .fetch_one(&mut self.tx)
        .await
    }

    pub(crate) async fn insert_human(&mut self, new_human: NewHuman) -> Result<Human, sqlx::Error> {
        sqlx::query_as!(
            Human,
            "
            INSERT INTO humans (name, appears_in, home_planet)
            VALUES ($1, $2, $3)
            RETURNING id, name, appears_in AS \"appears_in: _\", home_planet
            ",
            new_human.name(),
            new_human.appears_in() as _,
            new_human.home_planet(),
        )
        .fetch_one(&mut self.tx)
        .await
    }
}
