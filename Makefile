repo=$(shell basename `pwd`)

run r:
	cargo run

debug:
	cargo build

release:
	cargo build --release

find:
	fd . -HI -tf -S +50k 'images/'

sync:
	rsync -av --delete --exclude='Makefile' --exclude='*.swp' --exclude='.git/*' --exclude='target/*' ./ jumper:/root/${repo}

clean c:
	@rm -f .DS_Store
