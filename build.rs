use autotools::Config;
use std::env;
use std::process::Command;

pub fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let avahi_dir = format!("{}/avahi",crate_dir);

    Command::new(&format!("{}/autogen.sh",avahi_dir))
        .current_dir(&avahi_dir)
        .output()
        .unwrap();

    let dst = Config::new(&avahi_dir)
        .reconf("-ivf")
        .with("xml",Some("none"))
        .with("distro",
              if cfg!(osx) {
                  Some("darwin")
              } else {
                  Some("none")
              }
        )
        .enable("compat-libdns_sd",None)
        .disable("glib",None)
        .disable("gobject",None)
        .disable("libevent",None)
        .disable("qt4",None)
        .without("libintl-prefix",None)
        .without("libiconv-prefix",None)
        .disable("qt5",None)
        .disable("gtk",None)
        .disable("gtk3",None)
        .disable("dbus",None)
        .disable("gdbm",None)
        .disable("libdaemon",None)
        .disable("python",None)
        .disable("pygobject",None)
        .disable("python-dbus",None)
        .disable("mono",None)
        .disable("monodoc",None)
        .disable("autoipd",None)
        .disable("manpages",None)
        .disable("xmltoman",None)
        .target("compile")
        .build();

    build_deps::rerun_if_changed_paths(&format!("{}/*",avahi_dir)).unwrap();
    println!("cargo:rustc-link-search=native={}/out/lib", dst.display());
    println!("cargo:rustc-link-lib=static=avahi-core.a");
    println!("cargo:rustc-link-lib=static=avahi-common.a");
}
