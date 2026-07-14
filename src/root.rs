use std::collections::HashMap;

use zbus::Connection;
use zbus::connection::Builder;
use zbus::Proxy;
use zbus::interface;
use zbus::zvariant::OwnedFd;


struct SleepInhibitor {
    inhibitors: HashMap<u32, OwnedFd>,
    next_cookie: u32,
}

#[interface(name = "com.addmix.SleepInhibitor")]
impl SleepInhibitor {
    //&self is the dbus object instance
    //application and reason are the program requesting an inhibitor, and the reason why
    //returns a cookie to track the inhibit request
    async fn add_inhibitor(&mut self, application: &str, reason: &str) -> zbus::fdo::Result<u32> {
        let system_connection = Connection::system()
        .await
        .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;

        let login1 = Proxy::new(
            &system_connection,
            "org.freedesktop.login1",
            "/org/freedesktop/login1",
            "org.freedesktop.login1.Manager",
        )
        .await
        .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;

        let reponse = login1
            .call_method("Inhibit", &("sleep", application, reason, "block"))
            .await
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;

        let fd: OwnedFd = reponse.body()
        .deserialize()
        .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;

        let cookie = self.next_cookie;
        self.next_cookie += 1;

        self.inhibitors.insert(cookie, fd);
        println!("Inhibit request from Application: {} Reason: {} Cookie:{}", application, reason, cookie);

        Ok(cookie)
    }

    async fn remove_inhibitor(&mut self, cookie: u32) -> zbus::fdo::Result<()> {
        println!("UnInhibit called. Cookie={}", cookie);
        self.inhibitors.remove(&cookie);

        Ok(())
    }
}

pub async fn run_root_service() -> zbus::Result<()> {
    let system_connection = Builder::system()?
        .name("com.addmix.SleepInhibitor")?
        .build()
        .await?;
    
    let root_service = SleepInhibitor {
        inhibitors: HashMap::new(),
        next_cookie: 1,
    };

    system_connection
        .object_server()
        .at("/com/addmix/SleepInhibitor", root_service)
        .await?;

    println!("Root sleep inhibitor service running");

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}

