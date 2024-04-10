// FORM HERE
// https://www.sheshbabu.com/posts/rust-error-handling/
use std::collections::HashMap;

fn main() {
  match get_current_date() {
    Ok(date) => println!("We've time travelled to {}!!", date),
    Err(e) => eprintln!("Oh noes, we don't know which era we're in! :( \n  {}", e),
  }
}

fn get_current_date() -> Result<String, reqwest::Error> {
  let url = "https://postman-echo.com/time/object";
  let res = reqwest::blocking::get(url);

  let response = match res {
    Ok(res) => res,
    Err(err) => return Err(err),
  };

  let body = response.json::<HashMap<String, i32>>();

  let json = match body {
    Ok(json) => json,
    Err(err) => return Err(err),
  };

  // let date = json["years"].to_string();
  let formatted_date = format!("{}-{}-{}", res["years"], res["months"] + 1, res["date"]);
  let parsed_date = NaiveDate::parse_from_str(formatted_date.as_str(), "%Y-%m-%d")?;
  let date = parsed_date.format("%Y %B %d").to_string();

  Ok(date)
}

// cargo fmt -- --emit=files ./examples/bubble_up_error_string.rs
// cargo run --example bubble_up_error_string
