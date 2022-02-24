fn main() {
	match std::env::var("NODE_API_WINDOWS_X64_IMPORT_LIBRARY") {
		Ok(path) => {
			println!("cargo:rustc-link-search={}", path);
			println!("cargo:rustc-link-lib=node");
		}
		Err(std::env::VarError::NotPresent) => {
			let version = "v16.4.0";
			let arch = "x64";
			let url = format!("https://nodejs.org/dist/{version}/win-{arch}/node.lib");
			let node_lib_bytes = reqwest::blocking::get(url).unwrap().bytes().unwrap();
			let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
			let node_lib_path = out_dir.join("node.lib");
			std::fs::write(node_lib_path, node_lib_bytes).unwrap();
			println!("cargo:rustc-link-search={}", out_dir.to_str().unwrap());
			println!("cargo:rustc-link-lib=node");
		}
		Err(e) => {
			panic!(e);
		}
	}
}
