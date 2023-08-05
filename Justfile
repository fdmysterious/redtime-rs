#####################################################
# Configuration variables
#####################################################

test_redmine_container:="redtime-test-redmine"
redmine_port:="3000"
test_db_path:="misc/redmine-test-db.db"
test_api_key:="e32a11873d3fc493b70384b27e63fc070b4e5143"


#####################################################
# Public recipes
#####################################################

# Initialize the test redmine server
redmine-init:
	@echo "Init. test redmine docker container"
	docker create -p {{redmine_port}}:3000 --name {{test_redmine_container}} redmine && just redmine-start && just redmine-import-db


	@echo "Init test .env file"
	just init-env-config

	@printf "\n\033[1mRedmine server successfully initialized. Open http://$(just redmine-get-ip):{{redmine_port}} to access web interface.\033[0m\n"

# Start the test redmine server. If doesn't exists, creates it and initialize a test database. Prints the IP address
redmine-start:
	docker start {{test_redmine_container}}

# Stop the test redmine server
redmine-stop:
	docker stop {{test_redmine_container}}

# Remove the test redmine server
redmine-remove:
	docker rm {{test_redmine_container}}

# Open the redmine web page
redmine-open:
	#!/bin/sh
	ip=$(just redmine-get-ip)
	port={{redmine_port}}

	xdg-open http://$ip:$port


# Export the sqlite db from the test redmine container to the host
redmine-export-db:
	docker cp {{test_redmine_container}}:/usr/src/redmine/sqlite/redmine.db {{test_db_path}}


#####################################################
# Private recipes
#####################################################

# Get the redmine server IP
[private]
redmine-get-ip:
	@docker inspect -f '{{{{range.NetworkSettings.Networks}}{{{{.IPAddress}}{{{{end}}' {{test_redmine_container}}

# Get the redmine API key
[private]
redmine-get-api_key:
	@echo "Install sqlite3 to retrieve API key"
	@docker exec -it {{test_redmine_container}} sh -c "apt-get update && apt-get install -y sqlite3"
	@docker exec -it {{test_redmine_container}} sqlite3 /usr/src/redmine/sqlite/redmine.db "select value from tokens where action='api'" | sed 's/.$//'

# Import the sqlite db from the host to the test redmine container
[private]
redmine-import-db:
	docker cp {{test_db_path}} {{test_redmine_container}}:/usr/src/redmine/sqlite/redmine.db && docker exec {{test_redmine_container}} chown redmine:redmine /usr/src/redmine/sqlite/redmine.db
	
# Inits the test .env file
[private]
init-env-config:
	echo "REDMINE_API_KEY=\"{{test_api_key}}\"" > .env
	echo "REDMINE_URL=\"http://$(just redmine-get-ip):3000\"" >> .env

[private]
redmine-shell:
	docker exec -it {{test_redmine_container}} bash
