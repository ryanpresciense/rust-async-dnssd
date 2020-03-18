use autotools::Config;
use std::env;
use std::env::{var, var_os};
use std::process::Command;

fn cfg_arch() -> String {
	var("CARGO_CFG_TARGET_ARCH").expect("couldn't find target architecture")
}

fn cfg_family_is(family: &str) -> bool {
	var_os("CARGO_CFG_TARGET_FAMILY").unwrap() == *family
}

fn cfg_os_is(family: &str) -> bool {
	var_os("CARGO_CFG_TARGET_OS").unwrap() == *family
}

fn find_avahi_compat_dns_sd() {
	// on unix but not darwin link avahi compat
	if cfg_family_is("unix") && !(cfg_os_is("macos") || cfg_os_is("ios")) {
		pkg_config::probe_library("avahi-compat-libdns_sd").unwrap();
	}
}

fn find_windows_dns_sd() {
	if cfg_family_is("windows") {
		let platform = match cfg_arch().as_str() {
			"x86_64" => "x64",
			"x86" => "Win32",
			arch => panic!("unsupported target architecture: {:?}", arch),
		};

		match var("BONJOUR_SDK_HOME") {
			Ok(path) => println!("cargo:rustc-link-search=native={}\\Lib\\{}", path, platform),
			Err(e) => panic!("Can't find Bonjour SDK (download from https://developer.apple.com/opensource/) at BONJOUR_SDK_HOME: {}", e),
		}
		println!("cargo:rustc-link-lib=dnssd");
	}
}

fn from_source() {
	let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let avahi_dir = format!("{}/avahi", crate_dir);

	build_deps::rerun_if_changed_paths(&format!("{}/*", avahi_dir)).unwrap();
	let dst = Config::new(&avahi_dir)
		.reconf("-ivf")
		.with("xml", Some("none"))
		.with(
			"distro",
			if cfg!(osx) {
				Some("darwin")
			} else {
				Some("none")
			},
		)
		.disable_shared()
		.enable_static()
		.enable("compat-libdns_sd", None)
		.disable("glib", None)
		.disable("gobject", None)
		.disable("libevent", None)
		.disable("qt4", None)
		.without("libintl-prefix", None)
		.without("libiconv-prefix", None)
		.disable("qt5", None)
		.disable("gtk", None)
		.disable("gtk3", None)
		.disable("dbus", None)
		.disable("gdbm", None)
		.disable("libdaemon", None)
		.disable("python", None)
		.disable("pygobject", None)
		.disable("python-dbus", None)
		.disable("mono", None)
		.disable("monodoc", None)
		.disable("autoipd", None)
		.disable("manpages", None)
		.disable("xmltoman", None)
		.env("CROSS", std::env::var("TARGET").unwrap_or("".to_owned()))
		.env("CC", std::env::var("RUSTC_LINKER").unwrap_or("".to_owned()))
		.build();

	println!("cargo:rustc-link-search=native={}/lib", dst.display());
	println!("cargo:rustc-link-search=native={}", dst.display());
	println!("cargo:rustc-link-lib=static=avahi-core");
	println!("cargo:rustc-link-lib=static=avahi-common");
}

fn from_pkgconfig() {
	find_avahi_compat_dns_sd();
	find_windows_dns_sd();
}

pub fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	if cfg!(feature = "vendored") {
		from_source()
	} else {
		from_pkgconfig()
	}
}
