
fetch-json:
	-git clone --recursive https://github.com/RedisJSON/RedisJSON/

	-git clone --recursive https://github.com/RediSearch/RediSearch.git

prepare-json:
	sudo apt install build-essential llvm cmake libclang1 libclang-dev cargo

prepare-search:
	cd RediSearch; \
	sudo make setup

build-json: fetch-json prepare-json
	cd RedisJSON; \
	cargo build --release

build-search: fetch-search prepare-search
	cd RediSearch; \
	sudo make build -j16

prepare:
	-mkdir dist

build: build-search build-json prepare
	sudo mv RediSearch/bin/*/search/*.so dist
	mv RedisJSON/target/release/*.so dist
	@echo "done"

clean:
	rm -rf ./RedisJSON
	rm -rf ./RediSearch
