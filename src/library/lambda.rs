pub mod handler {
  use crate::dtos::request::Request;
  use crate::dtos::response::Response;
  use crate::errors::string_error::StrError;
  use lambda_runtime::{Context, Error};

  pub(crate) async fn execute(event: Request, ctx: Context) -> Result<Response, Error> {
    log::info!("input {:?}", event);

    if event.name.is_empty() {
      Err(Box::new(StrError("Lambda input not valid")))
    } else {
      let resp = Response {
        req_id: ctx.request_id,
        msg: format!("Hello {}!", event.name),
      };
      Ok(resp)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::dtos::request::Request;
  use crate::execute;
  use lambda_runtime::Context;

  #[tokio::test]
  async fn return_message() {
    //ASSERT
    let request = Request {
      name: "Daniele".to_string(),
    };

    let ctx = Context::default();

    // ACT
    let result = execute(request, ctx).await.expect("Unable to send request");

    //ASSERT
    assert_eq!(result.msg, "Hello Daniele!");
  }

  #[tokio::test]
  async fn return_error() {
    //ASSERT
    let request = Request::default();
    let ctx = Context::default();

    // ACT
    let result = execute(request, ctx).await;
    assert!(result.is_err());
  }
}
