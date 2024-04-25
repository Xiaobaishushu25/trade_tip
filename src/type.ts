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
    ma5: number | null;
    ma10: number | null;
    ma20: number | null;
    ma30: number | null;
    ma60: number | null;
}
interface StockInfoG {
    index: number;
    group_name: string;
    code: string;
    name: string;
    live_data: StockLiveData;
    // price: number;
    // change_percent: number;
    // volume: number;
    // high: number;
    // low: number;
    // open: number;
    // ma5: number | null;
    // ma10: number | null;
    // ma20: number | null;
    // ma30: number | null;
    // ma60: number | null;
    box?: string; // Optional field in TypeScript corresponds to Option<String> in Rust
    hold: boolean;
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
// interface StockGroup{
//     index: number;
//     name: string;
//     stock_codes: string[];
// }
interface StockGroup{
    index: number;
    name: string;
    stocks_change: boolean;
}
// 定义枚举类型
// HLS(Horizontal line )：水平直线
//HLS(Horizontal line segment)：水平线段
// LS(line segment)：垂直线段
// Text(text)：文本
enum PaintState {
    Null,HL,HLS,LS,Text
}
// 定义点的接口
interface Point {
    x: number;
    y: number;
}
interface Line {
    id: string;
    start: [number,number];
    end: [number,number]|undefined;
    type: PaintState;
}
// interface StockGroup{
//     index: number;
//     name: string;
//     stock_codes: string[];
// }
export { PaintState }; // 导出枚举值
export type { StockInfo,StockLiveData,StockGroup,StockInfoG,StockData,Point,Line };