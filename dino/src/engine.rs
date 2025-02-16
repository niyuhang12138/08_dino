use anyhow::Result;
use rquickjs::{Context, Function, Object, Runtime};

#[allow(unused)]
pub struct JsWorker {
    rt: Runtime,
    ctx: Context,
}

fn print(msg: String) {
    println!("{msg}");
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

    pub fn run(&self, code: &str) -> Result<()> {
        self.ctx.with(|ctx| -> Result<(), anyhow::Error> {
            ctx.eval_promise(code)?.finish::<()>()?;
            Ok(())
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn js_worker_should_work() -> Result<()> {
        let code = r#"(function(){async function hello(){print("Hello, World!");}return{hello:hello};})();"#;
        let worker = JsWorker::try_new(code)?;
        worker.run("await handlers.hello();")?;
        Ok(())
    }
}
