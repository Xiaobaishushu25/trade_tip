use crate::app_errors::AppResult;
use crate::entities::graphic::Column;
use crate::entities::prelude::{ActiveGraphic, Graphic, Graphics};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use std::collections::HashMap;

pub struct GraphicCurd;
impl GraphicCurd {
    // pub async fn query_all()->AppResult<Vec<Graphic>>{
    //     let db = crate::entities::DB.get().ok_or(anyhow::anyhow!("数据库未初始化"))?;
    //     let graphics = Graphics::find().all(db).await?;
    //     Ok(graphics)
    // }
    pub async fn query_by_code(code: String) -> AppResult<Vec<Graphic>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let graphics = Graphics::find()
            .filter(Column::Code.eq(code))
            .all(db)
            .await?;
        Ok(graphics)
    }
    ///查询全部股票的所有水平图形元素的y,即全部的箱体区域
    pub async fn query_only_horizontal_all() -> AppResult<HashMap<String, Vec<f64>>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let graphics = Graphics::find()
            .select_only()
            .column(Column::Code)
            .column(Column::Start)
            .filter(Column::Horizontal.eq(true))
            .into_tuple::<(String, String)>()
            .all(db)
            .await?;
        let mut result: HashMap<String, Vec<f64>> = HashMap::with_capacity(graphics.len());
        for data in graphics.into_iter() {
            let y = data
                .1
                .split(",")
                .map(|num| num.parse::<f64>().unwrap())
                .collect::<Vec<_>>()[1];
            // result.insert(data.0,y);
            //如果存在，则插入，否则创建
            result.entry(data.0).or_insert(Vec::new()).push(y);
        }
        for value in result.values_mut() {
            value.sort_by(|a, b| a.partial_cmp(b).unwrap()); // 默认按照升序排序 f64 类型的元素
        }
        // let mut vec1 = vec![1f64, 0.2f64];
        // vec1.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Ok(result)
    }
    ///根据code查询所有水平图形元素的y,即箱体区域
    pub async fn query_only_horizontal_by_code(code: String) -> AppResult<Vec<f64>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let graphics = Graphics::find()
            .select_only()
            .column(Column::Start)
            .filter(Column::Horizontal.eq(true))
            .filter(Column::Code.eq(code))
            .into_tuple::<String>()
            .all(db)
            .await?;
        let mut result = vec![];
        for start in graphics.into_iter() {
            let y = start
                .split(",")
                .map(|num| num.parse::<f64>().unwrap())
                .collect::<Vec<_>>()[1];
            result.push(y);
        }
        Ok(result)
    }
    ///根据code更新所有相关的图形元素
    /// 采取删除后插入的方式，先删除现存的所有code持有的图形，再插入新的
    pub async fn update_all(code: String, graphic_list: Vec<Graphic>) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let _ = Graphics::delete_many()
            .filter(Column::Code.eq(code))
            .exec(db)
            .await?;
        if !graphic_list.is_empty() {
            let _ = Graphics::insert_many(
                graphic_list
                    .into_iter()
                    .map(ActiveGraphic::from)
                    .collect::<Vec<_>>(),
            )
            .exec(db)
            .await?;
        }
        Ok(())
    }
    pub async fn insert(graphic: Graphic) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let active: ActiveGraphic = graphic.into();
        active.insert(db).await?;
        Ok(())
    }
    pub async fn delete_by_id(id: String) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let _ = Graphics::delete_by_id(id).exec(db).await?;
        Ok(())
    }
    ///根据分组id删除所有图形，这个分组不是股票分组
    pub async fn delete_by_group_id(group_id: String) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let _ = Graphics::delete_many()
            .filter(Column::GroupId.eq(group_id))
            .exec(db)
            .await?;
        Ok(())
    }
    ///根据code删除所有图形
    pub async fn delete_by_code(code: String) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let _ = Graphics::delete_many()
            .filter(Column::Code.eq(code))
            .exec(db)
            .await?;
        Ok(())
    }
}
