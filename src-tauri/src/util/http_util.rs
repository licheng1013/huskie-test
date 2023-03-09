
pub struct HttpUtil {}

impl HttpUtil {
   
    #[tokio::main]
    async fn get() -> Result<(), Box<dyn std::error::Error>> {
        let resp = 
        reqwest::get("https://www.shileke.cn/api/banner/list")
            .await?.text().await?;
        println!("{:#?}", resp);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::HttpUtil;

    #[test]
    fn test()  {
        HttpUtil::get().unwrap()
    }
}