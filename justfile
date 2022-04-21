# vim: tw=4 sw=4 expandtab
alias c := clean
alias r := run

run:
    cargo run

build PATTERN:
    # cargo build --timings --profile {{PATTERN}}
    cargo build --profile {{PATTERN}}

clean-release:
    cargo clean --release

clean-debug:
    cargo clean --profile dev

fmt:
    cargo fmt --all

clean:
    @rm -f .DS_Store
