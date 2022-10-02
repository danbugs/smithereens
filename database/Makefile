# MIGRATIONS
.PHONY: migration-run
migration-run:
	diesel migration run --database-url ${PIDGTM_DATABASE_URL}

.PHONY: migration-redo
migration-redo:
	diesel migration redo --database-url ${PIDGTM_DATABASE_URL}

.PHONY: migration-revert
migration-revert:
	diesel migration revert --database-url ${PIDGTM_DATABASE_URL}