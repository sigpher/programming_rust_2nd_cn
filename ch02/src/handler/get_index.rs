use actix_web::{HttpResponse, Responder};

pub async fn get_index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
<title>GCD Calculator</title>
<form action="/gcd" method="post">
  <input type="text" name="n"/>
  <input type="text" name="m"/>
  <button type="submit">Compute GCD</button>
</form>
"#,
    )
}
