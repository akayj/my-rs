repo=$(shell basename `pwd`)

run r:
	cargo run

debug:
	cargo build

release:
	cargo build --release

find:
	fd . -HI -tf -S +80ki 'images/' -X ls -lhS

sync:
	# rsync -av --delete --exclude='Makefile' --exclude='*.swp' --exclude='.git/*' --exclude='target/*' ./ jumper:/root/${repo}
	rsync -av --delete --exclude='Makefile' --exclude='*.swp' --exclude='.git/*' --exclude='target/*' --exclude='images/*' ./ local-deb:/home/yj/${repo}

clean c:
	@rm -f .DS_Store
