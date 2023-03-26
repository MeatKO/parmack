use std::env;

#[cfg(not(target_env = "gnu"))]
fn check_target_env()
{
	compile_error!("GNU Toolchain is required !")
}

fn main() 
{
    let target_os = 
		env::var("CARGO_CFG_TARGET_OS")
		.expect("Couldn't find Cargo target OS");

    match target_os.as_str()
    {
        "linux" => 
        {
            println!("cargo:rustc-link-lib=xcb");
            println!("cargo:rustc-link-lib=xcb-xfixes");
        }
        "windows" => 
        { 
            let libclang_path = 
                env::var_os("LIBCLANG_PATH")
                .expect("Couldn't Get ENV Variable LIBCLANG_PATH.\nPerhaps the MSVC Clang build tools were not installed or added to PATH?\nlibclang.lib can be found at <VS installation location>\\VC\\Tools\\Llvm\\lib\n");
            
            println!("cargo:rustc-link-search={}", libclang_path.to_str().unwrap());
            println!("cargo:rustc-link-lib=libclang");
            println!("cargo:rustc-link-lib=user32");
        }
        _ => { panic!("Unsupported platform.\nCurrent supported platforms are linux & windows."); }
    }
}