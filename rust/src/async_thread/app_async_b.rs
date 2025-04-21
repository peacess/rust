use std::{future::Future, pin::Pin, sync::Arc};

struct App {
    pub count: std::sync::atomic::AtomicI32,
    pub name: String,
    pub trait_b: std::sync::OnceLock<TraitBImpl>,
}

trait TraitName {
    fn name(&self) -> Pin<Box<dyn Future<Output = String> + Send + '_>>;
}

struct TraitBImpl {
    app: &'static App,
}
impl TraitName for TraitBImpl {
    fn name(&self) -> Pin<Box<dyn Future<Output = String> + Send + '_>> {
        Box::pin(async move { self.app.name.clone() })
    }
}

impl TraitBImpl {
    async fn start(&self) {
        let name = self.name().await;
        println!("name: {}", name);
    }
    fn start2(&'static self) {
        tokio::spawn(async {
            let name = self.name().await;
            println!("name: {}", name);
        });
    }
}

fn main() {
    let f = || {
        let r = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;

        let app = App {
            count: std::sync::atomic::AtomicI32::new(0),
            name: "tt".to_string(),
            trait_b: Default::default(),
        };
        let static_app: &'static App = Box::leak(Box::new(app));
        {
            let b = TraitBImpl { app: static_app };
            static_app.trait_b.set(b);
        }
        r.block_on(async move {
            let app_b = static_app.trait_b.get().unwrap();
            let a_name = app_b.name().await;
            println!("name: {}", a_name);

            let t2 = tokio::spawn(async move {
                app_b.start().await;
                let t = app_b.name().await;
                println!("name: {}", t);
            });
            app_b.start2();
        });

        unsafe {
            Box::from_raw(static_app as *const _ as *mut App);
        }

        Ok::<(), anyhow::Error>(())
    };
    if let Err(e) = f() {
        println!("Error: {}", e);
    }
}
