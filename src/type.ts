interface StockInfo {
    code: string;
    index: number;
    name: string;
    price: number;
    change_percent: number;
    box?: string; // Optional field in TypeScript corresponds to Option<String> in Rust
    hold: boolean;
}
interface StockLiveData {
    code: string;
    price: number;
    change: number;
    percent: number;
    volume: number;
    high: number;
    low: number;
    open: number;
    ma5: number;
    ma10: number;
    ma20: number;
    ma30: number;
    ma60: number;
}
// interface rowData{
//     code:string,
//     price:number,
//     ma:string,//["均线状态",""up""]
//     box:string,
//     change:string,
//     breathClass:string,
//     advise:string[]
// }
interface rowData{
    code:string,
    price:number,
    ma:[string,string],//["均线状态",""up""]
    box:[string,string,number|undefined],//["已跌破箱体","down","1.4|undefined"]
    change:string,
    breathClass:string,
    advise:string[]
}
interface StockInfoG {
    index: number;
    group_name: string;
    code: string;
    name: string;
    live_data: StockLiveData;
    box?: string; // Optional field in TypeScript corresponds to Option<String> in Rust
    hold: boolean;
    rowData:rowData
}
interface StockData {
    // 在TypeScript中，没有直接的默认字段值或属性重命名特性，
    // 所以我们仅声明字段和类型。
    date: string; // Rust中的`day`字段，使用TypeScript的命名习惯
    open: number; // Rust中的`open`字段，使用`f64`对应的TypeScript类型`number`
    close: number; // Rust中的`close`字段
    high: number; // Rust中的`high`字段
    low: number; // Rust中的`low`字段
    vol: number; // Rust中的`vol`字段，对应`volume`的反序列化
    ma5?: number; // Rust中的`ma5`字段，使用`Option<f64>`对应的TypeScript类型`number | undefined`
    ma10?: number; // Rust中的`ma10`字段
    ma20?: number; // Rust中的`ma20`字段
    ma30?: number; // Rust中的`ma30`字段
    ma60?: number; // Rust中的`ma60`字段
}
interface StockGroup{
    index: number;
    name: string;
    stocks_change: boolean;
}
// 定义枚举类型
// HL(Horizontal line )：水平直线
// HLS(Horizontal line segment)：水平线段
// PHL 标价直线
// PHLS 标价线段
// LS(line segment)：斜线段
// Text(text)：文本
enum PaintState {
    Null,HL,HLS,PHL,PHLS,LS,Text
}

interface Graphic{
    group_id: string;
    code: string;
    id: string; // 使用Option<String>在Rust中表示可选字段，在TypeScript中对应为可选属性
    graphic_type: string;
    start: number[]; // 假设start是一个JSON格式的字符串，例如"[1.0,2.0]"
    end: number[];
    content?: string; // 文本内容
    style?: Style; // 假设config是包含文本配置信息的字符串
    horizontal?: boolean;
}
interface Style {
    color?: string; // 使用?表示属性是可选的，相当于Option<String>
    size?: number; // 文字专用，使用number类型对应f64，?表示可选
    font?: string; // 文字专用，使用string类型，?表示可选
    line_width?: number; // 线专用，使用number类型对应f64，?表示可选
}
export { PaintState }; // 导出枚举值
export type { StockInfo,StockLiveData,StockGroup,StockInfoG,StockData,Graphic,rowData };