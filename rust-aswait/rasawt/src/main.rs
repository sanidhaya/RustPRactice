use error_chain::error_chain;

error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);

    }
}


async fn testfunc()->Result<()>{
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status : {}\n",res.status());
    println!("Headers : {:?}\n",res.headers());
    let body = res.text().await?;
    println!("Body : {:?}\n",body);
    Ok(())
}

fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(
        async{
            if let Err(err) = testfunc().await{
                eprintln!("Error : {}",err);
            }
        }
    )
}
