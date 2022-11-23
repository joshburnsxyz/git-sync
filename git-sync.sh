#!/bin/sh

__GIT=$(which git)
__PWD=$(pwd)

[ ! -d "$__PWD/.git" ] && echo "Not a git repo" && exit;

echo "Performing sync for repo $__PWD"

$__GIT pull
$__GIT push
$__GIT push --tags
