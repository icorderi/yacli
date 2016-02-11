#!/bin/bash

# based on: http://sleepycoders.blogspot.se/2013/03/sharing-travis-ci-generated-files.html
# and: https://gist.github.com/domenic/ec8b0fc8ab45f39403dd

set -e # exit with nonzero exit code if anything fails

# Only do it if not acting on a pull request.
if [ "$TRAVIS_PULL_REQUEST" == "false" ]; then

  # Go to home and setup git
  cd $HOME
  git config --global user.name "Travis CI"
  git config --global user.email "travis@travis-ci.org"

  # Using token clone gh-pages branch. Pipe to /dev/null to avoid printing the decrypted key.
  git clone --quiet --branch=gh-pages https://${GH_TOKEN}@${GH_REPO}  gh-pages > /dev/null 2>&1

  # Go there, and overwrite everything with the freshly built contents.
  mkdir -p gh-pages/doc # in case it's the first time
  cd gh-pages/doc
  rm -rf *
  cp -Rf $TRAVIS_BUILD_DIR/target/doc/* .

  # add, commit and push files
  git add --all -f .
  git commit -m "Travis build $TRAVIS_BUILD_NUMBER docs pushed to gh-pages"
  git push -fq origin gh-pages > /dev/null 2>&1
fi
