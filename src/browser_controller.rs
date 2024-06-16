use thirtyfour::prelude::*;
use tokio;
use std::process::{Child, Command};
use std::time::Duration;
use std::thread;

pub struct BrowserController {
    driver_path: String,
    child: Option<Child>,
    driver: Option<WebDriver>,
}

impl BrowserController {
    pub fn new(driver_path: &str) -> Self {
        Self {
            driver_path: driver_path.to_string(),
            child: None,
            driver: None,
        }
    }

    pub async fn start_driver(&mut self) -> WebDriverResult<()> {
        self.child = Some(
            Command::new(&self.driver_path)
                .arg("--port=9515")
                .spawn()
                .expect("Failed to start ChromeDriver")
        );

        // 잠시 대기하여 ChromeDriver가 시작될 시간을 줍니다.
        thread::sleep(Duration::from_secs(2));

        // WebDriver 초기화
        self.driver = Some(WebDriver::new("http://localhost:9515", DesiredCapabilities::chrome()).await?);

        Ok(())
    }

    pub async fn navigate_to(&self, url: &str) -> WebDriverResult<()> {
        if let Some(driver) = &self.driver {
            driver.goto(url).await?;
        }
        Ok(())
    }

    pub async fn search(&self, query: &str) -> WebDriverResult<()> {
        if let Some(driver) = &self.driver {
            let search_box = driver.find(By::Name("q")).await?;
            search_box.send_keys(query).await?;
            search_box.send_keys(Keys::Enter).await?;
            driver.find(By::Css("h3")).await?;
        }
        Ok(())
    }

    pub async fn quit(&mut self) -> WebDriverResult<()> {
        if let Some(driver) = &self.driver {
            driver.quit().await?;
        }
        if let Some(child) = &mut self.child {
            child.kill().expect("Failed to kill ChromeDriver process");
        }
        Ok(())
    }
}
