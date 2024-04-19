interface StockInfo {
    code: string;
    index: number;
    name: string;
    price: number;
    change_percent: number;
    box?: string; // Optional field in TypeScript corresponds to Option<String> in Rust
    hold: boolean;
}
interface StockGroup{
    index: number;
    name: string;
    stocks: StockInfo[];
}
export type { StockInfo,StockGroup };