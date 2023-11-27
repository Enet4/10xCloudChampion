use cloud_champion::central::cards::all::ALL_CARDS;
use cloud_champion::central::engine::GameEngine;
use cloud_champion::components::business::{Business, BusinessProps};
use cloud_champion::components::hardware::{Power, Rack};
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                let (can_save, has_save) = match WorldState::has_saved_game() {
                    Ok(true) => (true, true),
                    Ok(false) => (true, false),
                    Err(_) => (false, false),
                };

                html! {
                    <Menu
                        newgame_handler={link.callback(|_| Msg::NewGame)}
                        continuegame_handler={link.callback(|_| Msg::ContinueGame)}
                        {has_save}
                        {can_save}
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
        let state = match ctx.props().origin {
            GameStateOrigin::New => WorldState::default(),
            GameStateOrigin::Continue => {
                // load from local storage
                WorldState::load_game()
                    .expect_throw("Failed to load game state from local storage")
                    .unwrap_or_default()
            }
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

    fn destroy(&mut self, _ctx: &Context<Self>) {
        // try to save before closing
        if let Err(e) = self.state.save_game() {
            gloo_console::error!("Failed to save game state: {:?}", e);
        }
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
            energy_consumption_rate: if self.state.can_see_energy_consumption {
                Some(self.state.electricity.energy_consumption_rate)
            } else {
                None
            },
            request_rates: if self.state.can_see_request_rates {
                Some((self.engine.drop_rate, self.engine.failure_rate))
            } else {
                None
            },
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
            demand: Some(self.state.demand).filter(|_| self.state.can_see_demand),
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

        let powersave = self.state.is_powersaving();
        let nodes = &self.state.nodes;

        let equipment = if nodes.len() <= 4 {
            let link = ctx.link().clone();
            let on_player_action = move |action| link.send_message(action);
            html! {
                <Rack
                    can_buy_nodes={self.state.can_buy_nodes}
                    can_buy_racks={self.state.can_buy_racks}
                    funds={self.state.funds}
                    nodes={nodes.clone()}
                    {powersave}
                    {on_player_action} />
            }
        } else {
            // TODO multiple racks
            let link = ctx.link().clone();
            let on_player_action = move |action| link.send_message(action);
            html! {
                <Rack
                    can_buy_nodes={self.state.can_buy_nodes}
                    can_buy_racks={self.state.can_buy_racks}
                    funds={self.state.funds}
                    nodes={nodes.clone()}
                    {powersave}
                    {on_player_action} />
            }
        };

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
                            {equipment}
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
