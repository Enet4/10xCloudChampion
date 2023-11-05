use cloud_champion::central::state::ServiceCounts;
use cloud_champion::components::business::{Business, BusinessProps};
use cloud_champion::components::hardware::Power;
use cloud_champion::components::services::CloudService;
use cloud_champion::{Memory, Money, Ops, ServiceKind};
use yew::prelude::*;

use cloud_champion::components::panel::Panel;
use cloud_champion::{components::card::*, Cost};

#[function_component(Playground)]
pub fn playground() -> Html {
    let business_props = BusinessProps {
        funds: Money::from_dollars(1000),
        base_service: ServiceCounts {
            available: Ops(1000),
            total: Ops(2000),
        },
        super_service: Some(ServiceCounts {
            available: Ops(0),
            total: Ops(2000),
        }),
        epic_service: Some(ServiceCounts::default()),
        ..Default::default()
    };
    html! {
        <>
            <header>
                <div>
                </div>
                <div>
                    <h1>{ "10\u{00d7} Cloud Champion Playground" }</h1>
                    <span class="subtitle">
                        { "A place where I can put lots of random components " }
                        { "to see how they look" }
                    </span>
                </div>
                // empty div to make it even
                <div />
            </header>
            <main>
                <div class="panel-container">
                    <Panel title="Services">
                        <div>
                            <CloudService kind={ServiceKind::Base} />
                            <CloudService kind={ServiceKind::Super} />
                            <CloudService kind={ServiceKind::Epic} />
                        </div>
                    </Panel>
                    <Panel title="Business">
                        <Business ..business_props />
                    </Panel>
                    <Panel title="Hardware">
                        <Power cpu_load={0.3} mem_load={0.5} mem_total={Memory::mb(256)} />

                    </Panel>
                    <Panel title="Cards">
                        <Card
                            id="0"
                            title="New card"
                            description="A test card to give you a welcoming bonus"
                            cost={Cost::dollars(-100)}
                            effect={()}
                            />
                            <Card
                            id="1"
                            title="Unreachable"
                            description="This tests a card which can never be reached"
                            cost={Cost::super_ops(999999)}
                            disabled=true
                            effect={()}
                            />
                    </Panel>
                </div>
            </main>
        </>
    }
}
