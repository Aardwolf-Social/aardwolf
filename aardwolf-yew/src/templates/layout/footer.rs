use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {

    html!{
        <>
            <footer class="footer">
                <div class="container">
                <div class="content has-text-centered">
                    <a href="termsofservice.html" class="footer_box">{" Terms of Service "}</a>
                    <span class="vertical_line" />
                    <span class="footer_box">{" Copyright 2018 "}</span>
                    <span class="vertical_line" />
                    <a href="https://github.com/BanjoFox/aardwolf" class="footer_box"> {" Check us out on GitHub "}<i class="fa fa-github"></i>{"!"}</a>
                    <span class="vertical_line" />
                    <a href="https://www.patreon.com/banjofox" class="footer_box">{" Buy the team a coffee "}<i class="fa fa-beer" aria-hidden="true"></i></a>		
                </div>
                </div>
            </footer>
        </>
    }
}