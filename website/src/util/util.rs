use yew::Html;
use gloo::utils as gloo_utils;

const TEXT: &str     = include_str!("../../data/front.html");

pub fn lipsum_text() -> Html {
    let div  = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(TEXT);
    div.set_class_name("lipsum");
    div.set_class_name("space-y-5 text-justify");

    let h1 = div.query_selector("h1").unwrap().unwrap();
    h1.set_class_name("flex my-1 sm:my-5 text-2xl font-bold");

    Html::VRef(div.into())
}
