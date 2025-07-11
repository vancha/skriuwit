// SPDX-License-Identifier: MPL-2.0

use crate::components::document_store::DocumentStore;
use crate::components::search_engine::DocumentSearchEngine;
use crate::config::Config;
use crate::fl;
use crate::models::document::Document;
use crate::models::tag::Tag;
use crate::styles::{custom_button_style, selected_button_style, tag_button_style };
use cosmic::app::context_drawer;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::{Alignment, Length, Padding, Pixels, Subscription};
use cosmic::prelude::*;
use cosmic::widget::{
    self, Column, Row, button, icon, menu, nav_bar, scrollable, text, text_input,
};
use cosmic::{cosmic_theme, theme};
use std::collections::HashMap;
use std::path::PathBuf;
use std::collections::HashSet;

const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const APP_ICON: &[u8] = include_bytes!("../resources/icons/hicolor/scalable/apps/icon.svg");

/// The application model stores app-specific state used to describe its interface and
/// drive its logic.
pub struct AppModel {
    ///A string meant to hold the state of the text_input used for filtering documents
    search_field_buffer: String,
    /// Application state which is managed by the COSMIC runtime.
    core: cosmic::Core,
    /// Display a context drawer with the designated page if defined.
    context_page: ContextPage,
    /// Contains items assigned to the nav bar panel.
    nav: nav_bar::Model,
    /// Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    // Configuration data that persists between application runs.
    config: Config,
    //The object responsible for filtering and retrieving the documents
    documents: Vec<Document>, //DocumentManager,
    engine: DocumentSearchEngine,
    store: DocumentStore,
    selected_tags:HashSet<Tag>,
    selected_document: Option<Document>,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    //iced/cosmic specific messages
    OpenRepositoryUrl,
    ToggleContextPage(ContextPage),
    UpdateConfig(Config),
    //project specific messages
    SearchFieldInputChanged(String),
    LaunchUrl(String),
    ChooseFile,
    AddFile(Option<PathBuf>),
    TagSelected(Tag),
    DocumentSelected(Document),
}

