fn main() {
    build_ructe();
}

/* Compile Ructe templates to rust code
 */
fn build_ructe() {
    let out_dir = "./compiled_templates/";
    let in_dir = "./templates";
    ructe::compile_templates(in_dir.as_ref(), out_dir.as_ref()).unwrap();
}
