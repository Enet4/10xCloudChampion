use cloud_champion::central::engine::GameEngine;
use cloud_champion::components::business::{Business, BusinessProps};
use cloud_champion::components::hardware::{Node, Power, Rack};
use cloud_champion::components::services::CloudService;
use cloud_champion::components::total_stats::{TotalStats, TotalStatsProps};
use cloud_champion::{
    GameMsg, GameWatch, Memory, Money, Ops, ServiceKind, UserAction, WorldState,
    TIME_UNITS_PER_TICK,
};
use yew::prelude::*;

use cloud_champion::components::panel::Panel;
use cloud_champion::{components::card::*, Cost};

#[derive(Debug)]
pub(crate) struct Playground {
    state: WorldState,
    engine: GameEngine,
    watch: GameWatch,
}

impl Component for Playground {
    type Message = GameMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut out = Self {
            state: WorldState::default(),
            engine: GameEngine::new(),
            watch: GameWatch::new(),
        };

        out.state.funds = Money::dollars(100);

        let link = ctx.link().clone();
        out.watch
            .start_with(move || link.send_message(GameMsg::Tick));

        out
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameMsg::Action(action) => {
                gloo_console::debug!(format!("Received action: {:?}", action));
                self.engine.apply_action(&mut self.state, action);
                true
            }
            GameMsg::Tick => {
                let time = self.state.time + TIME_UNITS_PER_TICK as u64;
                self.engine.update(&mut self.state, time);
                true
            }
            GameMsg::Pause => {
                self.watch.stop();
                true
            }
            GameMsg::Resume => {
                let link = ctx.link().clone();
                self.watch
                    .start_with(move || link.send_message(GameMsg::Tick));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let business_props = BusinessProps {
            funds: self.state.funds,
            base_ops_available: self.state.base_service.available,
            super_ops_available: Some(Ops(1000)),
            epic_ops_available: Some(Ops(1000)),
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

        let total_stats_props = TotalStatsProps {
            base_ops_total: Ops(1000),
            super_ops_total: Some(Ops(5000)),
            epic_ops_total: Some(Ops(0)),
            awesome_ops_total: None,
        };

        let mut cards = vec![];
        let link = ctx.link().clone();
        cards.push(html! {
            <Card
                id="test-0"
                title="New card"
                description="A test card to give you a welcoming bonus"
                cost={Cost::nothing()}
                on_click={move |_| link.send_message(UserAction::UseCard { id: "test-0".into() }) }
                />
        });

        let link = ctx.link().clone();
        cards.push(html! {
            <Card
                id="test-1"
                title="Powerup"
                description="Test improving your services"
                cost={Cost::base_ops(500)}
                on_click={move |_| link.send_message(UserAction::UseCard { id: "test-1".into() }) }
                />
        });
        let link = ctx.link().clone();
        cards.push(html! {
            <Card
                id="test-2"
                title="Unreachable"
                description="This tests a card which can never be reached"
                cost={Cost::super_ops(100_000)}
                disabled={true}
                on_click={move |_| link.send_message(UserAction::UseCard { id: "test-2".into() }) }
                />
        });

        html! {
            <>
                <header>
                    <TotalStats ..total_stats_props />
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
                            {cards}
                        </Panel>
                    </div>
                </main>
            </>
        }
    }
}
