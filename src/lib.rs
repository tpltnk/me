// To suppress WASM generated code warnings.
#![allow(non_upper_case_globals)]

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
use yew::{prelude::*, start_app};


pub struct Root;

impl Component for Root {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <div class="flex justify-center items-center h-screen w-screen bg-gray-900">
        <p class="text-white text-3xl font-signika">{ "Coming soon. 🌴" }</p>
      </div>
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    true
  }
}

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
  start_app::<Root>();
  Ok(())
}
