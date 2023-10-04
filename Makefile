.PHONY: deploy
deploy:
	cargo shuttle deploy --allow-dirty

.PHONY: logs
logs:
	cargo shuttle logs --follow

.PHONY: dev watch local
dev watch local:
	cargo shuttle run
