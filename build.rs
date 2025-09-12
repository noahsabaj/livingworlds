use std::env;

fn main() {
    // Only configure Steam linking when the steam feature is enabled
    if env::var("CARGO_FEATURE_STEAM").is_ok() {
        let sdk_path = "/home/nsabaj/Code/sdk";
        
        // Tell cargo where to find the Steam API library
        println!("cargo:rustc-link-search=native={}/redistributable_bin/linux64", sdk_path);
        
        // Link against the Steam API
        println!("cargo:rustc-link-lib=dylib=steam_api");
        
        // Set rpath so the game can find libsteam_api.so at runtime
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}/redistributable_bin/linux64", sdk_path);
        
        // Tell cargo to rerun this script if it changes
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-env-changed=CARGO_FEATURE_STEAM");
    }
}