use tide::Request;

#[derive(Clone, Debug)]
struct State {}

async fn hello(_req: Request<State>) -> tide::Result {
  return Ok("Hello world!".into());
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::with_state(State {});
  
    app.at("/hello").get(hello);
  
    app.listen("127.0.0.1:8080").await?;
  
    return Ok(());
  }