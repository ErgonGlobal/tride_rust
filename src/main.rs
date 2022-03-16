extern crate redis;
use redis::Commands;

fn fetch_an_integer() -> redis::RedisResult<String> {
    // connect to redis
    let client = redis::Client::open("redis://10.124.1.14/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _ : () = con.set("my_key", "42")?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.

    con.get("my_key")

    
}

fn main() {
    let key  =  fetch_an_integer();
    println!("key: {}", key.unwrap());
}