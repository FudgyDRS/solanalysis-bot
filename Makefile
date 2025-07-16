POSTGRES_CONTAINER_NAME = local-postgres
POSTGRES_USER = postgres
POSTGRES_PASSWORD = password
POSTGRES_DB = solanalysis
POSTGRES_PORT = 8099
POSTGRES_IMAGE = postgres:latest
POSTGRES_HOST = localhost
SQL_DIR = shared/database/migrations

.PHONY: start-db stop-db clean-db logs

start-db:
	docker run --name $(POSTGRES_CONTAINER_NAME) \
		-e POSTGRES_USER=$(POSTGRES_USER) \
		-e POSTGRES_PASSWORD=$(POSTGRES_PASSWORD) \
		-e POSTGRES_DB=$(POSTGRES_DB) \
		-p $(POSTGRES_PORT):5432 -d $(POSTGRES_IMAGE)

stop-db:
	docker stop $(POSTGRES_CONTAINER_NAME)

clean-db:
	docker rm $(POSTGRES_CONTAINER_NAME)

logs:
	docker logs -f $(POSTGRES_CONTAINER_NAME)

run-migrations:
	for file in $(SQL_DIR)/*.sql; do \
		echo "Applying $$file..."; \
		psql -h $(POSTGRES_HOST) -p $(POSTGRES_PORT) -U $(POSTGRES_USER) -d $(POSTGRES_DB) -f $$file; \
	done
