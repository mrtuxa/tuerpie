mkdir app
cd app || exit
git clone https://github.com/dezentrale/spaceapi-rs app/spaceapi
cd app/spaceapi || exit
nix-shell -p pkg-config openssl --command "cargo build --release"
cd .. || exit
cp target/release/spaceapi-dezentrale-client .
cp ../target/release/pi .