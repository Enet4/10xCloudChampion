use cloud_champion::central::cards::all::ALL_CARDS;
use cloud_champion::central::engine::GameEngine;
use cloud_champion::components::business::{Business, BusinessProps};
use cloud_champion::components::hardware::{Node, Power, Rack};
use cloud_champion::components::menu::Menu;
use cloud_champion::components::services::CloudService;
use cloud_champion::components::total_stats::{TotalStats, TotalStatsProps};
use cloud_champion::{
    GameMsg, GameWatch, Memory, Money, Ops, PlayerAction, ServiceKind, WorldState,
    TIME_UNITS_PER_CYCLE,
};
use js_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

use cloud_champion::components::card::*;
use cloud_champion::components::panel::Panel;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Msg {
    NewGame,
    ContinueGame,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) enum AppState {
    /// The player is in the main menu
    #[default]
    MainMenu,
    /// A game is ongoing
    Game(GameStateOrigin),
}

#[derive(Debug)]
pub(crate) struct App {
    state: AppState,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: AppState::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewGame => {
                self.state = AppState::Game(GameStateOrigin::New);
                true
            }
            Msg::ContinueGame => {
                self.state = AppState::Game(GameStateOrigin::Continue);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.state {
            AppState::MainMenu => {
                let link = ctx.link().clone();
                let has_save = WorldState::has_saved_game().map(|_| true).unwrap_or(false);
                html! {
                    <Menu
                        newgame_handler={link.callback(|_| Msg::NewGame)}
                        continuegame_handler={link.callback(|_| Msg::ContinueGame)}
                        {has_save}
                        />
                }
            }
            AppState::Game(origin) => {
                html! {
                    <Game origin={*origin} />
                }
            }
        }
    }
}

/// The top level application state
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum GameStateOrigin {
    /// The player initiated a new game
    New,
    /// A game is being continued from a saved state
    Continue,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub(crate) struct GameProps {
    origin: GameStateOrigin,
}

#[derive(Debug)]
pub(crate) struct Game {
    state: WorldState,
    engine: GameEngine<Game>,
    watch: GameWatch,
}

impl Component for Game {
    type Message = GameMsg;
    type Properties = GameProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut state = match ctx.props().origin {
            GameStateOrigin::New => WorldState::default(),
            GameStateOrigin::Continue => {
                // load from local storage
                WorldState::load_game()
                    .expect_throw("Failed to load game state from local storage")
                    .unwrap_or_default()
            }
        };

        // reset CPU load and waiting requests
        // because request queue is not saved
        for node in state.nodes.iter_mut() {
            node.processing = 0;
            node.ram_reserved = Memory::zero();
            node.ram_usage = Memory::zero();
            node.requests.clear();
        }

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
                let time = self.state.time + TIME_UNITS_PER_CYCLE as u64;
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
        // gather service state
        let super_service = &self.state.super_service;
        let epic_service = &self.state.epic_service;
        let awesome_service = &self.state.awesome_service;

        // business panel: stats & electricity bills
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

        // service panel: cloud services
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
                    new={self.state.base_service.total == Ops(0)}
                    private={self.state.base_service.private}
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
                let amount = self.state.ops_per_click;
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
                    private={super_service.private}
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
                let amount = self.state.ops_per_click;
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
                    private={epic_service.private}
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
                let amount = self.state.ops_per_click;
                move |_| {
                    link.send_message(PlayerAction::OpClick {
                        kind: ServiceKind::Awesome,
                        amount,
                    })
                }
            };

            html! {
                <CloudService
                    kind={ServiceKind::Awesome}
                    price={awesome_service.price}
                    on_click={on_op_click}
                    {on_price_change}
                    new={awesome_service.total == Ops(0)}
                    private={awesome_service.private}
                    />
            }
        } else {
            html! {}
        };

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

        let all_cards = ALL_CARDS;

        let cards: Html = all_cards
            .iter()
            .filter(|card| card.should_appear(&self.state))
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
                        cpus={node.num_cores} ram={node.ram_capacity}
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
                        <h1>{ "10\u{00d7} Cloud Champion" }</h1>
                        <span class="subtitle"></span>
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
