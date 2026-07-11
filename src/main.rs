use syncify::{self,  get_metadata};

const URL: &str = "https://open.spotify.com/track/0InIeZW4P6VO7dUGRM4AKH?si=601229ace85c4f4c";

#[tokio::main]
async fn main() {
    let data = get_metadata(URL).await.unwrap();
    println!("{:?}", data);
}
