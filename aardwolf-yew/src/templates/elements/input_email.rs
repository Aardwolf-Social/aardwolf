use yew::prelude::*;

#[function_component(InputEmail)]
pub fn input_email() -> Html {

    html!{
        <>
            <div class="field">
            <label class="label">{"Email Address"}</label>
                <div class="control has-icons-left">
                    <input class="input" type="email" placeholder="e.g. HyenaHugger@SmallCo.net" />
                    <span class="icon is-small is-left">
                        <span class="fa fa-envelope"></span>
                    </span>
                </div>
            </div>
        </>
    }
}