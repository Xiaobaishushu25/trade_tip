interface StockInfo {
    code: string;
    index: number;
    name: string;
    box?: string; // Optional field in TypeScript corresponds to Option<String> in Rust
    hold: boolean;
}
export type { StockInfo };