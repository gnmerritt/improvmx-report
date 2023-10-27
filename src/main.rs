use std::error::Error;

mod improvmx;

fn main() -> Result<(), Box<dyn Error>> {
    let key = std::env::var("IMPROVMX_KEY")?;
    let client = improvmx::ImprovMx::new(&key);
    let domains = client.domains()?;
    for d in domains {
        println!("{:?}", &d);
        let undelivered = client.undelivered_messages(&d)?;
        println!("{:?}", undelivered);
    }
    Ok(())
}
