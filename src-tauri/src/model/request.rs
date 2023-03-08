use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct  Request {
    /** 请求地址 */
    pub request_addr: String,
    /** 方法 */
    pub method: String,
    /** 线程数 */
    pub thread: i32,
    /** 次数 */
    pub total: i32,
}

