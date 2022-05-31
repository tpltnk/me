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
      <div class="grid grid-rows-3 h-screen w-screen bg-gray-900">
        <div class="flex h-[1fr] w-screen"></div>
        <div class="flex justify-center items-center h-full w-screen">

          /*  after:content-[''] */
          <p
            class=r#"
            text-white
            text-3xl
            font-signika
            inline-block
            after:block
            after:border-b-2
            after:w-0
            after:duration-150
            after:ease-out
            hover:after:w-full
            "#
          >
            { "In development. 🌴" }
          </p>
        </div>
        <div class="flex flex-row mt-auto justify-between items-center p-7 h-[1fr] w-screen">
          <div class="flex flex-row gap-2">
            <a class="text-white" href="https://github.com/tpltnk">
              <span class="fa-xl fa-brands fa-github-square"></span>
            </a>
            /*
            <a class="text-white" href="https://tpltnk.github.io/blog">
              <span class="fa-xl fa-solid fa-square-rss"></span>
            </a>
            */
            <a class="text-white" href="https://www.youtube.com/channel/UCbu4FIogkvz4UbQ0VjWo7LQ">
              <span class="fa-xl fa-brands fa-youtube-square"></span>
            </a>
          </div>
          <div class="flex flex-row gap-2">
            <a class="text-white" href="https://discordapp.com/users/978319857307230248">
              <span class="fa-xl fa-brands fa-discord"></span>
            </a>
            <a class="text-white" href="https://reddit.com/user/kenpaicat">
              <span class="fa-xl fa-brands fa-reddit-square"></span>
            </a>
            <a class="text-white" href="https://twitter.com/tpltnk">
              <span class="fa-xl fa-brands fa-twitter-square"></span>
            </a>
          </div>
        </div>
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
