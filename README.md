# Yew Router (fork)

This is fork of version 0.15 of the `yew-router`.

## Motivation

The main motivation behind the fork is the lack of nested router support in the 0.16 release. This fork ports the 0.15
router from Yew 0.18 to Yew 0.19 and renames it to `yew-router-nested`.

Also see: https://github.com/yewstack/yew/issues/1853

## Using

You can use this by patching the `yew-router` dependency in your `Cargo.toml`:

~~~toml
[dependencies]
yew-router-nested = "0.16"
~~~

If you want to keep `yew_router` as the module name, you can use:

~~~toml
[dependencies]
yew-router = { version = "0.16", package="yew-router-nested" }
~~~

## Migration

While this should mostly be a drop-in replacement for the 0.15 router, there may be a few steps required to migrate:

* The name of the crate is `yew-router-nested`.
* When using `Switch` in `Properties`, it may be necessary to also make the types implementing 
  switch `PartialEq` and `Clone`. This is required due to the new requirement of Properties to implement `PartialEq`.
