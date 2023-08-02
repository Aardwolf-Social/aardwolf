use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageTitleProps {
    pub page_title: String,
}

#[function_component(PageTitle)]
pub fn main_title(props: &PageTitleProps) -> Html {
    html! {
        <>
            <head>
                <title>{&props.page_title}</title>
            </head>
        </>
    }
}
