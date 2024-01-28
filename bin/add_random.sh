#!/bin/bash

set -e

rstring=$(tr -dc A-Za-z0-9 </dev/urandom | head -c 16)
nextweek=$(date +%s --date="next week")

if [ -z "$1" ]; then
  curlhost="localhost"
else
  curlhost="$1"
fi

if [ ! -z '$2' ]; then
  rstring=$2
fi

json=$(cat <<EOF
{
  "code": "$rstring",
  "expires_at": $nextweek,
  "creator_name": "cgen-$rstring",
  "creator_url": "https://cgen-$rstring.foo",
  "submitter_name": "sgen-$rstring",
  "submitter_url": "https://sgen-$rstring.bar"
}
EOF
)

echo "Request:"
echo "$json" | jq

response=$(
curl -s -X PUT http://$curlhost:8000/v1/codes \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json' \
  -H 'X-Api-Key: common_api_key' \
  --data "$json")

echo "Response:"
echo "$response" | jq
