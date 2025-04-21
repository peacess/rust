use std::{future::Future, pin::Pin, sync::Arc};

struct App {
    pub count: std::sync::atomic::AtomicI32,
    pub name: String,
}

trait TraitName {
    fn name(&self) -> Pin<Box<dyn Future<Output = String> + Send + '_>>;
}

struct TraitAImpl {
    app: Arc<App>,
}

impl TraitName for TraitAImpl {
    fn name(&self) -> Pin<Box<dyn Future<Output = String> + Send + '_>> {
        Box::pin(async move { self.app.name.clone() })
    }
}

unsafe impl Send for TraitAImpl {}

impl TraitAImpl {
    async fn start(&self) {
        let name = self.name().await;
        println!("name: {}", name);
    }
    // fn start2(&'static self) {
    //     tokio::spawn(async {
    //         let name = self.name().await;
    //         println!("name: {}", name);
    //     });
    // }
}

fn main() {
    let f = || {
        let r = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;

        let app = Arc::new(App {
            count: std::sync::atomic::AtomicI32::new(0),
            name: "tt".to_string(),
        });
        r.block_on(async move {
            let a = Arc::new(TraitAImpl { app: app.clone() });
            // let b = TraitBImpl { app: &*app };
            let a_name = a.name().await;
            println!("name: {}", a_name);

            let t2 = tokio::spawn(async move {
                a.start().await;
                let t = a.name().await;
                println!("name: {}", t);
            });
        });

        Ok::<(), anyhow::Error>(())
    };
    if let Err(e) = f() {
        println!("Error: {}", e);
    }
}
