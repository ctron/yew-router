# yew-router-nested
A routing library for the [Yew](https://github.com/yewstack/yew) frontend framework.

**NOTE:** This is a fork of the original `yew-router` (0.15) library. It was ported to `yew` 0.19, in order to keep
the ability to use nested routers. With `yew` 0.19 the `yew-router` (0.16) was re-written, but no longer supported
the nesting of routers. This fork exists to fill the gap until the new router gains the same functionality.

### Example
```rust
#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/profile/{id}"]
    Profile(u32),
    #[to = "/forum{*:rest}"]
    Forum(ForumRoute),
    #[to = "/"]
    Index,
}

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum ForumRoute {
    #[to = "/{subforum}/{thread_slug}"]
    SubForumAndThread{subforum: String, thread_slug: String},
    #[to = "/{subforum}"]
    SubForum{subforum: String},
}

html! {
    <Router<AppRoute, ()>
        render = { Router::render(|switch: AppRoute| {
            match switch {
                AppRoute::Profile(id) => html!{<ProfileComponent id = id/>},
                AppRoute::Index => html!{<IndexComponent/>},
                AppRoute::Forum(forum_route) => html!{<ForumComponent route = forum_route/>},
            }
        }) }
    />
}
```

### How it works
This library works by getting the url location from the browser and uses it to instantiate a type that implements Switch.
Simply using `<a></a>` tags to go to your route will not work out of the box, and are inefficient because the server will return the whole app bundle again at best, and at worst just return a 404 message if the server isn't configured properly.
Using this library's RouteService, RouteAgent, RouterButton, and RouterLink to set the location via `history.push_state()` will change the route without retrieving the whole app again.
#### Server configuration
In order for an external link to your webapp to work, the server must be configured to return the `index.html` file for any GET request that would otherwise return a `404` for any conceivable client-side route.
It can't be a `3xx` redirect to `index.html`, as that will change the url in the browser, causing the routing to fail - it must be a `200` response containing the content of `index.html`.
Once the content of `index.html` loads, it will in turn load the rest of your assets as expected and your app will start, the router will detect the current route, and set your application state accordingly.

If you choose to serve the app from the same server as your api, it is recommended to mount your api under `/api` and mount your assets under `/` and have `/` return the content of `index.html`.

Look at https://webpack.js.org/configuration/dev-server/#devserverhistoryapifallback for info on how to configure a webpack dev server to have this behavior.


### How to Include
You can use the released version by adding these to your dependencies.
```toml
[dependencies]
yew-router-nested = "0.16.0"
yew = "0.19.0"
```

You can also import the package, and keep the module name `yew_router` in your code:
```toml
[dependencies]
yew-router = { version = "0.16.0", package = "yew-router-nested" }
yew = "0.19.0"
```

You can use the in-development version in your project by adding it to your dependencies like so:
```toml
[dependencies]
yew-router = { git = "https://github.com/ctron/yew-router", branch="main" }
yew = {git = "https://github.com/yewstack/yew", branch = "master"}
```


#### Minimum rustc
Currently, this library targets rustc 1.39.0, but development is done on the latest stable release.
This library aims to track Yew`s minimum supported rustc version.

-----
### Contributions/Requests

If you have any questions, suggestions, or want to contribute, please open an Issue or PR and we will get back to you in a timely manner.
