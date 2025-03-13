use sqlx::{query, query_as};
use crate::types::Interaction;
use crate::db::DbPool;

pub async fn insert_interaction(db_pool: &DbPool, interaction: &Interaction) -> Result<(), sqlx::Error> {
    if let Some(pool) = &db_pool.sqlite {
        query("INSERT INTO interactions (id, model, prompt, response, created_at) VALUES (?, ?, ?, ?, ?)")
            .bind(interaction.id.clone())
            .bind(interaction.model.clone())
            .bind(interaction.prompt.clone())
            .bind(interaction.response.clone())
            .bind(interaction.created_at)
            .execute(pool)
            .await?;
    } else if let Some(pool) = &db_pool.postgres {
         query("INSERT INTO interactions (id, model, prompt, response, created_at) VALUES ($1, $2, $3, $4, $5)")
            .bind(interaction.id.clone())
            .bind(interaction.model.clone())
            .bind(interaction.prompt.clone())
            .bind(interaction.response.clone())
            .bind(interaction.created_at)
            .execute(pool)
            .await?;
    }
    else if let Some(pool) = &db_pool.mysql {
         query("INSERT INTO interactions (id, model, prompt, response, created_at) VALUES (?, ?, ?, ?, ?)")
            .bind(interaction.id.clone())
            .bind(interaction.model.clone())
            .bind(interaction.prompt.clone())
            .bind(interaction.response.clone())
            .bind(interaction.created_at)
            .execute(pool)
            .await?;
    }

    Ok(())
}

pub async fn get_interactions(db_pool: &DbPool) -> Result<Vec<Interaction>, sqlx::Error> {
     if let Some(pool) = &db_pool.sqlite {
        query_as::<_, Interaction>("SELECT * FROM interactions")
            .fetch_all(pool)
            .await
    } else  if let Some(pool) = &db_pool.postgres {
        query_as::<_, Interaction>("SELECT * FROM interactions")
            .fetch_all(pool)
            .await
    } else {
        query_as::<_, Interaction>("SELECT * FROM interactions")
            .fetch_all(db_pool.mysql.as_ref().unwrap())
            .await
    }
}
