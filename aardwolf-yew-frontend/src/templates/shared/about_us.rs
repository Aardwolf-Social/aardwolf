use yew::prelude::*;
use rust_i18n::t;

#[function_component(AboutUs)]
pub fn about_us() -> Html {
    html! {
        <>
            <div class="column is-mobile">
                <h1 class="title">
                    {t!("About Aardwolf")}
                </h1>
                <p class="subtitle">
                    {t!("This is who we are!")}
                </p>
                <p>{t!("Aardwolf is a platform for creating new social networks, connected across the web. While existing social media sites work to funnel the world into a single shared experience (and advertising marketplace), we recognize that we are all individuals with different identities and interests.")}</p><br />

                <p>{t!("Each server hosts it's own community of users who are posting, sharing pictures, links, etc. They are replying and liking each other's posts, and re-sharing the ones they like best.")}</p><br />

                <p>{("Users are not limited to only interacting with other users on their service: they can follow users on other sites that are powered by Aardwolf just as if they were on their own site (the official term is Federation). They can even connect with users on other platforms, if -- like the microblogging service")} <a href="http://joinmastodon.org">{" Mastodon "}</a> {t!("-- they implement the same open protocols Aardwolf is built on.")}</p><br />

                <p><a href="https://github.com/banjofox/aardwolf">{"Aardwolf "}</a> {t!("is open source, so developers who want to contribute or understand how it works can dig in and do so.")}</p>
            </div>
        </>
    }
}
