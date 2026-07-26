alias b := build
alias s := strip
alias bs := buildStrip
alias rl := release
alias gc := git-cliff

# cargo build --release
build:
    cargo build --release

# strip -S -x
strip:
    strip -S -x ../target/release/weather_cli

# build && strip
buildStrip:
    just build && just strip

# update changelogs
git-cliff:
    git cliff -o CHANGELOG.md && git cliff --latest -o LATEST_CHANGE.md

# goes to homebrew-tap, generates new rb file, adds, commits with version, pushes (RUN THIS AFTER TAGGING)
release:
    cd ../../homebrew-tap && just a
