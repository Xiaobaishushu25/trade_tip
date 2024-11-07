use sea_orm::ActiveModelTrait;
use crate::app_errors::AppResult;
use crate::entities::prelude::{ActivePosition, Position};

pub struct PositionCurd;
impl PositionCurd {
    pub async fn insert_position(position: Position) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let active: ActivePosition = position.into();
        active.insert(db).await?;
        Ok(())
    }
}