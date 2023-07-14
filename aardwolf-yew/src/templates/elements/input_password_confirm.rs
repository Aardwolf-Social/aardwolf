use yew::prelude::*;

#[function_component(InputPasswordConfirm)]
pub fn input_password_confirm() -> Html {

    html!{
        <>
            <div class="field">
                <label class="label">{"Confirm Password"}</label>
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