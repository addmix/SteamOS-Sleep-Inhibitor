use zbus::connection::Builder;
use zbus::interface;
use zbus::Proxy;

struct ScreenSaver {
    root_service: Proxy<'static>,
}

impl ScreenSaver {
    fn new(root_service: Proxy<'static>) -> Self {
        Self { root_service }
    }
}

#[interface(name = "org.freedesktop.ScreenSaver")]
impl ScreenSaver {
    async fn inhibit(&self, application: &str, reason: &str,) -> zbus::fdo::Result<u32> { //returns a cookie
        self.root_service
            .call("AddInhibitor", &(application, reason))
            .await
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))
    }

    async fn un_inhibit(&self, cookie: u32) -> zbus::fdo::Result<()> {
        self.root_service
            .call("RemoveInhibitor", &(cookie))
            .await
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))
    }
}

pub async fn run_user_service() -> zbus::Result<()> {
    //create a connection to the user session bus and own the name "org.freedesktop.ScreenSaver" (wip)
    let session_connection = Builder::session()?
    .name("org.freedesktop.ScreenSaver")? //TODO: WARNING: IMPORTANT: make sure the correct name is exposed
    .build()
    .await?;

    // Connect to root helper over system bus
    let system_connection = zbus::Connection::system().await?;

    let root_proxy = Proxy::new(
        &system_connection,
        "com.addmix.SleepInhibitor",
        "/com/addmix/SleepInhibitor",
        "com.addmix.SleepInhibitor",
    )
    .await?;

    let screensaver = ScreenSaver::new(root_proxy);

    //connect the "/org/freedesktop/ScreenSaver" interface to the SleepInhibitor impl
    session_connection.object_server()
    .at("/org/freedesktop/ScreenSaver", screensaver)
    .await?;

    println!("Connected to session bus");

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
