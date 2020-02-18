use lightning_net_tokio::Connection;

async fn f() {
    tokio::spawn(async move {
        Connection::setup_outbound().await;
    });
}

fn main() {}
