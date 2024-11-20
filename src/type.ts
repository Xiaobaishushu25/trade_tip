// interface StockInfo {
//     code: string;
//     index: number;
//     name: string;
//     price: number;
//     change_percent: number;
//     box?: string; // Optional field in TypeScript corresponds to Option<String> in Rust
//     hold: boolean;
// }
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
interface RowData {
    code:string,
    price:number, //存老价格，用于和最新价格比较设定每行的css样式
    ma:[string,string],//["均线缠绕",""up""]
    box:[string,string,number|undefined],//["已跌破箱体","down","1.4|undefined"]
    change:string,
    breathClass:string,//每一行的呼吸样式
    advise:string[]//["买入",""up-tag"]
}
interface StockInfoG {
    index: number;
    group_name: string;
    code: string;
    name: string;
    live_data: StockLiveData;
    box?: string; // Optional field in TypeScript corresponds to Option<String> in Rust
    hold: boolean;
    rowData:RowData
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
    id: string;
    graphic_type: string;
    //这里面的左边含义是：10表示横坐标（代表时间，比如一共有20条数据，那么10代表第十条个数据的时间的横坐标），2.0表示纵坐标（代表价格）
    start: number[]; // [10,2.0]
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

interface TransactionRecord {
    date: string;
    time: string;
    code: string;
    name: string;
    direction: string;
    num: number;
    price: number;
    amount: number;
    remark: string
}

interface DisplayConfig{
    a_extend: boolean,
    bs_size:number,//BS点的大小
    k_show_begin:number,//K线显示百分比
    default_remark:string[],
}
interface DataConfig{
    update_freq: number,
    box_num:number,
    up_t_trigger:number,
    down_t_trigger:number,
}

interface Config {
    display_config: DisplayConfig,
    data_config: DataConfig,
}
interface Position{
    date: string;        // Primary key, no auto-increment
    position: number;    // 当前仓位
    sh: number;          // 上证指数，代码缩写：sh
    sz: number;          // 深证成指，代码缩写：sz
    cyb: number;         // 创业板指，拼音缩写：cyb
    sz50: number;        // 上证50，拼音缩写：sz50
    hs300: number;       // 沪深300指数，拼音缩写：hs300
    zz500: number;
}
export { PaintState }; // 导出枚举值
export type { StockLiveData,StockGroup,StockInfoG,StockData,Graphic,RowData,TransactionRecord,DisplayConfig,DataConfig,Config,Position };