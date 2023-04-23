
all: clean build_rs build_client
default: all

clean:
	rm -rvf app/

build_rs:
	cargo build --release
build_client:
	sh build.sh