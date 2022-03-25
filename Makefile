repo=$(shell basename `pwd`)

run r:
	cargo run

debug:
	cargo build

release:
	cargo build --release

sync:
	rsync -av --delete --exclude='Makefile' --exclude='*.swp' --exclude='.git/*' --exclude='target/*' ./ jumper:/root/${repo}

clean c:
	@rm -f .DS_Store
