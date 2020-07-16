use hello_world::greeter_client::GreeterClient;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Make sure the server is running.\n");
    println!("client 1: should not work (http://127.0.99.99:50051)");
    let client1 = GreeterClient::connect("http://127.0.99.99:50051").await;
    dbg!(&client1);
    println!("client 2: works (http://127.0.0.1:50051)");
    let client2 = GreeterClient::connect("http://127.0.0.1:50051").await;
    dbg!(&client2);

    Ok(())
}
