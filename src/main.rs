#![deny(clippy::all)]
#![forbid(unsafe_code)]

use eframe::egui;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use tokio::runtime::Runtime;

struct MyApp {
    // Sender/Receiver for async notifications.
    tx: Sender<u32>,
    rx: Receiver<u32>,

    // Silly app state.
    value: u32,
    count: u32,
}

#[derive(Deserialize, Serialize)]
struct HttpbinJson {
    json: Body,
}

#[derive(Deserialize, Serialize)]
struct Body {
    incr: u32,
}

fn main() {
    let rt = Runtime::new().expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    // Execute the runtime in its own thread.
    // The future doesn't have to do anything. In this example, it just sleeps forever.
    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });

    // Run the GUI in the main thread.
    eframe::run_native(
        "Hello egui + tokio",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

impl Default for MyApp {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            tx,
            rx,
            value: 1,
            count: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update the counter with the async response.
        if let Ok(incr) = self.rx.try_recv() {
            self.count += incr;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Press the button to initiate an HTTP request.");
            ui.label("If successful, the count will increase by the following value.");
            ui.add(egui::Slider::new(&mut self.value, 1..=120).text("value"));

            if ui.button(format!("Count: {}", self.count)).clicked() {
                send_req(self.value, self.tx.clone(), ctx.clone());
            }
        });
    }
}

fn send_req(incr: u32, tx: Sender<u32>, ctx: egui::Context) {
    tokio::spawn(async move {
        // Send a request with an increment value.
        let body: HttpbinJson = Client::default()
            .post("https://httpbin.org/anything")
            .json(&Body { incr })
            .send()
            .await
            .expect("Unable to send request")
            .json()
            .await
            .expect("Unable to parse response");

        // After parsing the response, notify the GUI thread of the increment value.
        let _ = tx.send(body.json.incr);
        ctx.request_repaint();
    });
}
