mod block;
mod blockchain;
mod p2p;

use blockchain::Blockchain;

#[tokio::main]
async fn main() {

    let mut chain = Blockchain::new();

    chain.add_block(
        "Genesis 1:2".into(),
        "And the earth was without form, and void; and darkness was upon the face of the deep.".into(),
    );

    chain.add_block(
        "Genesis 1:3".into(),
        "And God said, Let there be light: and there was light.".into(),
    );

    println!("Scripture Chain started");

    p2p::start_p2p().await;
}
