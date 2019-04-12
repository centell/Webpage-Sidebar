use yew::prelude::*;
use ws_model::site::Site;

#[derive(Clone)]
pub struct SiteLine {
    site: Site,
    remove_this: Option<Callback<Site>>,
}

pub enum Msg {
    RemoveMe,
    OpenSite,
}

#[derive(PartialEq, Default, Clone)]
pub struct Props {
    pub site: Site,
    pub remove_this: Option<Callback<Site>>,
    // pub edit_this: Option<Callback<Site>>,
}

impl Component for SiteLine {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        SiteLine {
            site: props.site,
            remove_this: props.remove_this,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::RemoveMe => {
                if let Some(cb) = &self.remove_this {
                    cb.emit(self.site.clone());
                }
                false
            },
            Msg::OpenSite => {
                js!{
                    @(no_return)

                    // open sidebar
                    if (browser.sidebarAction.open !== undefined) { browser.sidebarAction.open(); }

                    // set URL
                    browser.sidebarAction.getPanel({})
                    .then((panel) => {
                        browser.sidebarAction.setPanel({ panel: @{&self.site.url.0} });
                    });
                }
                false
            },
            // currently unreachable
            // _ => false
        }
    }
    
    fn change(&mut self, props: Self::Properties) -> bool {
        if self.site != props.site {
            self.site = props.site;
            true
        }
        else {
            false
        }
        
    }
}

impl Renderable<Self> for SiteLine {
    fn view(&self) -> Html<Self> {
        html! {
            <li>
                // https://github.com/DenisKolodin/yew/issues/152
                <a class="open-site list-item", href="#", onclick=|_| Msg::OpenSite, >
                    { &(self.site.url.value()) }
                </a>
                <input class="remove-site list-item", type={"button"}, value={"X"}, onclick=|_| Msg::RemoveMe, />
            </li>
        }
    }
}