#[derive(Debug)]
struct Trade {
    ticker: String,
    id: u32,
    price: f32,
    active: bool,
    quantity: f32,
}

impl Trade {
    fn total(&self) -> f32 {
        self.price * self.quantity
    }

    //functions without &self
    fn trade(ticker: String, price: f32, quantity: f32) -> Trade {
        Trade {
            ticker: ticker,
            price: price,
            quantity: quantity,
            active: true,
            id: 1,
        }
    }
}

fn build_trade(ticker: String, price: f32, quantity: f32) -> Trade {
    Trade {
        ticker,
        id: 1,
        price,
        active: true,
        quantity,
    }
}

pub fn structs_demo() {
    println!("--------------------------------------------------");
    println!("structs demo");
    println!("--------------------------------------------------");
    let mut trade = Trade {
        ticker: String::from("MSFT"),
        id: 1,
        price: 400.0,
        quantity: 100.0,
        active: true,
    };
    println!("trade details: {} - {} ", trade.ticker, trade.price);

    //changing trade.
    trade.price = 410.0;
    println!("trade details: {} - {} ", trade.ticker, trade.price);

    //build trade
    let trade2 = build_trade(String::from("GOOG"), 160.0, 200.0);
    println!("trade details: {} - {} ", trade.ticker, trade.price);

    //using .. operator
    let trade3 = Trade {
        ticker: String::from("OrCL"), //=> note that the ticker from trade2 will be lost if I comment this line
        ..trade2
    };
    println!("trade details: {} - {} ", trade3.ticker, trade3.price);

    //=> note that the ticker from trade2 will be lost if we get from trade2
    println!("trade details: {} - {} ", trade2.ticker, trade2.price);

    println!("trade details: {trade2:?}");
    println!("total: {}", trade2.total());

    //tuple structs
    struct Color(i32, i32, i32);
    let c = Color(1, 2, 3);
    //destructuring needs explicit Color
    let Color(r, g, b) = c;

    //using funtions
    let trade4 = Trade::trade(String::from("X"), 40.1, 4.1);
    println!("trade details: {trade4:?}");
    println!("total: {}", trade4.total());
}

//enums
//enums can be defined with mix of different data types
enum TradeType {
    BUY(i32, String),
    SELL(i32),
    OtherType(i32, i32, i32),
}

impl TradeType {
    fn call(&self) {
        println!("TradeType enum...");
    }
}

pub fn enun_demo() {
    let e = TradeType::BUY(1, String::from("test"));
    e.call();
}

//match.. something similar to switch .. case

enum OperationType {
    Plus,
    Minus,
    Division,
    Multiply,
}

fn value(operation: OperationType) -> String {
    match (operation) {
        OperationType::Plus => String::from("+"),
        OperationType::Minus => String::from("-"),
        OperationType::Multiply => String::from("*"),
        OperationType::Division => String::from("/"),
    }
}


