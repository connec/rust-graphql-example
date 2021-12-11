use uuid::Uuid;

use crate::model::{Human, NewHuman};

pub(crate) struct Db {
    pool: sqlx::PgPool,
}

impl Db {
    pub(crate) async fn connect(uri: &str) -> Result<Self, sqlx::Error> {
        Ok(Self {
            pool: sqlx::PgPool::connect(uri).await?,
        })
    }

    pub(crate) async fn list_humans(&self) -> Result<Vec<Human>, sqlx::Error> {
        sqlx::query_as!(
            Human,
            "
            SELECT id, name, appears_in AS \"appears_in: _\", home_planet
            FROM humans
            ",
        )
        .fetch_all(&self.pool)
        .await
    }

    pub(crate) async fn get_human(&self, id: &Uuid) -> Result<Human, sqlx::Error> {
        sqlx::query_as!(
            Human,
            "
            SELECT id, name, appears_in AS \"appears_in: _\", home_planet
            FROM humans
            WHERE id = $1
            ",
            id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub(crate) async fn insert_human(&self, new_human: NewHuman) -> Result<Human, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }
}
