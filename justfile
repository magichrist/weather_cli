alias b:= build
alias s:= strip
alias bs:=buildStrip
alias rl:=release
# cargo build --release
build:
	cargo build --release
# strip -S -x
strip:
	strip -S -x ../target/release/weather_cli
# build && strip
buildStrip:
    just build && just strip
# goes to homebrew-tap, generates new rb file, adds, commits with version, pushes
release:
    cd ../../homebrew-tap && pwd && just a

