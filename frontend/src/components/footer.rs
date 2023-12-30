use yew::{function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="footer fixed-bottom" style="background-color:#D2D2D2;">
            <div class="container" style="color:#C6263E;">
                <center>
                    <small style="font-size: 0.8rem;">
                        {"Made with "}
                        // icon for made with love
                        <img src="https://img.icons8.com/ios/50/000000/like--v1.png" alt="Made with love" class="img-fluid" style="max-width:16px"/>
                        {", "}
                        // icon for rust
                        <img src="https://devicons.railway.app/i/rust-dark.svg" alt="Rust" class="img-fluid" style="max-width:16px"/>
                        {", and "}
                        // icon for webassembly
                        <img src="https://devicons.railway.app/i/wasm.svg" alt="WebAssembly" class="img-fluid" style="max-width:16px"/>
                        // by
                        {" by "}
                        // link to my twitter
                        <a class="link-danger" target="_blank" href="https://twitter.com/dantotto">
                        {"@dantotto"}
                        </a>
                        {"."}
                    </small>
                    // in a new line mention that the code is open-source and link its' repository
                    <br/>
                    <small style="font-size: 0.8rem;">
                        {"This website's code is open-source and can be found "}
                        <a class="link-danger" target="_blank" href="https://github.com/danbugs/smithereens">
                            {"here"}
                        </a>
                        {"."}
                    </small>
                </center>
            </div>
        </div>
    }
}
