use std::collections::HashMap;
use typed_builder::TypedBuilder;

use anyhow::Result;
use rquickjs::{Context, FromJs, Function, IntoJs, Object, Promise, Runtime, Value};

#[allow(unused)]
pub struct JsWorker {
    rt: Runtime,
    ctx: Context,
}

#[derive(Debug, TypedBuilder)]
pub struct Request {
    #[builder(default)]
    pub headers: HashMap<String, String>,
    #[builder(setter(into))]
    pub method: String,
    #[builder(setter(into))]
    pub url: String,
    #[builder(default, setter(strip_option))]
    pub body: Option<String>,
}

#[derive(Debug)]
pub struct Response {
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

    pub fn run(&self, name: &str, req: Request) -> Result<Response> {
        self.ctx.with(|ctx| {
            let global = ctx.globals();
            let handlers: Object = global.get("handlers")?;
            let fun: Function = handlers.get(name)?;
            let v: Promise = fun.call((req,))?;

            Ok::<_, anyhow::Error>(v.finish()?)
        })
    }
}

impl<'js> IntoJs<'js> for Request {
    fn into_js(self, ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<rquickjs::Value<'js>> {
        let obj = Object::new(ctx.clone())?;

        obj.set("headers", self.headers.into_js(ctx)?)?;
        obj.set("method", self.method.into_js(ctx)?)?;
        obj.set("url", self.url.into_js(ctx)?)?;
        obj.set("body", self.body.into_js(ctx)?)?;

        Ok(obj.into())
    }
}

impl<'js> FromJs<'js> for Response {
    fn from_js(ctx: &rquickjs::Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        let obj = Object::from_js(ctx, value)?;

        let headers: HashMap<String, String> = obj.get("headers")?;
        let status: u16 = obj.get("status")?;
        let body: Option<String> = obj.get("body")?;

        Ok(Response {
            headers,
            status,
            body,
        })
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
        let req = Request::builder().method("GET").url("/").build();
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
