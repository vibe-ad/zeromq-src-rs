use std::{env, path::Path};

#[derive(Debug, Clone)]
pub struct Build {}

impl Build {
    /// Create a new build.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Build {
    fn default() -> Self {
        Self::new()
    }
}

impl Build {
    /// Build and link the lib based on the provided options.
    ///
    /// Returns an `Artifacts` which contains metadata for linking
    /// against the compiled lib from rust code.
    pub fn build(&mut self) {
        let vendor = Path::new(env!("CARGO_MANIFEST_DIR")).join("vendor");
        let mut builder = cmake::Config::new(vendor);
        builder
            .configure_arg("-DZMQ_BUILD_TESTS=OFF")
            .configure_arg("-DENABLE_WS=OFF")
            .configure_arg("-DENABLE_DRAFTS=OFF")
            .configure_arg("-DBUILD_STATIC=1")
            .configure_arg("-DENABLE_RADIX_TREE=1")
            .configure_arg("-DBUILD_SHARED=0");

        match env::consts::ARCH {
            "x86_64" => {
                // should this be enabled for ARM?
                builder.configure_arg("-DENABLE_INTRINSICS=1");
            }
            _ => (),
        };

        match env::consts::OS {
            "linux" => {
                builder
                    .configure_arg("-DENABLE_EVENTFD=ON")
                    .configure_arg("-DPOLLER=epoll");
            }
            "macos" | "ios" | "freebsd" | "dragonfly" | "netbsd"
            | "openbsd" => {
                builder.configure_arg("-DPOLLER=kqueue");
            }
            _ => (),
        }

        let dst = builder.build();

        let install_dir = dst.clone();
        let lib_dir = match env::consts::OS {
            "linux" => dst.join("lib64"),
            _ => dst.join("lib"),
        };
        let include_dir = dst.join("include");

        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=static=zmq");
        println!("cargo:include={}", include_dir.display());
        println!("cargo:lib={}", lib_dir.display());
        println!("cargo:out={}", install_dir.display());
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn version_works() {
        let version = testcrate::version();
        println!("{:?}", version);
        assert_eq!(version, (4, 3, 5));
    }
}
