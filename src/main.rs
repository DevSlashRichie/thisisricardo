use yew::{Component, Context, Html, html};
use gloo::timers::callback::Interval;

enum Msg {
    Tick,
    ListEnd,
    TickCaret,
}

struct Model {
    current_display: String,

    current_word: usize,
    words: Vec<&'static str>,
    current_index: i32,

    threshold_time: i32,
    current_threshold: i32,

    caret: bool,

    interval: Option<Interval>,
}

impl Model {
    /// index exclusive
    fn update_display(&mut self, ctx: &Context<Self>) {
        if self.words.is_empty() {
            return;
        }

        let index = self.current_index as usize;

        let current_word = self.words.get(self.current_word)
            .unwrap();

        if index as usize > current_word.len() {
            if self.current_threshold > 0 {
                self.current_threshold -= 1;

                if self.current_threshold == 0 {
                    self.current_word += 1;
                    self.current_index = 0;

                    if self.current_word == self.words.len() {
                        ctx.link().send_message(Msg::ListEnd);
                    }
                }

                return;
            }

            self.current_threshold = self.threshold_time;
            return;
        }

        if index == 0 {
            self.current_display = String::from("");
        } else {
            self.current_display = current_word[..index].to_string();
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let speed = 100;

        let interval = {
            let link = ctx.link().clone();
            Interval::new(speed, move || {
                link.send_message(Msg::Tick);
            })
        };


        Self {
            interval: Some(interval),

            current_display: "".to_string(),
            words: vec!["Hello There!", "This is Ricardo", "Contact me :)"],

            current_word: 0,
            current_index: 0,

            threshold_time: 8,
            current_threshold: 0,

            caret: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                self.update_display(ctx);
                self.current_index += 1;
            }

            Msg::TickCaret => {
                self.caret = !self.caret;
            }

            Msg::ListEnd => {
                self.interval = None;
                let link = ctx.link().clone();
                Interval::new(600, move || {
                    link.send_message(Msg::TickCaret);
                }).forget();
            }
        }

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <div class="wrapper">
                    <div class="main-bg" />
                    <div class="content">
                    <div class="writer-wrapper">
                        <div class="title">
                            <h1>{ "This is " }<span class="my-name">{"Ricardo"}</span></h1>
                            <span class="downscore" />
                        </div>
                        <div class="writer">
                            <p>{ &self.current_display }{" "}</p>
                            if self.caret {
                                <span class="caret" />
                            } else {
                                <span class="hidden-caret" />
                            }
                        </div>
                    </div>

                    <div class="social">
                        <div><a class="tw" href="https://twitter.com/SiendoRicardo" target="_blank">{"Twitter"}</a></div>
                        <div><a class="lk" href="https://www.linkedin.com/in/ricardo-rodr%C3%ADguez-medina-775a8628/" target="_blank">{"Linkedin"}</a></div>
                        <div><a class="mm" href="https://mymetaverse.io" target="_blank">{"MyMetaverse"}</a></div>
                    </div>
                    </div>


                </div>
                <footer>
                    { "Made with "} <b class="heart">{"❤️"}</b> {" and " }<b class="rust">{"Rust"}</b>
                </footer>
            </main>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
