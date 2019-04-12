use yew::prelude::*;

use ws_model::site::*;
use ws_model::types::*;

use crate::components::siteline::SiteLine;
use crate::components::siteinput::SiteInput;

/// currently this is main page.
/// When navigation added This should be inserted into nav manager.
pub struct SiteListPage {
    site_list: Vec<Site>
}

pub enum Msg {
    AddSite(Url),
    RemoveSite(Url),
    UpdateSite(Site),
}

impl Component for SiteListPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        SiteListPage {
            site_list: Vec::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::AddSite(url) => {
                self.site_list.push(Site::new(url));
                true
            },
            Msg::RemoveSite(url) => {
                self.site_list.retain(|site| site.url != url);
                true
            },
            _ => false
        }
    }
}

impl Renderable<SiteListPage> for SiteListPage {
    fn view(&self) -> Html<SiteListPage> {
        let sites = self
            .site_list
            .iter()
            .map(|site| html! { <SiteLine: site=site, remove_this=|site: Site| Msg::RemoveSite(site.url), /> });
        
        html! {
            <nav id="sites",>
                <ul>
                    { for sites }
                </ul>
            </nav>
            <SiteInput: onsignal=|url| Msg::AddSite(Url(url)), />
        }
    }
}