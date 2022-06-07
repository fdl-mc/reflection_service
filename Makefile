build:
	@docker build -t fdl-mc/api/reflection .
run:
	@docker run fdl-mc/api/reflection
deploy:
	@docker-compose up -d
