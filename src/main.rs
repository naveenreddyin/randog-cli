use randog;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let fact = randog::get_fact().await?;
    println!("{}",fact.facts[0]) ;
    Ok(())
}
