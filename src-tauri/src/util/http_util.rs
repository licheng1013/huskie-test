use std::{sync::mpsc, time::SystemTime};

use crate::model::request::Request;

use super::thread_util::ThreadUtil;

pub struct HttpUtil {}

impl HttpUtil {
    #[tokio::main]
    async fn get(request: Request) -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get(request.request_addr)
            .await?
            .text()
            .await?;
        println!("{:#?}", resp);
        Ok(())
    }

   pub fn get_test(request: Request) -> Result<Request, Box<dyn std::error::Error>> {
        let thread_count = request.thread;
        let (tx, rx) = mpsc::channel();
        let f = move || {
            for _i in 0..request.total {
                let _res = match reqwest::blocking::get(&request.request_addr) {
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
        let result = Request{
            request_addr: todo!(),
            thread: todo!(),
            method: todo!(),
            total: todo!(),
            time: end_time.duration_since(start_time).unwrap().as_millis(),
        };
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use crate::model::request::Request;

    use super::HttpUtil;

    #[test]
    fn test() {
        let v = Request{
            request_addr: "http://localhost:8088/index".into(),
            method: "GET".into(),
            thread: 1,
            total: 10000,
            time: 1, //每个线程请求数量
        };
        HttpUtil::get_test(v);
    }
}
