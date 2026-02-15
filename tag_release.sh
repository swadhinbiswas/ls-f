#!/bin/bash

# Check if a version argument is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 1.0.0"
    exit 1
fi

VERSION=$1
TAG="v$VERSION"

# Confirm with the user
read -p "This will create and push tag $TAG. Proceed? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

# Create the tag
echo "Creating tag $TAG..."
git tag -a "$TAG" -m "Release $TAG"

# Push the tag
echo "Pushing tag $TAG to origin..."
git push origin "$TAG"

echo "Done! This should trigger the Release workflow on GitHub."
