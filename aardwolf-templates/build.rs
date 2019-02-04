fn main() {
    build_ructe();
}

/* Compile Ructe templates to rust code
 */
fn build_ructe() {
    let in_dir = "./templates";
    let out_dir = "./compiled_templates/";
    ructe::compile_templates(in_dir.as_ref(), out_dir.as_ref()).unwrap();
}
