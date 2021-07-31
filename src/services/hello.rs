use eyre::Result;

pub async fn hello() -> Result<&'static str> {
    Ok("Hello")
}
