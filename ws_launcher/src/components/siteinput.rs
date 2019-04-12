use yew::prelude::*;

#[derive(PartialEq)]
pub struct SiteInput {
    url: String,
    onsignal: Option<Callback<String>>
}

pub enum Msg {
    UpdateUrl(String),
    Add,
    None
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub onsignal: Option<Callback<String>>
}

impl Component for SiteInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        SiteInput {
            url: "".to_owned(),
            onsignal: props.onsignal
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateUrl(url) => { self.url = url; true },
            Msg::Add => {
                if let Some(ref mut callback) = self.onsignal {
                    callback.emit(self.url.to_owned());
                    self.url = "".to_owned();
                    true
                }
                else { false }
            },
            Msg::None => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.onsignal = props.onsignal;
        true
    }
}

impl Renderable<Self> for SiteInput {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="add-site",>
                <input class="url", type="text", value=&self.url, onchange=|e| 
                    if let ChangeData::Value(v) = e {
                        Msg::UpdateUrl(v)
                    } else {
                        Msg::None
                    }
                , />
                <input class="add-site", type="button", onclick=|_| Msg::Add, value={"+"}, />
            </div>
        }
    }
}
