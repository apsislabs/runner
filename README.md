`runner` is a simple utility that will run a given command, then run forever and can be signalled to start/stop/restart the given command.

## Motivation

This was mostly written as an exercise to play with rust, but it's also useful in docker-compose environments where you have a server that you may want to restart without shutting down the container.

## Example:

1. `runner serve app sleep 100000` # starts sleeping forever

from a different tab:
1.  `runner stop app` # stops the sleep process
1.  `runner start app` # starts the sleep process
1.  `runner start app` # does nothing, because the sleep process is already running
1.  `runner restart app` # stops and then starts the sleep process

## Building

`cargo build`

## Building Releases

Note: Releases must be built from x86_64-apple-darwin environments.

1. `cargo bump --git-tag`
1. `bin/build-releases`

