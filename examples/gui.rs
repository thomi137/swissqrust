/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use std::path::PathBuf;
use iced::widget::{text, column, center, button, pick_list, row};
use iced::{Center, Element, Task, Theme, Length, window};

use swiss_qrust::{create_pdf, BillData, InputBill, Language};

#[derive(Default)]
struct SwissQrApp {
    status_message: String,
    selected_language: Language,
}

#[derive(Debug, Clone)]
enum Message {
    OpenPicker,
    LanguageChanged(Language),
    FileSelected(Option<PathBuf>),
    PdfGenerated(Result<PathBuf, String>),
}
impl SwissQrApp {
    // Logic remains mostly the same, but no longer tied to a trait
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LanguageChanged(lang) => {
                self.selected_language = lang;
                Task::none()
            },
            Message::OpenPicker => {
                Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .add_filter("Config Files", &["toml", "json"])
                            .set_title("Select Bill Configuration")
                            .pick_file()
                            .await
                            .map(|handle| handle.path().to_path_buf())
                    },
                    Message::FileSelected,
                )
            },

            Message::FileSelected(Some(path)) => {
                self.status_message = format!("Processing: {:?}", path.file_name().unwrap());
                // Task to run your PDF logic in the background
                Task::perform(generate_pdf(path, self.selected_language), Message::PdfGenerated)
            },

            Message::FileSelected(None) => {
                self.status_message = "Selection cancelled.".into();
                Task::none()
            },

            Message::PdfGenerated(Ok(saved_path)) => {
                self.status_message = format!("Success! Saved to: {:?}", saved_path);
                Task::none()
            }

            Message::PdfGenerated(Err(e)) => {
                self.status_message = format!("Error: {}", e);
                Task::none()
            }

        }
    }

    fn view(&self) -> Element<'_, Message> {

        let languages: &[Language] = &[
            Language::De,
            Language::Fr,
            Language::It,
            Language::En
        ];

        center(
            column![
            text("Swiss QR Bill Tool").size(40),
            row![
                    text("Language:"),
                    pick_list(languages, Some(&self.selected_language), Message::LanguageChanged)
                    .placeholder("Select language".to_string())
                    .width(Length::Shrink),
                ].spacing(10).align_y(Center),
            button("Pick .toml or .json file").on_press(Message::OpenPicker),
            text(&self.status_message),
        ]
                .spacing(20)
                .align_x(Center)
        ).into()
    }
}

fn main() -> iced::Result {
    // Instead of Sandbox::run, we use the application builder
    iced::application(SwissQrApp::default, SwissQrApp::update, SwissQrApp::view)
        .title("Swiss QR Bill GUI")
        .window(window::Settings {
            size: (500.0, 400.0).into(), // Custom window size
            ..Default::default()
        })
        .theme(Theme::TokyoNight)
        .run()
}

async fn generate_pdf(input_path: PathBuf, language: Language) -> Result<PathBuf, String> {
    // Calling a dedicated private helper keeps the async wrapper clean
    let output_path = input_path.with_extension("pdf");
    process_bill(&input_path, &output_path, language).map_err(|e| format!("{e:#}"))?;
    Ok(output_path)
}

fn process_bill(in_path: &PathBuf, out_path: &PathBuf, language: Language) -> anyhow::Result<()> {

    let out_path_str = out_path.to_str().ok_or_else(|| anyhow::anyhow!("Invalid output path"))?;

    let content = std::fs::read_to_string(in_path)?;
    let input_bill: InputBill = match in_path.extension().and_then(|s| s.to_str()) {
        Some("toml") => toml::from_str(&content)?,
        Some("json") => serde_json::from_str(&content)?,
        _ => anyhow::bail!("Use .toml or .json"),
    };

    let bill_data: BillData = input_bill.try_into()?;

    create_pdf(out_path_str, language, &bill_data)?;
    Ok(())
}

