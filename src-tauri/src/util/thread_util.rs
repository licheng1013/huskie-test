use std::thread;

pub struct ThreadUtil {}

impl ThreadUtil {
    fn run(f: Box<dyn Fn() + Send + 'static>) {
        //f(); // 这行移除注释会报错,因为rust不允许调用后,再次移动闭包
        let handle = thread::spawn(move || {
            f();
        });
        handle.join().unwrap()
    }
    fn many_run(threadCount: i32, f: Box<dyn Fn() + Send + 'static>) {
        ThreadUtil::run(f);
        //for i in 0..threadCount {
        //}
    }

}

#[cfg(test)]
mod test {
    use super::ThreadUtil;

    #[test]
    fn test() {
        let f = Box::new(|| {
            for i in 1..10000 {
                println!("HelloWorld: {:?}", i)
            }
        });
        ThreadUtil::many_run(10,f);
    }
}
