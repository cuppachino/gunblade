mod credentials;
mod errors;
mod https;
use credentials::*;
use gunblade::partition_results;
use https::LcuClient;

#[tokio::main]
async fn main() {
    let (auth_tokens, _auth_errors) = partition_results(get_credentials().unwrap());
    let clients = Vec::from_iter(auth_tokens);

    for client in &clients {
        get_current_summoner(client).await.unwrap();
    }
}

async fn get_current_summoner(lcu: &LcuClient) -> Result<(), reqwest::Error> {
    let url = format!("{}/lol-summoner/v1/current-summoner", lcu.host_name);
    let res = lcu.client.get(url).send().await?;

    if res.status() == 200 {
        println!("res: {:?}", &res.text().await.unwrap());
    } else {
        println!("res: {:#?}", res);
    }
    Ok(())
}
