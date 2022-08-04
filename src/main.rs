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

fn q(v: Value) -> Value {
    json!({
       "isBase64Encoded": false,
       "statusCode": 200,
       "headers": {
          "content-type": "application/json",
          "Access-Control-Allow-Origin": "*"
       },
       "body": v.to_string()
    })
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();

    let body: Value = match serde_json::from_str(event["body"].as_str().unwrap_or("")) {
       Ok(v) => v,
       Err(_) => { return Ok(q(json!({ "error": "Failed to parse body as json", "result": "" }))); },
    };

    let code = match body["source"].as_str() {
       Some(c) => c,
       None => { return Ok(q(json!({ "error": "Expected 'source' parameter", "result": "" }))) }
    };

    let mut params = Vec::new();
    if let Some(p) = body["p1"].as_str() { params.push(p.to_string()); }
    if let Some(p) = body["p2"].as_str() { params.push(p.to_string()); }
    if let Some(p) = body["p3"].as_str() { params.push(p.to_string()); }
    if let Some(p) = body["p4"].as_str() { params.push(p.to_string()); }
    if let Some(p) = body["p5"].as_str() { params.push(p.to_string()); }

    let call = match make_call(&params) {
       Ok(call) => call,
       Err(msg) => { return Ok(q(json!({ "error": msg, "result": "" }))) }
    };
    let (norm, _cost, _size, _time) = match builder::eval_code(&call, &code, false) {
       Ok((n,c,s,t)) => (n,c,s,t),
       Err(msg) => { return Ok(q(json!({ "error": msg, "result": "" }))) }
    };

    Ok(q(json!({ "error": "", "result": format!("{}", norm) })))
}
