repo=$(shell basename `pwd`)

sync:
	rsync -av --delete --exclude='*.swp' --exclude='.git/*' --exclude='target/*' ./ jumper:/root/${repo}
