use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};
use crate::app_errors::AppResult;
use crate::entities::position::Column;
use crate::entities::prelude::{ActivePosition, Position, Positions};

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
    /// 更新
    /// 只更新position字段
    pub async fn update_position(position: Position) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let mut active_model = position.into_active_model();
        active_model.reset(Column::Position);
        // let old_position = Positions::find_by_id(position.date).one(db).await?.ok_or(anyhow::anyhow!("未找到该记录{}", position.date))?;
        active_model.update(db).await?;
        Ok(())
    }
}