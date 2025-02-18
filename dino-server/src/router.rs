use crate::{config::ProjectRoutes, AppError};
use arc_swap::ArcSwap;
use axum::http::Method;
use matchit::{Match, Router};
use std::{ops::Deref, sync::Arc};

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct SwappableAppRouter {
    pub inner: Arc<ArcSwap<AppRouterInner>>,
}

#[derive(Clone, Debug)]
pub struct AppRouterInner {
    pub code: String,
    pub routes: Router<MethodRoute>,
}

#[allow(unused)]
#[derive(Clone)]
pub struct AppRouter(Arc<AppRouterInner>);

#[allow(unused)]
#[derive(Clone, Default, Debug)]
pub struct MethodRoute {
    pub get: Option<String>,
    pub post: Option<String>,
    pub put: Option<String>,
    pub delete: Option<String>,
    pub patch: Option<String>,
    pub head: Option<String>,
    pub options: Option<String>,
    pub connect: Option<String>,
    pub trace: Option<String>,
}

#[allow(unused)]
impl SwappableAppRouter {
    pub fn try_new(code: impl Into<String>, routes: ProjectRoutes) -> anyhow::Result<Self> {
        let router = Self::get_router(routes)?;
        Ok(Self {
            inner: Arc::new(ArcSwap::new(Arc::new(AppRouterInner::new(code, router)))),
        })
    }

    pub fn swap(&self, code: impl Into<String>, routes: ProjectRoutes) -> anyhow::Result<()> {
        let router = Self::get_router(routes)?;
        self.inner
            .store(Arc::new(AppRouterInner::new(code, router)));

        Ok(())
    }

    pub fn load(&self) -> AppRouter {
        AppRouter(self.inner.load_full())
    }

    fn get_router(routes: ProjectRoutes) -> anyhow::Result<Router<MethodRoute>> {
        let mut router = Router::new();
        for (path, methods) in routes {
            let mut method_route = MethodRoute::default();
            for method in methods {
                match method.method {
                    Method::GET => method_route.get = Some(method.handler),
                    Method::POST => method_route.post = Some(method.handler),
                    Method::PUT => method_route.put = Some(method.handler),
                    Method::DELETE => method_route.delete = Some(method.handler),
                    Method::PATCH => method_route.patch = Some(method.handler),
                    Method::HEAD => method_route.head = Some(method.handler),
                    Method::OPTIONS => method_route.options = Some(method.handler),
                    Method::CONNECT => method_route.connect = Some(method.handler),
                    Method::TRACE => method_route.trace = Some(method.handler),
                    v => unreachable!("unsupported method {v}"),
                }
            }
            router.insert(path, method_route)?;
        }
        Ok(router)
    }
}

impl AppRouterInner {
    pub fn new(code: impl Into<String>, routes: Router<MethodRoute>) -> Self {
        Self {
            code: code.into(),
            routes,
        }
    }
}

impl Deref for AppRouter {
    type Target = AppRouterInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(unused)]
impl AppRouter {
    #[allow(elided_named_lifetimes)]
    pub fn match_it<'a>(
        &'a self,
        method: Method,
        path: &'a str,
    ) -> Result<Match<&'a str>, AppError> {
        let Ok(ret) = self.routes.at(path) else {
            return Err(AppError::RoutePathNotFound(path.to_string()));
        };
        let s = match method {
            Method::GET => ret.value.get.as_deref(),
            Method::HEAD => ret.value.head.as_deref(),
            Method::DELETE => ret.value.delete.as_deref(),
            Method::OPTIONS => ret.value.options.as_deref(),
            Method::PATCH => ret.value.patch.as_deref(),
            Method::POST => ret.value.post.as_deref(),
            Method::PUT => ret.value.put.as_deref(),
            Method::TRACE => ret.value.trace.as_deref(),
            Method::CONNECT => ret.value.connect.as_deref(),
            _ => unreachable!(),
        }
        .ok_or_else(|| AppError::RouteMethodNotAllowed(method))?;

        Ok(Match {
            value: s,
            params: ret.params,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ProjectConfig;

    #[test]
    fn app_router_match_should_work() {
        let config = include_str!("../fixtures/config.yml");
        let routes: ProjectConfig = serde_yml::from_str(config).unwrap();
        let routes = routes.routes;
        let router = SwappableAppRouter::try_new("", routes).unwrap();
        let app_router = router.load();
        let m = app_router.match_it(Method::GET, "/api/hello/1").unwrap();

        assert_eq!(m.value, "hello");
        assert_eq!(m.params.get("id"), Some("1"));

        let m = app_router.match_it(Method::POST, "/api/abc/1").unwrap();
        assert_eq!(m.value, "hello");
        assert_eq!(m.params.get("name"), Some("abc"));
        assert_eq!(m.params.get("id"), Some("1"));
    }

    #[test]
    fn app_router_swap_should_work() {
        let config = include_str!("../fixtures/config.yml");
        let routes: ProjectConfig = serde_yml::from_str(config).unwrap();
        let routes = routes.routes;
        let router = SwappableAppRouter::try_new("", routes).unwrap();
        let app_router = router.load();

        let m = app_router.match_it(Method::POST, "/api/abc/1").unwrap();
        assert_eq!(m.value, "hello");
        assert_eq!(m.params.get("name"), Some("abc"));
        assert_eq!(m.params.get("id"), Some("1"));

        let new_config = include_str!("../fixtures/config1.yml");
        let new_routes: ProjectConfig = serde_yml::from_str(new_config).unwrap();
        let new_routes = new_routes.routes;
        router.swap("", new_routes).unwrap();
        let app_router = router.load();

        let m = app_router.match_it(Method::POST, "/api/abc/1").unwrap();
        assert_eq!(m.value, "handler2");
        assert_eq!(m.params.get("name"), Some("abc"));
        assert_eq!(m.params.get("id"), Some("1"));
    }
}
