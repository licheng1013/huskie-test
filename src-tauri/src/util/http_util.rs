use std::{sync::mpsc, time::SystemTime};

use crate::model::request::Request;

use super::thread_util::ThreadUtil;

pub struct HttpUtil {}

impl HttpUtil {
    #[tokio::main]
    async fn get() -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get("https://www.shileke.cn/api/banner/list")
            .await?
            .text()
            .await?;
        println!("{:#?}", resp);
        Ok(())
    }

    fn get_test(request: Request) -> Result<(), Box<dyn std::error::Error>> {
        let thread_count = request.thread;
        let count = request.thread * request.total; // 100w
        let (tx, rx) = mpsc::channel();
        let f = move || {
            for _i in 0..count {
                let _res = match reqwest::blocking::get("https://www.shileke.cn/api/banner/list") {
                    Ok(ok) => {
                        println!("数据: {:?}", ok.text().unwrap())
                    },
                    Err(_) => {},
                };
            }
            tx.send(1).unwrap();
        };
        ThreadUtil::many_run(thread_count, f);
        let start_time = SystemTime::now();
        let mut number = 0;
        for _i in 0..thread_count {
            let received = rx.recv().unwrap();
            number += received;
            println!("数据: {:?}", number)
        }
        let end_time = SystemTime::now();
        println!(
            "执行总耗时: {:?}",
            end_time.duration_since(start_time).unwrap().as_millis()
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::HttpUtil;

    #[test]
    fn test() {
        HttpUtil::get().unwrap()
    }
}
