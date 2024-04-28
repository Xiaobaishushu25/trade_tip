//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::Serialize;
use crate::dtos::graphic_dto::GraphicDTO;

#[derive(Clone, Debug,  DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "graphic")]
pub struct Model {
    pub group_id: String,
    pub code:String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub graphic_type: String,
    pub start:String,//"[1.0,2.0]"
    pub end:String,
    pub content:Option<String>,//文本内容
    pub style:Option<String>,
    pub horizontal:Option<bool>,//是否水平
}
impl From<GraphicDTO> for Model {
    fn from(g:GraphicDTO) -> Self {
        Self{
            group_id:g.group_id,
            code:g.code,
            id:g.id,
            graphic_type:g.graphic_type,
            start:g.start.iter().map(|num| num.to_string()).collect::<Vec<_>>().join(","),
            end:g.end.iter().map(|num| num.to_string()).collect::<Vec<_>>().join(","),
            content:g.content,
            style:if g.style.is_some() { Some(serde_json::to_string(&g.style.unwrap()).unwrap()) }else { None },
            horizontal:g.horizontal,
        }
    }
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    
}

impl ActiveModelBehavior for ActiveModel {}