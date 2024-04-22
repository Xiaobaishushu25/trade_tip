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
export type { StockInfo,StockGroup,StockInfoG };