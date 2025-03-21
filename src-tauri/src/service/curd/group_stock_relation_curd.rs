use log::info;
use crate::app_errors::AppResult;
use crate::dtos::stock_dto::StockInfoG;
use crate::entities::group_stock_relation::Column;
use crate::entities::prelude::{
    ActiveGroupStockR, GroupStockR, GroupStockRs, StockGroups, StockInfos,
};
use crate::entities::{group_stock_relation, init_db_coon, open_db_log, stock_info};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, JoinType, LinkDef, Linked, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, RelationTrait,
};

pub struct GroupToStockInfo;

impl Linked for GroupToStockInfo {
    type FromEntity = StockGroups;
    type ToEntity = StockInfos;

    fn link(&self) -> Vec<LinkDef> {
        vec![
            group_stock_relation::Relation::StockGroups.def().rev(),
            group_stock_relation::Relation::StockInfos.def(),
        ]
    }
}

pub struct GroupStockRelationCurd;
impl GroupStockRelationCurd {
    ///不检测是否有重复记录，仅计算index后插入
    pub async fn insert(group_stock_r: GroupStockR) -> AppResult<GroupStockR> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        //查询当前组的index记录的最大值
        let max_index = GroupStockRs::find()
            .select_only()
            .expr(Column::Index.max())
            .filter(Column::GroupName.eq(group_stock_r.group_name.clone()))
            // .order_by_asc(Column::Index)
            .into_tuple::<Option<i32>>()
            .one(db)
            .await?
            .unwrap()
            .unwrap_or(0);
        let model = ActiveGroupStockR {
            group_name: Set(group_stock_r.group_name),
            stock_code: Set(group_stock_r.stock_code),
            index: Set(max_index + 1),
        }
        .insert(db)
        .await?;
        Ok(model)
    }
    pub async fn insert_many(group_stock_rs: Vec<GroupStockR>) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let models = group_stock_rs
            .into_iter()
            .map(|group_stock_r| {
                crate::entities::group_stock_relation::ActiveModel::from(group_stock_r)
            })
            .collect::<Vec<_>>();
        let _ = GroupStockRs::insert_many(models).exec(db).await?;
        Ok(())
    }
    ///根据分组名称查询分组下的所有股票代码（按照索引排序）
    pub async fn query_only_code_by_group_name(group_name: String) -> AppResult<Vec<String>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let codes = GroupStockRs::find()
            .select_only()
            .column(group_stock_relation::Column::StockCode)
            .filter(group_stock_relation::Column::GroupName.eq(group_name.clone()))
            .order_by_asc(group_stock_relation::Column::Index)
            .into_tuple::<String>()
            .all(db)
            .await?;
        Ok(codes)
    }
    ///根据分组名称查询分组下的所有股票（按照索引排序）
    pub async fn query_stocks_by_group_name(group_name: String) -> AppResult<Vec<StockInfoG>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        // let model = StockGroups::find()
        //     .filter(stock_group::Column::Name.eq(group_name.clone()))
        //     .one(db)
        //     .await?
        //     .ok_or(anyhow!("未找到{}分组", group_name))?;
        // let stock_infos = model.find_linked(GroupToStockInfo).all(db).await?;
        // println!("{:?}",stock_infos);
        // let x = StockGroups::find().find_also_linked(GroupToStockInfo).all(db).await?;
        // let x = StockGroups::find().find_with_linked(GroupToStockInfo).all(db).await?;
        // let x = StockGroups::find().find_with_linked(GroupToStockInfo).filter(stock_group::Column::Name.eq(group_name.clone())).all(db).await?;
        let more_infos = GroupStockRs::find()
            .column_as(stock_info::Column::Code, "code") // 假设stock_code来自stock_info表
            // .column_as(stock_group::Column::Name, "group_name")
            // .column_as(group_stock_relation::Column::Index, "index")
            .column_as(stock_info::Column::Name, "name")
            .column_as(stock_info::Column::Box, "box")
            .column_as(stock_info::Column::Hold, "hold")
            // .join(
            //     JoinType::InnerJoin,
            //     group_stock_relation::Relation::StockGroups.def(),
            // )
            .join(
                JoinType::InnerJoin,
                group_stock_relation::Relation::StockInfos.def(),
            )
            // .join_rev(JoinType::InnerJoin, stock_group::Relation::GroupStockRs.def())
            // .join_rev(JoinType::InnerJoin, stock_info::Relation::GroupStockRs.def())
            .filter(group_stock_relation::Column::GroupName.eq(group_name.clone()))
            // .filter(stock_group::Column::Name.eq(group_name.clone()))
            // .join_rev(JoinType::InnerJoin, group_stock_relation::Relation::StockInfos.def())
            .order_by_asc(group_stock_relation::Column::Index)
            .into_model::<StockInfoG>()
            // .into_tuple::<(String,String,i32,String,Option<String>)>()
            .all(db)
            .await?;
        // println!("{:?}", more_infos);
        // let vec = model.find_related(GroupStockRs).all(db).await?.into_iter().map(|model|async  {
        //     // tokio::spawn(async move{
        //     //     let option = model.find_related(StockInfos).one(db).await.unwrap().unwrap();
        //     //     option
        //     // });
        //     let option = model.find_related(StockInfos).one(db).await.unwrap().unwrap();
        //     option
        // }).collect::<Vec<_>>();
        // println!("{:?}",vec);
        Ok(more_infos)
    }
    ///根据股票代码查询所有所在的分组（没有该股票和没有任何分组时返回空的vec）
    pub async fn query_groups_by_stock_code(stock_code: String) -> AppResult<Vec<String>> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        // let option = StockInfos::find_by_id(stock_code).one(db).await?;
        // match option {
        //     None => {
        //         return Ok(None);
        //     }
        //     Some(model) => {
        //         let groups = model
        //             .find_related(GroupStockRs)
        //             .all(db)
        //             .await?
        //             .into_iter()
        //             .map(|model| model.group_name)
        //             .collect::<Vec<_>>();
        //         return Ok(Some(groups));
        //     }
        // }
        // let vec = GroupStockRs::find().filter(<group_stock_relation::Entity as sea_orm::EntityTrait>::Column::StockCode.eq(stock_code)).all(db).await?;
        let groups = GroupStockRs::find()
            .select_only()
            .column(Column::GroupName)
            .filter(Column::StockCode.eq(stock_code))
            .into_tuple::<String>()
            .all(db)
            .await?;
        // println!("{:?}", vec);
        // let result = GroupStockRs::find_by_stock_code(stock_code).one(db).await?;
        Ok(groups)
    }
    ///根据分组名称和股票代码更新分组
    /// 采取删除后插入的方式，先删除现存的所有主键相同的分组，再插入新的分组
    pub async fn update_groups_by_code(stock_code: String, groups: Vec<String>) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let _ = GroupStockRs::delete_many()
            .filter(Column::StockCode.eq(stock_code.clone()))
            .filter(Column::GroupName.is_not_in(groups.clone()))
            .exec(db)
            .await?;
        for name in groups.clone() {
            let result = GroupStockRs::find_by_id((name.clone(), stock_code.clone()))
                .count(db)
                .await?;
            if result == 0 {
                let model = GroupStockR::new(name, stock_code.clone());
                let _ = Self::insert(model).await?;
            }
        }
        Ok(())
    }
    pub async fn delete_by_code_and_group_name(code: String, group_name: String) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let _ = GroupStockRs::delete_by_id((group_name, code))
            .exec(db)
            .await?;
        Ok(())
    }
    pub async fn delete_by_group_name(group_name: String) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        let _ = GroupStockRs::delete_many()
            .filter(Column::GroupName.eq(group_name))
            .exec(db)
            .await?;
        Ok(())
    }
    ///根据股票代码删除所有相关分组信息
    pub async fn delete_by_stock_code(stock_code: String) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(anyhow::anyhow!("数据库未初始化"))?;
        info!("删除分组信息:{}", stock_code);
        let _ = GroupStockRs::delete_many()
            .filter(Column::StockCode.eq(stock_code))
            .exec(db)
            .await?;
        Ok(())
    }
}
#[tokio::test]
async fn test_insert() {
    init_db_coon().await;
    open_db_log().await;
    let model = GroupStockR::new("全部".to_string(), "sz_123456".to_string());
    let result = GroupStockRelationCurd::insert(model).await;
    println!("{:?}", result);
}
#[tokio::test]
async fn test_insert_many() {
    init_db_coon().await;
    let models = vec![
        GroupStockR::new("2".to_string(), "sz_123456".to_string()),
        GroupStockR::new("2".to_string(), "sz_123457".to_string()),
        GroupStockR::new("2".to_string(), "sz_123458".to_string()),
    ];
    let result = GroupStockRelationCurd::insert_many(models).await;
    println!("{:?}", result);
}
#[tokio::test]
async fn test_find_groups_by_stock_code() {
    init_db_coon().await;
    let result = GroupStockRelationCurd::query_groups_by_stock_code("123456".to_string()).await;
    println!("{:?}", result);
}
#[tokio::test]
async fn test_find_stocks_by_group_name() {
    init_db_coon().await;
    open_db_log().await;
    let result = GroupStockRelationCurd::query_stocks_by_group_name("全部".to_string()).await;
    println!("{:?}", result);
}
