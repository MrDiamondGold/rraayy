use std::fs::File;

use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};

extern crate gl_generator;

fn main() {
    // let dest = env::var("OUT_DIR").unwrap();
    // let mut file = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();
    let mut file = File::create("src/gl.rs").unwrap();

    Registry::new(Api::Gl, (4, 5), Profile::Compatibility, Fallbacks::All, [
            "GL_ARB_gl_spirv",
        ])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}