use yew::prelude::*;

#[function_component(InputPassword)]
pub fn input_password() -> Html {

    html!{
        <>
            <div class="field">
                <label class="label">{"Password"}</label>
                <div class="control has-icons-left">
                    <input class="input" type="password" placeholder="*************" />
                    <span class="icon is-small is-left">
                        <span class="fa fa-lock"></span>
                    </span>
                </div>
            </div>
        </>
    }
}