use ructe::{Ructe, RucteError};

fn main() -> Result<(), RucteError> {
    build_ructe()?;

    Ok(())
}

/* Compile Ructe templates to rust code
 */
fn build_ructe() -> Result<(), RucteError> {
    let out_dir = "./compiled_templates/";
    let in_dir = "./templates";

    Ructe::new(out_dir.into())?.compile_templates(in_dir)?;

    Ok(())
}
