#!/bin/bash

login_cl() {
  CL_URL=$1

  username=""
  password=""

  while IFS= read -r line; do
    if [[ "$username" == "" ]]; then
      username=${line}
    else
      password=${line}
    fi
  done <"./secrets/apicredentials"

  echo "" >./cookiefile

  curl -s -c ./cookiefile -d "{\"email\":\"${username}\", \"password\":\"${password}\"}" -X POST -H 'Content-Type: application/json' "$CL_URL/sessions" &>/dev/null
}
