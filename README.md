# Yew Router (fork)

This is fork of version 0.15 of the `yew-router`. The main motivation behind the fork is the lack of nested router
support in the 0.16 release.

Also see: https://github.com/yewstack/yew/issues/1853

This fork ports the 0.15 router from Yew 0.18 to Yew 0.19.

## Using

You can use this by patching the `yew-router` dependency in your `Cargo.toml`:

~~~toml
[patch.crates-io]
yew-router = { git = "https://github.com/ctron/yew-router", branch="main" }
~~~

## Migration

While this should mostly be a drop-in replacement for the 0.15 router, there may be a few steps required to migrate:

* When using `Switch` in `Properties`, it may be necessary to also make the types implementing 
  switch `PartialEq` and `Clone`. This is required due to the new requirement of Properties to implement `PartialEq`.
