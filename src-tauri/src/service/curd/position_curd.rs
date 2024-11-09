use crate::app_errors::AppResult;
use crate::entities::position::Column;
use crate::entities::prelude::{ActivePosition, Position, Positions};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, QueryOrder};
use sea_orm::ActiveValue::Set;

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
    pub async fn insert_many_positions(positions: Vec<Position>) -> AppResult<()> {
        assert_ne!(positions.len(), 0);
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        Positions::insert_many(
            positions
                .into_iter()
                .map(|item| item.into_active_model())
                .collect::<Vec<_>>(),
        )
        .exec(db)
        .await?;
        Ok(())
    }
    /// 更新
    /// 只更新position字段
    // pub async fn update_position(position: Position) -> AppResult<()> {
    //     let db = crate::entities::DB
    //         .get()
    //         .ok_or(anyhow::anyhow!("数据库未初始化"))?;
    //     let mut active_model = position.into_active_model();
    //     active_model.reset(Column::Position);
    //     // let old_position = Positions::find_by_id(position.date).one(db).await?.ok_or(anyhow::anyhow!("未找到该记录{}", position.date))?;
    //     active_model.update(db).await?;
    //     Ok(())
    // }
    /// 更新
    /// 只更新position字段
    pub async fn update_position_by_id(date:String, position_num: f64) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let position = Positions::find_by_id(date.clone()).one(db).await?.ok_or(anyhow::anyhow!("未找到该记录{}", date))?;
        let mut active_position = position.into_active_model();
        active_position.position = Set(position_num);
        active_position.update(db).await?;
        // let mut active_model = position.into_active_model();
        // active_model.reset(Column::Position);
        // let old_position = Positions::find_by_id(position.date).one(db).await?.ok_or(anyhow::anyhow!("未找到该记录{}", position.date))?;
        // active_model.update(db).await?;
        Ok(())
    }
    ///查询最新的持仓数据
    /// 注意，没有数据的时候返回None
    pub async fn query_latest_position() -> AppResult<Option<Position>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        Ok(Positions::find()
            .order_by_desc(Column::Date)
            .one(db)
            .await?)
    }
    ///查询所有的持仓数据（以日期升序排列，即最新数据在最后面）
    pub async fn query_all() -> AppResult<Vec<Position>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        Ok(Positions::find()
            // .order_by_desc(Column::Date)
            .order_by_asc(Column::Date)
            .all(db)
            .await?)
    }
}
