use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Team {
    None,
    Cross,
    Circle,
}
impl Team {
    pub fn get_char(&self) -> char {
        match self {
            Team::None   => ' ',
            Team::Cross  => 'x',
            Team::Circle => 'o',
        }
    }
}

pub enum Message {
    PlayMove { x: usize, y: usize },
    PlayAgain,
}

pub struct TikTakToe {
    grid: [[Team; 3]; 3],
    next_turn: Team,
    victory_state: Team,
}

impl Component for TikTakToe {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
    	Default::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::PlayMove { x, y } => self.play_move(x, y),
            Message::PlayAgain         => self.reset()
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div>
                <span>{ format!("Current player:{}", self.next_turn.get_char()) }</span>
                <div class={classes!("grid")}>
                {
                    // That is ugly asf :(

                    (0..3).map(|y| {
                        (0..3).map(|x| {
                        	let x_owned = x;
                        	let y_owned = y;
                            html! {
                            	<div 
                            		class={classes!("gridcell")} 
									onclick={link.callback(move |_| Message::PlayMove { x: x_owned, y: y_owned })}
								>
									{ self.grid[y][x].get_char() }
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()
                    })
                    .map(|r| {
                        html! {
                            <>
                                { for r }
                            </>
                        }
                    })
                    .collect::<Html>()
                }
                </div>
				if self.victory_state != Team::None {
					<span class={classes!("victory-text")}>{ "Player " } <b>{self.victory_state.get_char()}</b> { " won !!!"} </span>
					<button 
						class={classes!("restart-button")}
						onclick={link.callback(move |_| Message::PlayAgain)}
					>{ "Play again" }</button>
				}
            </div>
        }
    }
}

impl Default for TikTakToe {
    fn default() -> Self {
        Self {
            grid          : [[Team::None; 3]; 3],
            next_turn     : Team::Cross,
            victory_state : Team::None,
        }
    }
}

impl TikTakToe {
    fn play_move(&mut self, x: usize, y: usize) {
		if self.victory_state != Team::None {
			return;
		}

        log::info!("Move played in {}, {}", x, y);
        if !(0..=2).contains(&x) || !(0..=2).contains(&y)
		{
			log::error!("Invalid move: y={}, x={}, wtf", y, x);
			return;
		}

		if self.grid[y][x] != Team::None
		{
			log::info!("Tile y={}, x={} is not empty, cannot play here", y, x);
			return;
		}

        self.grid[y][x] = self.next_turn;
        self.next_turn = if self.next_turn == Team::Cross { Team::Circle } else { Team::Cross };

		self.check_for_victory();
    }

    fn check_for_victory(&mut self) {
		// rows
		for y in 0..3 {
			if self.grid[y][0] == self.grid[y][1] && self.grid[y][0] == self.grid[y][2] && self.grid[y][0] != Team::None {
				self.victory_state = self.grid[y][0];
				return;
			}
		}

		// columns
		for x in 0..3 {
			if self.grid[0][x] == self.grid[1][x] && self.grid[0][x] == self.grid[2][x] && self.grid[0][x] != Team::None {
				self.victory_state = self.grid[0][x];
				return;
			}
		}

		// diagonals
		if self.grid[0][0] == self.grid[1][1] && self.grid[0][0] == self.grid[2][2] && self.grid[0][0] != Team::None {
			self.victory_state = self.grid[0][0];
			return;
		}
		if self.grid[2][0] == self.grid[1][1] && self.grid[2][0] == self.grid[0][2] && self.grid[2][0] != Team::None {
			self.victory_state = self.grid[2][0];
		}
	}

	fn reset(&mut self) {
		std::mem::replace(self, Self::default());
	}
}

