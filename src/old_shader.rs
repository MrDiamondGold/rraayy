use std::{ffi::CStr, fs::File, io::Read, path::Path};

use crate::gl;

#[allow(dead_code)]
pub enum ShaderKind {
    Vertex,
    Fragment,
}

#[allow(dead_code)]
pub struct Shader {
    id: u32,
}

#[allow(dead_code)]
pub struct ShaderCompiler {
    compiler: shaderc::Compiler,
}

#[allow(dead_code)]
impl ShaderCompiler {
    pub fn new() -> Self {
        Self {
            compiler: shaderc::Compiler::new().unwrap(),
        }
    }

    pub fn compile<P: AsRef<Path>>(&mut self, path: P, kind: ShaderKind) -> Shader {
        let gl_kind = match kind {
            ShaderKind::Vertex => {
                gl::VERTEX_SHADER
            },
            ShaderKind::Fragment => {
                gl::FRAGMENT_SHADER
            },
        };

        let shaderc_kind = match kind {
            ShaderKind::Vertex => {
                shaderc::ShaderKind::Vertex
            },
            ShaderKind::Fragment => {
                shaderc::ShaderKind::Fragment
            },
        };

        let mut source: String = String::new();
        
        let filename = path.as_ref().clone().file_name().unwrap().to_str().unwrap();

        File::open(path.as_ref()).unwrap().read_to_string(&mut source).unwrap();

        let options = shaderc::CompileOptions::new().unwrap();
        let binary = self.compiler.compile_into_spirv(source.as_str(), shaderc_kind, filename, "main", Some(&options)).unwrap();

        let id = unsafe {
            gl::CreateShader(gl_kind)
        };

        unsafe {
            gl::ShaderBinary(1, &id, gl::SHADER_BINARY_FORMAT_SPIR_V_ARB, binary.as_binary_u8().as_ptr() as *const _, binary.len() as i32);
            gl::SpecializeShaderARB(id, b"main".as_ptr() as *const _, 0, std::ptr::null(), std::ptr::null());
            
            let mut compiled = 0;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut compiled);

            if compiled != 1 {
                let mut log_size = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut log_size);
                
                println!("Len: {}", log_size);

                let mut log: Vec<gl::types::GLchar> = Vec::with_capacity((log_size + 1) as usize);
                log.set_len((log_size + 1) as usize);

                gl::GetShaderInfoLog(id, log_size, &mut log_size, log.as_mut_ptr() as *mut _);
                let c_str = CStr::from_ptr(log.as_ptr());
                let str = c_str.to_str().unwrap();

                println!("{}", str);

                gl::DeleteShader(id);

                panic!("{}", str);
            }
        }

        Shader {
            id
        }
    }
}