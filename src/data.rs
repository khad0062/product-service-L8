use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "PlayStation 5".to_string(),
            price: 500,
            description: "PlayStation 5 Slim Digital Edition Console.".to_string(),
            image: "/playstations.jpeg".to_string()
        },
        Product {
            id: 2,
            name: "HP Laptopp".to_string(),
            price: 400,
            description: "HP 15" Laptop w/ 1 year of Microsoft 365 - Natural Silver (Intel N100/128GB SSD/4GB RAM)".to_string(),
            image: "/hplaptop.jpg".to_string()
        },
        Product {
            id: 3,
            name: "OLED TV".to_string(),
            price: 10000,
            description: "Samsung 2024 42" 4K HDR S90D ATMOS OLED TV (QN42S90DA-OPENBOX)".to_string(),
            image: "/oledtv.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Mac Book".to_string(),
            price: 2000,
            description: "Apple MacBook Air 13.6" w/ Touch ID (2025) - Midnight (Apple M4 / 16GB RAM / 256GB SSD) - English.".to_string(),
            image: "/macbook.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Earbud".to_string(),
            price: 123,
            description: "Sony WF-C700N In-Ear Noise Cancelling True Wireless Earbuds - Black".to_string(),
            image: "/earbud.jpg".to_string()
        }
    ]
}
