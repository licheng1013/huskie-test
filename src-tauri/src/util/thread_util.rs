use std::thread;

pub struct ThreadUtil {}

impl ThreadUtil {
    // impl是编译的时候内存就固定了的，dyn是不固定的
    fn run(f: impl Fn() + Send + 'static) {
        //f(); // 这行移除注释会报错,因为rust不允许调用后,再次移动闭包
        let _handle = thread::spawn(move || {
            f();
        });
    }
    pub fn many_run(thread_count: i32,f: impl Fn() + Send  + 'static + Clone) {
        for _i in 0..thread_count {
            ThreadUtil::run(f.clone());
        }
    }

}

#[cfg(test)]
mod test {
    use std::{thread, time::{Duration, SystemTime}, sync::mpsc};

    use super::ThreadUtil;

    #[test]
    fn test() {
        let thread_count = 100;
        let count = 1000000; // 100w
        let (tx, rx) = mpsc::channel();
        let f = move|| {
            for _i in 0..count {
                //println!("HelloWorld: {:?}", i);
            }
            tx.send(1).unwrap();
        };
        ThreadUtil::many_run(thread_count,f);
        let start_time = SystemTime::now();
        let mut number  = 0;
        for _i in 0..thread_count{
            let received = rx.recv().unwrap();
            number+=received;
            println!("数据: {:?}",number)
        }
        let end_time = SystemTime::now();
        println!("执行总耗时: {:?}",end_time.duration_since(start_time).unwrap().as_millis())
    }
}