/// Create a COSMIC application from the app model
impl cosmic::Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::executor::Default;

    /// Data that your application receives to its init method.
    type Flags = ();

    /// Messages which the application and its widgets will emit.
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "com.github.vancha.skriuwit";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        // Create a nav bar with three page items.
        let context_page = ContextPage::default();
        let search_field_buffer = String::from("");
        let key_binds = HashMap::new();
        let mut engine = DocumentSearchEngine::new();
        let store = DocumentStore::new();
        let loaded_documents = store.get_all_documents();
        let loaded_tags = store.get_all_tags();
        let config = cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
            .map(|context| match Config::get_entry(&context) {
                Ok(config) => config,
                Err((_errors, config)) => config,
            })
            .unwrap_or_default();
        let mut nav = nav_bar::Model::default();
        nav.insert().text("Document overview").activate();

        for doc in &loaded_documents {
            engine.add_document(doc.clone());
        }
        let selected_tags = HashSet::new();
        // Construct the app model with the runtime's core.
        let mut app = AppModel {
            core,
            context_page,
            nav,
            search_field_buffer,
            key_binds,
            // Optional configuration file for an application.
            config,
            documents: loaded_documents,
            engine,
            store,
            selected_tags,
            selected_document: None,
        };

        // Create a startup task that sets the window title.
        // Also make sure we start loading our documents from disk on app creation

        let update_title = app.update_title();

        (app, update_title)
    }

    /// Elements to pack at the start of the header bar.
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            //menu::root(fl!("view").into_fragment()), //Does not compile
            Element::from(menu::root(fl!("view"))),
            menu::items(
                &self.key_binds,
                vec![menu::Item::Button(fl!("about"), None, MenuAction::About)],
            ),
        )]);

        vec![menu_bar.into()]
    }

    fn header_center(&self) -> Vec<Element<Self::Message>> {
        vec![text::body("Personal Document Manager").into()]
    }

    /// Enables the COSMIC application to create a nav bar with this model.
    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    /// Display a context drawer if the context page is requested.
    fn context_drawer(&self) -> Option<context_drawer::ContextDrawer<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => context_drawer::context_drawer(
                self.about(),
                Message::ToggleContextPage(ContextPage::About),
            )
            .title(fl!("about")),
        })
    }

    /// Describes the interface based on the current state of the application model.
    ///
    /// Application events will be processed through the view. Any messages emitted by
    /// events received by widgets will be passed to the update method.
    fn view(&self) -> Element<Self::Message> {
        let id : String = "test".to_string();
        Row::from_vec(vec![
            Column::from_vec(
                self.store
                    .get_all_tags()
                    .into_iter()
                    .map(|tag| {
                        button::text(tag.clone().title)
                            .class( if self.selected_tags.contains(&tag) { tag_button_style( tag.get_color() ) } else { cosmic::style::Button::default() } )
                            .width(Length::Fill)
                            .on_press(Message::TagSelected(tag.clone()))
                    })
                    .map(Into::<Element<Message>>::into)
                    .collect::<Vec<_>>(),
            )
            .width(if !self.core.nav_bar_active() {
                Length::Fixed(288.0)
            } else {
                Length::Fixed(0.0)
            })
            .into(),
            Column::from_vec(vec![
                text_input("Search", &self.search_field_buffer)
                    .on_input(Message::SearchFieldInputChanged)
                    .into(),
                button::text("Add Document")
                    .on_press(Message::ChooseFile)
                    .into(),
                scrollable(
                    Column::from_vec(
                        self.documents
                            .iter()
                            .map(|document| {
                                button::custom(document.view())
                                .class(if Some(document) == self.selected_document.as_ref() {  selected_button_style() } else { cosmic::style::Button::default() } )
                                .on_press(Message::DocumentSelected(document.clone()))
                                .into()
                            })
                            .collect::<Vec<_>>(),
                    )
                    .spacing(Pixels::from(20.0))
                    .width(Length::Fill)
                    .height(Length::Shrink),
                )
                .into(),
            ])
            .spacing(Pixels::from(20.0))
            .padding(Padding::from(20))
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .into(),
            /*Column::from_vec(vec![]) // will show selected document
            .spacing(Pixels::from(20.0))
            .padding(Padding::from(20))
            .width(Length::FillPortion(3))
            .height(Length::Fill).into(),*/
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-running async tasks running in the background which
    /// emit messages to the application through a channel. They are started at the
    /// beginning of the application, and persist through its lifetime.
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            // Watch for application configuration changes.
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| {
                    // for why in update.errors {
                    //     tracing::error!(?why, "app config error");
                    // }

                    Message::UpdateConfig(update.config)
                }),
        ])
    }

    /// Handles messages emitted by the application and its widgets.
    ///
    /// Tasks may be returned for asynchronous execution of code in the background
    /// on the application's async runtime.
    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::OpenRepositoryUrl => {
                _ = open::that_detached(REPOSITORY);
            }

            Message::SearchFieldInputChanged(content) => {
                self.search_field_buffer = content;
            }

            Message::TagSelected(tag) => {
                if !self.selected_tags.contains(&tag) {
                    self.selected_tags.insert(tag);
                } else {
                    self.selected_tags.remove(&tag);
                }
                self.engine.filter_by_tags(vec![]);
            }

            Message::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    // Close the context drawer if the toggled context page is the same.
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    // Open the context drawer to display the requested context page.
                    self.context_page = context_page;
                    self.core.window.show_context = true;
                }
            }

            Message::UpdateConfig(config) => {
                self.config = config;
            }

            Message::LaunchUrl(url) => match open::that_detached(&url) {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("failed to open {url:?}: {err}");
                }
            },

            Message::ChooseFile => {
                return cosmic::task::future(async {
                    Message::AddFile(rfd::FileDialog::new().pick_file())
                });
            }

            Message::AddFile(path) => {
                if let Some(path) = path {
                    let doc = Document::new(&path);
                    self.engine.add_document(doc.clone());
                    self.store.upload_document(&doc);
                    if !self.documents.contains(&doc) {
                        self.documents.push(doc);
                    }
                }
            }

            Message::DocumentSelected(doc) => {
                match self.selected_document.as_ref() {
                    Some(current_document) => {
                        if current_document == &doc {
                         self.selected_document =None;
                        } else {
                            self.selected_document = Some(doc);
                        }
                    }
                    None => {
                        self.selected_document = Some(doc);
                    }
                }
            }
        }
        Task::none()
    }

    /// Called when a nav item is selected.
    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<cosmic::Action<Self::Message>> {
        // Activate the page in the model. We probably don't want to automatically active any these at startup
        self.nav.activate(id);
        Task::none()
    }
}

impl AppModel {
    /// The about page for this app.
    pub fn about(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(APP_ICON));

        let title = widget::text::title3(fl!("app-title"));

        let hash = env!("VERGEN_GIT_SHA");
        let short_hash: String = hash.chars().take(7).collect();
        let date = env!("VERGEN_GIT_COMMIT_DATE");

        let link = widget::button::link(REPOSITORY)
            .on_press(Message::OpenRepositoryUrl)
            .padding(0);

        widget::column()
            .push(icon)
            .push(title)
            .push(link)
            .push(
                widget::button::link(fl!(
                    "git-description",
                    hash = short_hash.as_str(),
                    date = date
                ))
                .on_press(Message::LaunchUrl(format!("{REPOSITORY}/commits/{hash}")))
                .padding(0),
            )
            .align_x(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    /// Updates the header and window titles.
    pub fn update_title(&mut self) -> Task<cosmic::Action<Message>> {
        let mut window_title = fl!("app-title");

        if let Some(page) = self.nav.text(self.nav.active()) {
            window_title.push_str(" — ");
            window_title.push_str(page);
        }

        if let Some(id) = self.core.main_window_id() {
            self.set_window_title(window_title, id)
        } else {
            Task::none()
        }
    }
}

/// The context page to display in the context drawer.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
        }
    }
}
