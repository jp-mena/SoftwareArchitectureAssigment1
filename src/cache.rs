use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct CacheService {
    enabled: bool,
}

impl CacheService {
    pub async fn new(_redis_url: Option<&str>) -> Result<Self> {
        // Por ahora, siempre deshabilitado para simplificar
        Ok(Self {
            enabled: false,
        })
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub async fn get<T>(&self, _key: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        Ok(None)
    }

    pub async fn set<T>(&self, _key: &str, _value: &T, _ttl: Option<std::time::Duration>) -> Result<()>
    where
        T: Serialize,
    {
        Ok(())
    }

    pub async fn delete(&self, _key: &str) -> Result<()> {
        Ok(())
    }

    pub async fn delete_pattern(&self, _pattern: &str) -> Result<()> {
        Ok(())
    }

    // Métodos específicos para el dominio de la aplicación
    pub async fn get_author(&self, _author_id: i32) -> Result<Option<crate::models::Author>> {
        Ok(None)
    }

    pub async fn set_author(&self, _author: &crate::models::Author) -> Result<()> {
        Ok(())
    }

    pub async fn get_book(&self, _book_id: i32) -> Result<Option<crate::models::Book>> {
        Ok(None)
    }

    pub async fn set_book(&self, _book: &crate::models::Book) -> Result<()> {
        Ok(())
    }

    pub async fn get_review_scores(&self, _book_id: i32) -> Result<Option<Vec<i32>>> {
        Ok(None)
    }

    pub async fn set_review_scores(&self, _book_id: i32, _scores: &Vec<i32>) -> Result<()> {
        Ok(())
    }

    pub async fn get_author_stats(&self, _author_id: i32) -> Result<Option<crate::models::SimpleAuthorStats>> {
        Ok(None)
    }

    pub async fn set_author_stats(&self, _author_id: i32, _stats: &crate::models::SimpleAuthorStats) -> Result<()> {
        Ok(())
    }

    // Métodos para purgar caché cuando se modifican elementos
    pub async fn purge_author_cache(&self, _author_id: i32) -> Result<()> {
        Ok(())
    }

    pub async fn purge_book_cache(&self, _book_id: i32) -> Result<()> {
        Ok(())
    }

    pub async fn purge_review_cache(&self, _book_id: i32) -> Result<()> {
        Ok(())
    }

    // Método para purgar todo el caché (útil para testing)
    pub async fn flush_all(&self) -> Result<()> {
        Ok(())
    }
}