export COMMAND_PREFIX='!'

export DISCORD_TOKEN=''
export ACCORD_TARGET='http://localhost:8265'
export ACCORD_BIND='localhost:8266'
export ACCORD_COMMAND_MATCH="^${COMMAND_PREFIX}\w+"
export ACCORD_COMMAND_PARSE="^${COMMAND_PREFIX}(\w+)(?:\s+(\w+))?"

export PHP_ENV='production'

export DATABASE_HOST='localhost'
export DATABASE_NAME='sassbot'
export DATABASE_USER='sassbot'

# export DATABASE_PASSWORD='password'
# OR:
# export DATABASE_PASSWORD_FILE='/path/to/file/containing/the/password'
#
# DATABASE_PASSWORD_FILE takes precendence
