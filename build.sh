mkdir app
cd app
nix-shell -p pkg-config openssl pkg-config git gcc rustc cargo --command "git clone https://github.com/dezentrale/spaceapi-rs && cargo build --release && mkdir ~/dezentrale -p & cp target/release/spaceapi-dezentrale* ~/dezentrale && cd ~/pkgs/spaceapi && cargo build --release && cp target/release/pi ~/dezentrale"
