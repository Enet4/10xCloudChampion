use cloud_champion::central::state::ServiceCounts;
use cloud_champion::components::business::{Business, BusinessProps};
use cloud_champion::components::hardware::{Node, Power, Rack};
use cloud_champion::components::services::CloudService;
use cloud_champion::components::total_stats::TotalStats;
use cloud_champion::{Memory, Money, Ops, ServiceKind};
use yew::prelude::*;

use cloud_champion::components::panel::Panel;
use cloud_champion::{components::card::*, Cost};

#[function_component(Playground)]
pub fn playground() -> Html {
    let base_service = ServiceCounts {
        available: Ops(1000),
        total: Ops(2000),
    };
    let super_service = Some(ServiceCounts {
        available: Ops(0),
        total: Ops(2000),
    });
    let epic_service = Some(ServiceCounts::default());

    let business_props = BusinessProps {
        funds: Money::dollars(1000),
        base_service,
        super_service,
        epic_service,
        ..Default::default()
    };

    let base_c = html! {
        <CloudService
            kind={ServiceKind::Base}
            price={Money::millicents(5)}
            on_click={|_| ()}
            on_price_change={|_| ()}
            />
    };

    let super_c = html! {
        <CloudService
            kind={ServiceKind::Super}
            price={Money::millicents(50)}
            on_click={|_| ()}
            on_price_change={|_| ()}
            />
    };

    let epic_c = html! {
        <CloudService
            kind={ServiceKind::Epic}
            price={Money::cents(2)}
            on_click={|_| ()}
            on_price_change={|_| ()}
            />
    };

    html! {
        <>
            <header>
                <TotalStats base_service={base_service} super_service={super_service} epic_service={epic_service} />
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
                            {base_c}
                            {super_c}
                            {epic_c}
                        </div>
                    </Panel>
                    <Panel title="Business">
                        <Business ..business_props />
                    </Panel>
                    <Panel title="Hardware">
                        <Power cpu_load={0.3} mem_load={0.5} mem_total={Memory::mb(256)} />
                        <Rack>
                            <Node cpus={1} ram={Memory::mb(256)} />
                        </Rack>
                    </Panel>
                    <Panel title="Projects">
                        <Card
                            id="0"
                            title="New card"
                            description="A test card to give you a welcoming bonus"
                            cost={Cost::nothing()}
                            />
                        <Card
                            id="1"
                            title="Powerup"
                            description="Test improving your services"
                            cost={Cost::base_ops(500)}
                            />
                        <Card
                            id="1"
                            title="Unreachable"
                            description="This tests a card which can never be reached"
                            cost={Cost::super_ops(100_000)}
                            disabled=true
                            />
                    </Panel>
                </div>
            </main>
        </>
    }
}
