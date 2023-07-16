use yew::prelude::*;

#[function_component(InputUsername)]
pub fn input_username() -> Html {

    html!{
        <>
            <div class="field">
            <label class="label">{"Desired Username"}</label>
                <div class="control has-icons-left">
                    <input class="input" type="text" placeholder="e.g. HyenaHugger" />
                    <span class="icon is-small is-left">
                        <span class="fa fa-user"></span>
                    </span>
                </div>
            </div>
        </>
    }
}