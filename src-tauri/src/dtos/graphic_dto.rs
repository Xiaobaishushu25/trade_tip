use crate::entities::prelude::Graphic;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphicDTO {
    pub group_id: String,
    pub code: String,
    pub id: String,
    pub graphic_type: String,
    pub start: Vec<f64>, //"[1.0,2.0]"
    pub end: Vec<f64>,
    pub content: Option<String>, //文本内容
    pub style: Option<Style>,
    pub horizontal: Option<bool>, //是否水平
                                  // pub style:String,
}
impl From<Graphic> for GraphicDTO {
    fn from(g: Graphic) -> Self {
        Self {
            group_id: g.group_id,
            code: g.code,
            id: g.id,
            graphic_type: g.graphic_type,
            start: g
                .start
                .split(",")
                .map(|num| num.parse::<f64>().unwrap())
                .collect::<Vec<_>>(),
            end: g
                .end
                .split(",")
                .map(|num| num.parse::<f64>().unwrap())
                .collect::<Vec<_>>(),
            content: g.content,
            style: if g.style.is_some() {
                Some(serde_json::from_str(&g.style.unwrap()).unwrap())
            } else {
                None
            },
            horizontal: g.horizontal,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Style {
    pub color: Option<String>,
    pub size: Option<f64>,       //文字专用
    pub font: Option<String>,    //文字专用
    pub line_width: Option<f64>, //线专用
}
#[test]
fn test_serde() {
    //     let json = r#"{"key": "value"}"#;
    //     let long_string = r###"This is a very
    // long string that spans
    // multiple lines"###;
    //     let st = r##"{"color":"#000000","size":12.0,"font":"Arial"}"##;
    // let x = serde_json::from_str::<Style>(r#"{"color":"#000000","size":12.0,"font":"Arial"}"#).unwrap();
    // let x = serde_json::from_str::<Style>(r##"{"color":"#000000","size":12.0,"font":"Arial"}"##).unwrap();
    let x = serde_json::from_str::<Style>("{}").unwrap();
    println!("{:?}", x);
}
