use yew::prelude::*;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="app">
            // <header class="app-header">
                // <a
                //     class="app-logo"
                //     href="https://yew.rs"
                //     target="_blank"
                //     rel="noopener noreferrer"
                // >
                // </a>
                // <p>
                //     { "Edit " } <code>{ "src/routes/home.rs" }</code> { " and save to reload." }
                // </p>
                // <a
                //     id="learn_yew"
                //     class="app-link"
                //     href="https://yew.rs"
                //     target="_blank"
                //     rel="noopener noreferrer"
                // >
                //     { "Learn Yew" }
                // </a>
                // <h1> { "This is a test" } </h1>
            // </header>
            
            <header class="page-title">
                <h1> { "Karl's Kurrencies" } </h1>
            </header>
            <div class="grid-container">
                <section class="about">
                    <div class="grid-item">
                        <h1> { "Welcome to Karl's Kurrencies" } </h1>
                        <p>
                            { "Karl's Kurrencies is a passive cryptocurrency trader. We have developed an algorithm " }
                            { "that will buy cryptocurrency when the value is low and will sell when the value is high. " }
                            { "This service is highly customizable as you have full control over the coin that is traded, " }
                            { "and how much you are willing to buy at a given time. This program currently only supports " }
                            { "Kraken wallets as it is too much work to give full supports across many different wallet APIs; " }
                            { "we are students after all, we do not get paid for this." }
                        </p>
                    </div>
                </section>
                <aside class="login">
                    <div class="grid-item">
                        <h1> { "Login " } </h1>
                        <p> 
                            { "Crate a free account below" }<br/>
                            { "Karl will love you forever" }
                        </p>
                    </div>
                </aside>
            </div>
        </div>
    }
}