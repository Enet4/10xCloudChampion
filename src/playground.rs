use cloud_champion::central::cards::all::ALL_CARDS;
use cloud_champion::central::engine::{CloudNode, GameEngine};
use cloud_champion::central::state::ServiceInfo;
use cloud_champion::components::business::{Business, BusinessProps};
use cloud_champion::components::hardware::{Node, Power, Rack};
use cloud_champion::components::services::CloudService;
use cloud_champion::components::total_stats::{TotalStats, TotalStatsProps};
use cloud_champion::{
    CloudUserSpec, GameMsg, GameWatch, Memory, Money, Ops, PlayerAction, ServiceKind, WorldState,
    TIME_UNITS_PER_TICK,
};
use yew::prelude::*;

use cloud_champion::components::card::*;
use cloud_champion::components::panel::Panel;

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
        // whichever we want it to be
        let state = WorldState {
            time: 100,
            funds: Money::dollars(50),
            spent: Money::zero(),
            demand: 1.,
            software_level: 0,
            base_service: ServiceInfo {
                price: Money::dec_cents(1),
                available: Ops(100),
                total: Ops(500),
                unlocked: true,
                private: false,
            },
            super_service: ServiceInfo {
                price: Money::dec_cents(5),
                available: Ops(0),
                total: Ops(20),
                unlocked: true,
                private: false,
            },
            epic_service: ServiceInfo::new_private(Money::cents(2)),
            awesome_service: ServiceInfo::new_locked(Money::cents(50)),
            nodes: vec![CloudNode::new(0)],
            user_specs: vec![
                CloudUserSpec {
                    amount: 1,
                    service: ServiceKind::Base,
                    trial_time: 0,
                    bad: false,
                },
                CloudUserSpec {
                    amount: 1,
                    service: ServiceKind::Super,
                    trial_time: 0,
                    bad: false,
                },
            ],
            ..Default::default()
        };

        let link = ctx.link().clone();
        let mut out = Self {
            state,
            engine: GameEngine::new(link),
            watch: GameWatch::new(),
        };

        let link = ctx.link().clone();
        out.watch
            .start_with(move || link.send_message(GameMsg::Tick));

        out.engine.bootstrap_events(&mut out.state);

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
        let super_service = &self.state.super_service;
        let epic_service = &self.state.epic_service;
        let awesome_service = &self.state.awesome_service;
        let electricity_bill = self.state.electricity.total_due.into_cent_precision();
        let business_props = BusinessProps {
            funds: self.state.funds,
            base_ops_available: self.state.base_service.available,
            super_ops_available: if super_service.unlocked {
                Some(super_service.available)
            } else {
                None
            },
            epic_ops_available: if epic_service.unlocked {
                Some(epic_service.available)
            } else {
                None
            },
            awesome_ops_available: if awesome_service.unlocked {
                Some(awesome_service.available)
            } else {
                None
            },

            electricity_bill,
            can_pay_bill: electricity_bill <= self.state.funds,
            on_pay_bills: {
                let link = ctx.link().clone();
                Callback::from(move |_| link.send_message(PlayerAction::PayElectricityBill))
            },
            client_count: None,
        };

        let base_c = {
            let on_price_change = {
                let link = ctx.link().clone();
                move |new_price: Money| {
                    link.send_message(PlayerAction::ChangePrice {
                        kind: ServiceKind::Base,
                        new_price,
                    });
                }
            };
            let on_op_click = {
                let link = ctx.link().clone();
                let amount = self.state.ops_per_click;
                move |_| {
                    link.send_message(PlayerAction::OpClick {
                        kind: ServiceKind::Base,
                        amount,
                    })
                }
            };

            html! {
                <CloudService
                    kind={ServiceKind::Base}
                    price={self.state.base_service.price}
                    on_click={on_op_click}
                    {on_price_change}
                    />
            }
        };

        let super_c = if super_service.unlocked {
            let on_price_change = {
                let link = ctx.link().clone();
                move |new_price: Money| {
                    link.send_message(PlayerAction::ChangePrice {
                        kind: ServiceKind::Super,
                        new_price,
                    });
                }
            };
            let on_op_click = {
                let link = ctx.link().clone();
                let amount = (self.state.ops_per_click / 10).min(1);
                move |_| {
                    link.send_message(PlayerAction::OpClick {
                        kind: ServiceKind::Super,
                        amount,
                    })
                }
            };

            html! {
                <CloudService
                    kind={ServiceKind::Super}
                    price={super_service.price}
                    on_click={on_op_click}
                    {on_price_change}
                    new={super_service.total == Ops(0)}
                    />
            }
        } else {
            html! {}
        };

        let epic_c = if epic_service.unlocked {
            let on_price_change = {
                let link = ctx.link().clone();
                move |new_price: Money| {
                    link.send_message(PlayerAction::ChangePrice {
                        kind: ServiceKind::Epic,
                        new_price,
                    });
                }
            };
            let on_op_click = {
                let link = ctx.link().clone();
                let amount = (self.state.ops_per_click / 50).max(1);
                move |_| {
                    link.send_message(PlayerAction::OpClick {
                        kind: ServiceKind::Epic,
                        amount,
                    })
                }
            };

            html! {
                <CloudService
                    kind={ServiceKind::Epic}
                    price={epic_service.price}
                    on_click={on_op_click}
                    {on_price_change}
                    new={epic_service.total == Ops(0)}
                    private={true}
                    />
            }
        } else {
            html! {}
        };

        let awesome_c = if awesome_service.unlocked {
            let on_price_change = {
                let link = ctx.link().clone();
                move |new_price: Money| {
                    link.send_message(PlayerAction::ChangePrice {
                        kind: ServiceKind::Awesome,
                        new_price,
                    });
                }
            };
            let on_op_click = {
                let link = ctx.link().clone();
                let amount = (self.state.ops_per_click / 1_000).max(1);
                move |_| {
                    link.send_message(PlayerAction::OpClick {
                        kind: ServiceKind::Awesome,
                        amount,
                    })
                }
            };

            html! {
                <CloudService
                    kind={ServiceKind::Epic}
                    price={epic_service.price}
                    on_click={on_op_click}
                    {on_price_change}
                    new={epic_service.total == Ops(0)}
                    private={true}
                    />
            }
        } else {
            html! {}
        };

        // Total stats header: all op counts
        let total_stats_props = TotalStatsProps {
            base_ops_total: self.state.base_service.total,
            super_ops_total: if super_service.unlocked {
                Some(super_service.total)
            } else {
                None
            },
            epic_ops_total: if epic_service.unlocked {
                Some(epic_service.total)
            } else {
                None
            },
            awesome_ops_total: if awesome_service.unlocked {
                Some(awesome_service.total)
            } else {
                None
            },
        };

        // Projects panel: cards
        let all_cards = ALL_CARDS;

        let cards: Html = all_cards
            .iter()
            .filter(|card| {
                // should not be a used card
                !self.state.is_card_used(card.id)
                // condition of appearance is fulfilled
                    && card.condition.should_appear(&self.state)
            })
            .map(|card| {
                let link = ctx.link().clone();
                let cost = card.cost.clone();
                let disabled = !self.state.can_afford(&cost);
                let id = card.id;
                html! {
                    <Card
                        {id}
                        title={card.title}
                        description={card.description}
                        {cost}
                        {disabled}
                        on_click={move |_| link.send_message(PlayerAction::UseCard { id: id.into() })}
                        />
                }
            })
            .collect();

        // Hardware panel: power stats and cloud nodes

        let (cpu_load, mem_load) = self.state.total_processing();
        let mem_total: Memory = self.state.nodes.iter().map(|n| n.ram_capacity).sum();

        let nodes: Html = self
            .state
            .nodes
            .iter()
            .map(|node| {
                let cpu_upgrade_cost = node.next_cpu_upgrade_cost();
                let ram_upgrade_cost = node.next_ram_upgrade_cost();
                let cpu_upgrade_disabled = cpu_upgrade_cost
                    .map(|cost| self.state.funds < cost)
                    .unwrap_or_default();
                let ram_upgrade_disabled = ram_upgrade_cost
                    .map(|cost| self.state.funds < cost)
                    .unwrap_or_default();
                let on_cpu_upgrade = {
                    let link = ctx.link().clone();
                    let node = node.id;
                    move |_| link.send_message(PlayerAction::UpgradeCpu { node })
                };
                let on_ram_upgrade = {
                    let link = ctx.link().clone();
                    let node = node.id;
                    move |_| link.send_message(PlayerAction::UpgradeRam { node })
                };
                html! {
                    <Node
                        cpus={node.num_cores} ram={Memory::mb(256)}
                        {cpu_upgrade_cost}
                        {ram_upgrade_cost}
                        {cpu_upgrade_disabled}
                        {ram_upgrade_disabled}
                        {on_cpu_upgrade}
                        {on_ram_upgrade}
                        />
                }
            })
            .collect();

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
                                {awesome_c}
                            </div>
                        </Panel>
                        <Panel title="Business">
                            <Business ..business_props />
                        </Panel>
                        <Panel title="Hardware">
                            <Power {cpu_load} {mem_load} {mem_total} />
                            <Rack>
                                {nodes}
                            </Rack>
                        </Panel>
                        <Panel title="Projects" classes={classes!["projects"]}>
                            {cards}
                        </Panel>
                    </div>
                </main>
            </>
        }
    }
}
