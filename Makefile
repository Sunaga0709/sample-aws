include .env

build:
	docker-compose build
up:
	docker-compose up -d
down:
	docker-compose down
rebuild:
	@make down
	@make build
	@make up
restart:
	@make down
	@make up
ps:
	docker-compose ps
cargo-add:
	docker-compose exec app cargo add ${name}
cargo-build:
	docker-compose exec app cargo build
cargo-build-prod:
	docker-compose exec app cargo build --release
cargo-check: # 型チェックや構文チェック(実際にコンパイルは行わない)
	docker-compose exec app cargo check
cargo-clean: # target配下のファイルを削除
	docker-compose exec app cargo clean
cargo-update: # クレートアップデート
	docker-compose exec app cargo update
cargo-fmt: # フォーマッタ
	docker-compose exec app cargo fmt
cargo-lint: # リンター
	@make cargo-fmt
	docker-compose exec app cargo clippy
cargo-run:
	docker-compose exec app cargo run
cargo-test:
	docker-compose exec app cargo test -- --nocapture
migrate-create:
	docker-compose exec migrate sqlx migrate add ${name}
migrate:
	docker-compose exec migrate sqlx migrate run
.PHONY: proto
proto:
	rm -f server/src/gen/*
	buf generate
