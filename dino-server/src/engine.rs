use anyhow::Result;
use axum::{body::Body, response::Response};
use dino_macros::{FromJs, IntoJs};
use rquickjs::{Context, Function, Object, Promise, Runtime};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[allow(unused)]
pub struct JsWorker {
    rt: Runtime,
    ctx: Context,
}

#[derive(Debug, TypedBuilder, IntoJs)]
pub struct Req {
    #[builder(setter(into))]
    pub method: String,
    #[builder(setter(into))]
    pub url: String,
    #[builder(default)]
    pub query: HashMap<String, String>,
    #[builder(default)]
    pub params: HashMap<String, String>,
    #[builder(default)]
    pub headers: HashMap<String, String>,
    #[builder(default)]
    pub body: Option<String>,
}

#[derive(Debug, FromJs)]
pub struct Res {
    pub headers: HashMap<String, String>,
    pub status: u16,
    pub body: Option<String>,
}

#[allow(unused)]
impl JsWorker {
    pub fn try_new(module: &str) -> Result<Self> {
        let rt = Runtime::new()?;
        let ctx = Context::full(&rt)?;

        ctx.with(|ctx| {
            let global = ctx.globals();
            let ret: Object = ctx.eval(module)?;
            global.set("handlers", ret)?;
            // setup print function
            let fun = Function::new(ctx.clone(), print)?.with_name("print")?;
            global.set("print", fun)?;
            Ok::<_, anyhow::Error>(())
        })?;

        Ok(Self { rt, ctx })
    }

    pub fn run(&self, name: &str, req: Req) -> Result<Res> {
        self.ctx.with(|ctx| {
            let global = ctx.globals();
            let handlers: Object = global.get("handlers")?;
            let fun: Function = handlers.get(name)?;
            let v: Promise = fun.call((req,))?;

            Ok::<_, anyhow::Error>(v.finish()?)
        })
    }
}

impl From<Res> for Response {
    fn from(value: Res) -> Self {
        // let mut builder = Response::builder().status(value.status);
        let mut builder = Response::builder();
        for (k, v) in value.headers {
            builder = builder.header(k, v);
        }

        if let Some(body) = value.body {
            builder.body(body.into()).unwrap()
        } else {
            builder.body(Body::empty()).unwrap()
        }
    }
}

fn print(msg: String) {
    println!("{msg}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn js_worker_should_work() -> Result<()> {
        let code = r#"
        (function(){
        async function hello(req){
            return {
                    status:200,
                    headers:{
                        "content-type":"application/json"
                    },
                    body: JSON.stringify(req),
            };
            }
            return{hello:hello};
        })();
        "#;
        let req = Req::builder().method("GET").url("/").build();
        let worker = JsWorker::try_new(code)?;
        let res = worker.run("hello", req)?;

        assert_eq!(res.status, 200);
        assert_eq!(
            res.headers.get("content-type"),
            Some(&"application/json".to_string())
        );
        Ok(())
    }
}
