#![deny(warnings)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate serde_derive;

use bytes::Buf as _;
use hyper::Client;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "http://jsonplaceholder.typicode.com/users".parse().unwrap();
    let users = fetch_json(url).await?;
    // print users
    println!("users: {:#?}", users);

    // print the sum of ids
    let sum = users.iter().fold(0, |acc, user| acc + user.id);
    println!("sum of ids: {}", sum);
    Ok(())
}

async fn fetch_json(url: hyper::Uri) -> Result<Vec<User>> {
    let client = Client::new();

    // Fetch the url...
    let res = client.get(url).await?;
    println!("\n{:#?}", res);

    // asynchronously aggregate the chunks of the body
    let body = hyper::body::aggregate(res).await?;
    println!("\n{:#?}", body);

    // try to parse as json with serde_json
    let users = serde_json::from_reader(body.reader())?;

    Ok(users)
}

#[derive(Deserialize, Debug)]
struct User {
    id: i32,
    name: String,
}

impl fmt::Debug for Buf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Buf")
         .finish()
    }
}