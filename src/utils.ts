import {ElNotification} from "element-plus";

function debounce<T extends (...args: any[]) => any>(
    callback: T,
    delay: number,
    isFirstExecutionImmediate: boolean = false
): (...args: Parameters<T>) => void {
    let timer: number | null = null;
    let firstExecutionHappened = false;

    return function (...args: Parameters<T>): void {
        const context = this;
        //文心一言写的这个防抖函数不行，我自己改了下
        //如果是立即执行，那么判断定时器是否在，如果不在说明可以执行，那么就执行，接着就设置定时器，到时间了吧定时器置为null，允许下一次执行
        if (isFirstExecutionImmediate){
            if (timer){
            }else {
                callback.apply(context, args);
            }
            timer = setTimeout(() => {
                // callback.apply(context, args);
                // clearTimeout(timer)
                timer = null;
            }, delay);
        }
        // if (timer) {
        //     clearTimeout(timer);
        //     timer = null;
        // }
        // if (isFirstExecutionImmediate && !firstExecutionHappened) {
        //     console.log("执行")
        //     callback.apply(context, args);
        //     firstExecutionHappened = true;
        // } else if (!isFirstExecutionImmediate){
        //     console.log("执行定时器")
        //     timer = setTimeout(() => {
        //         callback.apply(context, args);
        //     }, delay);
        // }
    };
}
function generateId(): string {
    // 生成一个 0 到 999999 之间的随机数
    const randomNumber = Math.floor(Math.random() * 1000000);

    // 将数字转换为字符串
    const id = randomNumber.toString();

    // 确保字符串是六位长度，如果不足六位，前面补零
    const paddedId = id.padStart(6, '0');

    return paddedId;
}
// const successNotification = (content:string) => {
//     ElNotification({
//         title: 'Success',
//         message: content,
//         type: 'success',
//         position: 'bottom-right',
//     })
// }
export { debounce, generateId};