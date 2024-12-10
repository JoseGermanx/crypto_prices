use std::str;

use serde::{Deserialize, Serialize};

fn main() {
    let mut coin: String = String::new();
    print!("Bienvenido a Crypto Prices!\n");
    print!("En este programa podrás consultar el precio de tu criptomoneda favorita en tiempo real.\n");
    println!("Ingrese el nombre de la criptomoneda: ");
    let _ = std::io::stdin()
        .read_line(&mut coin)
        .expect("Error reading input");

    let result_precio = get_precio(&coin);
    match result_precio {
        Ok(precio) => println!("El precio de {} es: ${}", coin, precio),
        Err(e) => println!("Error: {}", e),
    }
}

fn get_precio(coin: &str) -> Result<String, ureq::Error> {
    // let body: String = ureq::get(&format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", coin))
    let body: String = ureq::get(&format!(
        "https://api.coingecko.com/api/v3/coins/{}?localization=false",
        coin
    ))
    .call()?
    .into_string()?;

    let coin_data: CoinData = serde_json::from_str(&body).unwrap();
    //Ok(parsed[coin]["usd"].to_string())
    Ok(coin_data.market_data.current_price.usd.to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct CoinData {
    id: String,
    symbol: String,
    name: String,
    market_data: MarketData
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    current_price: Prices
}

#[derive(Serialize, Deserialize, Debug)]
struct Prices {
    usd: f32,
    clp: f32
}