use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // mini-redis アドレスのコネクションを開く
    let mut client = client::connect("127.0.0.1:6379").await?;

    let t1 = tokio::spawn(async {
        let res = client.get("hello").await;
    });

    let t2 = tokio::spawn(async {
        client.set("foo", "bar".into()).await;
    });

    t1.await.unwrap();
    t2.await.unwrap();

    Ok(())
}
