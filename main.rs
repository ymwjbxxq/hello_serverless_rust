use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use simple_logger::SimpleLogger;

mod dtos;
use dtos::request::Request;
use dtos::response::Response;


#[tokio::main]
async fn main() -> Result<(), Error> {
  // required to enable CloudWatch error logging by the runtime
  SimpleLogger::new()
    .with_level(LevelFilter::Info)
    .init()
    .unwrap();

  let func = handler_fn(my_handler);
  lambda_runtime::run(func).await?;
  Ok(())
}

pub(crate) async fn my_handler(event: Request, ctx: Context) -> Result<Response, Error> {
  log::info!("input {:?}", event);
  let name = event.name;

  let resp = Response {
    req_id: ctx.request_id,
    msg: format!("Hello {}!", name),
  };

  Ok(resp)
}
