#!/bin/bash

set -e -o pipefail

source ../server/Secrets.toml

# Upload static content to s3
s3base="s3://$CW_S3_BUCKET_NAME-www"
function s3c() {
  s3cmd --access_key=$CW_S3_ACCESS_KEY \
    --secret_key=$CW_S3_SECRET_KEY \
    --host="$CW_S3_ENDPOINT" \
    --host-bucket="$CW_S3_BUCKET_STYLE" $@
} 
# Clean bucket
# s3c rm --recursive --force $s3base
# exit 0

# s3 website enable
s3c mb "s3://$CW_S3_BUCKET_NAME-www"
s3c mb "$s3base"
s3c ws-create --ws-index=index.html --ws-error=assets/404.html $s3base
s3c ws-info $s3base
# Upload files
s3c --no-mime-magic --acl-public --delete-removed --delete-after sync ./dist/ "$s3base/"
s3c ls --recursive "$s3base/"
echo "- App URL: $CW_UI_URL"
