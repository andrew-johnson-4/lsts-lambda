use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};
use hvm::{language,builder};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

fn make_call(params: &Vec<String>) -> Result<language::Term, String> {
  let name = "Main".to_string();
  let mut args = Vec::new();
  for param in params {
    let term = language::read_term(param)?;
    args.push(term);
  }
  Ok(language::Term::Ctr { name, args })
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();

    let code = match event["source"].as_str() {
       Some(c) => c,
       None => { return Ok(json!({ "error": "Expected 'source' parameter" })) }
    };

    let mut params = Vec::new();
    if let Some(p) = event["p1"].as_str() { params.push(p.to_string()); }
    if let Some(p) = event["p2"].as_str() { params.push(p.to_string()); }
    if let Some(p) = event["p3"].as_str() { params.push(p.to_string()); }
    if let Some(p) = event["p4"].as_str() { params.push(p.to_string()); }
    if let Some(p) = event["p5"].as_str() { params.push(p.to_string()); }

    let call = match make_call(&params) {
       Ok(call) => call,
       Err(msg) => { return Ok(json!({ "error": msg })) }
    };
    let (norm, _cost, _size, _time) = match builder::eval_code(&call, &code, false) {
       Ok((n,c,s,t)) => (n,c,s,t),
       Err(msg) => { return Ok(json!({ "error": msg })) }
    };

    Ok(json!({ "result": format!("{}", norm) }))
}
