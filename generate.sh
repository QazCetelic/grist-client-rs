#!/usr/bin/bash

podman run --rm \
	-v ./:/local docker.io/openapitools/openapi-generator-cli generate \
	-i /local/openapi-spec/grist.yml \
	-g rust \
	-o /local/ \
	--skip-validate-spec