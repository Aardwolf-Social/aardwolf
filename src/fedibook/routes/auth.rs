use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::Template;

#[derive(Debug, Clone, PartialEq, FromForm)]
struct SignUpForm {
    csrf_token: String,
    username: String,
    email: String,
    password: String,
    password_confirmation: String,
}

#[derive(Debug, Clone, PartialEq, FromForm)]
struct SignInForm {
    csrf_token: String,
    email: String,
    password: String,
}

#[get("/auth/sign_up")]
fn sign_up_form() -> Template {
    let token = "some csrf token";
    Template::render("sign_up", hashmap!{ "token" => token })
}

#[get("/auth/sign_in")]
fn sign_in_form() -> Template {
    let token = "some csrf token";
    Template::render("sign_in", hashmap!{ "token" => token })
}

#[post("/auth", data = "<form>")]
fn sign_up(form: Form<SignUpForm>) -> Redirect {
    println!("got sign up form: {:#?}", form.into_inner());
    Redirect::to("/auth/sign_in")
}

#[post("/auth/sign_in", data = "<form>")]
fn sign_in(form: Form<SignInForm>) -> Redirect {
    println!("got sign in form: {:#?}", form.into_inner());
    Redirect::to("/app")
}
