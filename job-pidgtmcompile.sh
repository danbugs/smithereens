#!/bin/bash

# Use the PIDGTM_DATABASE_URL environment variable
DATABASE_URL="${PIDGTM_DATABASE_URL}"

# Check if the DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "PIDGTM_DATABASE_URL environment variable is not set. Exiting."
    exit 1
fi

# Read STARTGG_TOKENs from tokens.txt into an array
TOKENS=()
while IFS= read -r line; do
    TOKENS+=("$line")
done < tokens.txt

# Check if tokens.txt is not empty
if [ ${#TOKENS[@]} -eq 0 ]; then
    echo "No tokens found in tokens.txt. Exiting."
    exit 1
fi

# Read the data from id_ranges.txt and store it in arrays
declare -a START_IDS
declare -a END_IDS

while IFS=' |-' read -r job_number start_id end_id; do
    if ! [[ $job_number =~ ^[0-9]+$ ]]; then
        continue
    fi
    START_IDS+=("$start_id")
    END_IDS+=("$end_id")
done < id_ranges.txt

# Function to create secret
create_secret() {
  local job_number=$1
  local token=$2
  kubectl create secret generic pidgtm-secrets-$job_number --from-literal=STARTGG_TOKEN=$token
}

# Function to create job
create_job() {
  local job_number=$1
  local start_id=$2
  local end_id=$3
  local job_definition="apiVersion: batch/v1
kind: Job
metadata:
  name: job-pidgtm-compile-$job_number
spec:
  template:
    metadata:
      labels:
        app: pidgtm
    spec:
      containers:
      - name: pidgtm
        image: danstaken/pidgtm:latest
        imagePullPolicy: Always
        args: [\"compile\", \"$start_id\", \"$end_id\"]
        env:
        - name: STARTGG_TOKEN
          valueFrom:
            secretKeyRef:
              name: pidgtm-secrets-$job_number
              key: STARTGG_TOKEN
        - name: PIDGTM_DATABASE_URL
          value: \"$DATABASE_URL\"
      restartPolicy: OnFailure"
  echo "$job_definition" | kubectl apply -f -
}

# Main loop to create secrets and jobs
for i in "${!START_IDS[@]}"; do
  token_index=$((i % ${#TOKENS[@]}))  # Cycle through tokens
  create_secret "$i" "${TOKENS[$token_index]}"
  create_job "$i" "${START_IDS[$i]}" "${END_IDS[$i]}"
done
