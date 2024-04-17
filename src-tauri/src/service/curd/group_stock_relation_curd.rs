use sea_orm::{ActiveModelTrait, DbErr, EntityTrait};
use sea_orm::ActiveValue::Set;
use crate::app_errors::AppResult;
use crate::entities::init_db_coon;
use crate::entities::prelude::{ActiveGroupStockR, GroupStockR, GroupStockRs};

pub struct GroupStockRelationCurd;
impl GroupStockRelationCurd {
    pub async fn insert(group_stock_r:GroupStockR) -> AppResult<GroupStockR> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let model = ActiveGroupStockR {
            group_id: Set(group_stock_r.group_id),
            stock_code: Set(group_stock_r.stock_code),
            ..Default::default()
        }.insert(db).await?;
        Ok(model)
    }
    pub async fn insert_many(group_stock_rs: Vec<GroupStockR>) -> AppResult<()> {
        let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let models = group_stock_rs.into_iter().map(|group_stock_r| {
            // ActiveGroupStockR {
            //     group_id: Set(group_stock_r.group_id),
            //     stock_code: Set(group_stock_r.stock_code),
            // }
            let ac : ActiveGroupStockR= group_stock_r.into();
            ac
        }).collect::<Vec<_>>();
        let _ = GroupStockRs::insert_many(models).exec(db).await?;
        Ok(())
    }
}
#[tokio::test]
async fn test_insert() {
    init_db_coon().await;
    let model = GroupStockR::new(2,"sz_123456".to_string());
    let result = GroupStockRelationCurd::insert(model).await;
    println!("{:?}", result);
}