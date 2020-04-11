define log
    @echo "##############################################################"
    @echo "[$(shell date --rfc-3339=seconds)]: $(1) $(2) $(3) $(4) $(5) $(6) $(7)"
    @echo "##############################################################"
endef

help:
	@echo "clean - remove temporal files"
	@echo "build - build the project"
	@echo "test  - launch tests"

clean:
	$(call log,"Cleaning the project")

	@cargo clean

build:
	$(call log,"Building the project")

	@cargo build

docker_compose_up:
	@docker-compose -f .docker-compose_local.yml up -d && sleep 5; \
	 docker logs $$(docker ps -aq -f name=postgres);

docker_compose_down:
	@docker-compose -f .docker-compose_local.yml down -v

test: build docker_compose_up
	$(call log,"Launching tests")
	@RUST_TEST_THREADS=1 cargo test --workspace && RESULT=0 || RESULT=1; \
	 $(MAKE) docker_compose_down; \
	 exit $$RESULT