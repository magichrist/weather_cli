alias b:= build
alias s:= strip
alias bs:=buildStrip
# cargo build --release
build:
	cargo build --release
# strip -S -x
strip:
	strip -S -x ../target/release/weather_cli
# build && strip
buildStrip:
    just build && just strip

