use syncify::download;

const URL: &str = "https://open.spotify.com/track/1UGD3lW3tDmgZfAVDh6w7r?si=f66596d3899347d5";

// const URL: &str = "https://open.spotify.com/track/6Qyc6fS4DsZjB2mRW9DsQs?si=4098d33fff7c4737";

#[tokio::main]
async fn main() {
    download(URL).await;
}
