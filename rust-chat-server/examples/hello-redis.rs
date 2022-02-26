use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis server.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Get, set K-V pairs.
    client.set("foo", "bar".into()).await?;
    let res = client.get("foo").await?;
    print!("{:?}", res);
    Ok(())
}
