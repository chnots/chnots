set -eu

# ###################################
# Final Environments
# ###################################

# is script mode, set it is to 1, then this file is going to be executed as finanl request script.
SCRIPT_MODE=0

# request method: GET POST PUT DELETE
METHOD="PUT"

# request url
REQ_URL="http://localhost:3012/api/v1/chnot-query"

# headers
HEADERS=(
"Content-Type: application/json"
"K-namespace: public"
)

# body
BODY="$(cat <<EOB
{
"start_index": 0,
"page_size": 100
}
EOB
)"

