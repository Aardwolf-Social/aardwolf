use yew::prelude::*;

use crate::templates::elements::main_title::MainTitle;
use crate::templates::elements::input_email::InputEmail;
use crate::templates::elements::input_username::InputUsername;
use crate::templates::elements::input_password::InputPassword;
use crate::templates::elements::input_password_confirm::InputPasswordConfirm;

use crate::templates::shared::about_us::AboutUs;

#[function_component(SignUp)]
pub fn sign_up() -> Html {

    html!{
        <>
            <MainTitle />
            
            <section class="section">
                <div class="container">
        
                    <div class="columns is-centered">
        
                        // <!-- Begin Left-Hand Column Contents -->
                        <AboutUs />
                        // <!-- End Left-Hand Column -->
        
                        // <!-- Begin Right-Hand Column Contents -->			
                        <div class="column is-mobile">
                            <h1 class="title">
                                {"Create an Account"}
                            </h1>
                            <p class="subtitle">
                                {"Feel free to sign up!"}
                            </p>
        
                            <span style="color: red;">{"Error Message"}</span>
        
                            <form class="box" method="POST" action="/auth">
                                
                                // <!-- Reusable Username Input -->
                                <InputUsername />

                                // <!-- End Reusable Username Input -->

                                // <!-- Reusable Email Input -->
                                <InputEmail />
                                // <!-- End Reusable Email Input -->

                                // <!-- Reusable Password Input -->
                                <InputPassword />
                                // <!-- End Reusable Password Input -->

                                // <!-- Reusable Password Confirm Input -->
                                <InputPasswordConfirm />
                                // <!-- End Reusable Password Confirm Input -->
                                <button>{"Sign Up"}</button>
                            </form>
                        </div>// <!-- End Right-Hand Column -->
                    </div>
                </div>
            </section>
        </>
    }
}