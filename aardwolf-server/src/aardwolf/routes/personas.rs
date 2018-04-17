#[get("/new")]
fn new() -> String {
    format!("placeholder")
}

#[post("/create")]
fn create() -> String {
    format!("placeholder")
}

#[get("/delete/<delete_persona>")]
fn delete(delete_persona: i32) -> String {
    format!("placeholder, {}", delete_persona)
}

#[get("/switch/<switch_persona>")]
fn switch(switch_persona: i32) -> String {
    format!("placeholder, {}", switch_persona)
}
