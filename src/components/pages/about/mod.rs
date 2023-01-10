use yew::prelude::*;

use crate::components::*;

pub struct About;
impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <main>
          <div
            class="py-40 px-4 sm:px-6 md:px-8 border-b border-secondary-500/40 shadow-2xl"
          >
            <div class="relative max-w-5xl mx-auto">
              <h1
                class="font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center text-white"
              >
                {"About Us"}
              </h1>

              <p class="mt-6 text-lg text-white text-center max-w-3xl mx-auto">
                {"Who are we? what are we about? "}
                <a
                  class="inline text-primary-400 hover:text-accent-400"
                  href="#staff"
                >
                  {"Lets find out!"}
                </a>
              </p>
            </div>
          </div>
          <div class="relative border-b border-secondary-500/40 shadow-2xl">
            <div class="my-32 mx-32">
              <h1 class="inline text-left font-bold text-2xl text-white">
                <a class="font-black text-primary-700">{"Who"}</a>
                {" we are."}
              </h1>
              <div id="staff" class="justify-items-center grid grid-cols-2">
                <div
                  class="py-6 px-4 sm:px-6 md:px-8 dark: border-secondary-600/40"
                >
                  <a
                    href="#"
                    class="flex flex-col items-center bg-white border rounded-lg shadow-md md:flex-row md:max-w-xl hover:bg-primary-100 dark:border-primary-800 dark:bg-primary-800/10 dark:hover:bg-primary-800"
                  >
                    <img
                      class="object-cover w-full rounded-t-lg h-288 w-110 md:h-auto md:w-48 md:rounded-none md:rounded-l-lg"
                      src="images/logo/square/1024.png"
                      alt=""
                    />
                    <div class="flex flex-col justify-between p-4 leading-normal">
                      <div class="mb-2 grid grid-flow-col text-xl text-white font-light ">
                        <p>
                          {"Lorem ipsum dolor sit amet fermentum ut curabitur
                          maecenas facilisis ullamcorper ornare arcu amet dui
                          habitasse placerat suspendisse vulputate nisl."}
                        </p>
                        <p >
                          {"Lorem ipsum dolor sit amet fermentum ut curabitur
                          maecenas facilisis ullamcorper ornare arcu amet dui
                          habitasse placerat suspendisse vulputate nisl."}
                        </p>
                      </div>
                      <figcaption class="align-text-bottom font-medium">
                        <div class="text-slate-300">
                          {"Co-Founder / CEO of Futino"}
                        </div>
                        <div class="text-slate-400">{"Jorge Lewis"}</div>
                      </figcaption>
                    </div>
                  </a>
                </div>
              </div>
            </div>
          </div>


          <div class="relative border-b border-secondary-500/40 shadow-2xl">
            <div class="my-32 mx-32">
              <h1 class="inline text-left font-bold text-2xl text-white">
                <a class="font-black text-primary-700">{"What"}</a>
                {" we do."}
              </h1>
              <h1 class="text-left text-white font-normal text-xl max-w-2xl pt-6">
                {"Futino creates and maintains dynamic web-apps that don't rely on
                proprietary subscription-based solutions. We help growing companies
                and startups to build their presence online with beautiful websites
                and apps that they can fully customise."}
              </h1>
            </div>
          </div>
          <div class="relative border-b border-secondary-500/40 shadow-2xl">
            <div class="my-32 mx-32">
              <h1 class="inline text-left font-bold text-2xl text-white">
                <a class="font-black text-primary-700">{"Why"}</a>
                {" we do."}
              </h1>
              <h1 class="text-left text-white font-normal text-xl max-w-2xl pt-6">
                {"Futino creates and maintains dynamic web-apps that don't rely on
                proprietary subscription-based solutions. We help growing companies
                and startups to build their presence online with beautiful websites
                and apps that they can fully customise."}
              </h1>
            </div>
          </div>
        </main>
        }
    }
}
