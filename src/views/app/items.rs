use super::content_loader::add_component;
use super::content_loader::read_file;
use actix_web::HttpResponse;

pub async fn items() -> HttpResponse {
    let mut html_data = read_file("./templates/main.html");
    let javascript_data = read_file("./javascript/main.js");
    let base_css_data = read_file("./css/base.css");
    let main_css_data = read_file("./css/main.css");

    html_data = html_data.replace("{{JAVASCRIPT}}", &javascript_data);
    html_data = html_data.replace("{{BASE_CSS}}", &base_css_data);
    html_data = html_data.replace("{{CSS}}", &main_css_data);
    html_data = add_component(String::from("header"), html_data);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
