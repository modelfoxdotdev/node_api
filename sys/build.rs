fn main() {
	let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
	match target_os.as_str() {
		"macos" => {
			macos_setup();
		}
		"windows" => {
			windows_setup();
		}
		_ => {}
	};
}

fn macos_setup() {
	println!("cargo:rustc-cdylib-link-arg=-Wl");
	println!("cargo:rustc-cdylib-link-arg=-undefined");
	println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
}

fn windows_setup() {
	let version = "v16.4.0";
	let arch = "x64";
	let url = format!(
		"https://nodejs.org/dist/{version}/win-{arch}/node.lib",
		version = version,
		arch = arch
	);
	let node_lib_bytes = reqwest::blocking::get(url).unwrap().bytes().unwrap();
	let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
	let node_lib_path = out_dir.join("node.lib");
	std::fs::write(node_lib_path, node_lib_bytes).unwrap();
	println!("cargo:rustc-link-search={}", out_dir.to_str().unwrap());
	println!("cargo:rustc-link-lib=node");
}
