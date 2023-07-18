use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <>
            <footer class="footer">
                <div class="container">
                <div class="content has-text-centered">
                    <a href="termsofservice.html" class="footer-box">{" Terms of Service "}</a>
                    <span class="vertical-line" />
                    <span class="footer-box">{" Copyright 2018 "}</span>
                    <span class="vertical-line" />
                    <a href="https://github.com/BanjoFox/aardwolf" class="footer-box"> {" Check us out on GitHub "}<i class="fa fa-github"></i>{"!"}</a>
                    <span class="vertical-line" />
                    <a href="https://www.patreon.com/banjofox" class="footer-box">{" Buy the team a coffee "}<i class="fa fa-beer" aria-hidden="true"></i></a>
                </div>
                </div>
            </footer>
        </>
    }
}
