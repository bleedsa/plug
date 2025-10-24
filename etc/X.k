rule:{[n;c]"rule ",n,"\n  command = ",c}
bld:{[r;o;i]"build ",o,": ",r," ",i}
cargo:bld["cargo"]

tar:"x86_64-unknown-linux-gnu"
rel:0

`0:'("T = ",tar
     "R = ",$[rel;"release";"debug"]
     "F = ",,/("--manifest-path=$in/Cargo.toml"
               $[rel;"--release";""])
     rule["cargo";" "/("CARGO_TARGET_DIR=target cargo +stable build"
                       "--manifest-path=$in/Cargo.toml --target $T")]
     cargo["target/$T/$R";"sin"]);
