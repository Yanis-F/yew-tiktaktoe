use yew::prelude::*;

use crate::tiktaktoe::*;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1>{ "TikTakToe" }</h1>
                <div>
                    <TikTakToe />
                </div>
            </>
        }
    }
}

// Cell: affiche avec 33% de la taille -> bind on_click() pour de vrai
// Grid: affiche ces différentes Cell. Au moment d'afficher, balance juste le callback on_click avec l'id
