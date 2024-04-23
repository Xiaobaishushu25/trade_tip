interface StockInfo {
    code: string;
    index: number;
    name: string;
    price: number;
    change_percent: number;
    box?: string; // Optional field in TypeScript corresponds to Option<String> in Rust
    hold: boolean;
}
interface StockInfoG {
    index: number;
    group_name: string;
    code: string;
    name: string;
    price: number;
    change_percent: number;
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
// interface StockGroup{
//     index: number;
//     name: string;
//     stock_codes: string[];
// }
export type { StockInfo,StockGroup,StockInfoG,StockData };