use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    hooks::use_navigate,
    StaticSegment,
};
use radix_leptos::Button as RadixButton;
use leptos_forms_rs::components::*;

use crate::auth::{
    answer_nonce, clear_token, create_identity, docs_content, gate_check, grant_access,
    pending_list, read_settings, read_token, request_nonce, revoke_access, save_settings,
    sign_out, store_token, VisitorSettings,
};
use crate::store::{registry_rows, ModelRow};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0, viewport-fit=cover"/>
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link href="https://fonts.googleapis.com/css2?family=Inter:opsz,wght@14..32,300;14..32,400;14..32,500;14..32,600;14..32,700&display=swap" rel="stylesheet"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

const CARDS: [(&str, &str); 8] = [
    ("ExoTalk", "The hosting platform for the Exosystem. All apps unfold from within it."),
    ("ExoSoul", "The authoring system. Directives authored here drive every app's UI."),
    ("Conscia", "Node connectivity and governance surface — discovery, capabilities, and petitions."),
    ("Exonomy", "P2P economy with voucher-based exchange. Communities create and trade value independently of traditional banks."),
    ("Exocracy", "Community project management. Exocrats collaborate on large-scale initiatives using Exonomy vouchers."),
    ("RepubLet", "Cold storage for the Exosystem — secure, decentralized data persistence."),
    ("ThreeSteps (3S)", "Decentralized, privacy-first social media for Catholics. P2P, local-first, offline-capable — supporting faith formation through Baptism, First Communion, and Confirmation."),
    ("InterNovel", "Collaborative writing platform for young creators. DID-based identity ensures clear attribution and IP ownership. Features Character Travel Trunk, Novel Spine, Casting Studio, and Novel Café."),
];

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/exosystems_net.css"/>
        <Title text="ExoSystems"/>
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("projects") view=HomePage/>
                    <Route path=StaticSegment("signin") view=SigninPage/>
                    <Route path=StaticSegment("docs") view=DocsPage/>
                    <Route path=StaticSegment("models") view=ModelsPage/>
                    <Route path=StaticSegment("settings") view=SettingsPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Home keeps the carousel with the gadgetry overlaid on top of it.
#[component]
fn HomePage() -> impl IntoView {
    let gate = Resource::new(
        || (),
        |_| async move {
            match read_token() {
                Some(t) => gate_check(t).await.unwrap_or(None),
                None => None,
            }
        },
    );
    view! {
        <Suspense fallback=|| view! { <nav class="navbar"><span class="navbar-brand">"ExoSystems"</span></nav> }>
            {move || gate.get().map(|g| {
                let show = g.map(|v| v.model_updates).unwrap_or(false);
                view! {
                    <header class="masthead">
                        <div class="slide slide-1"></div>
                        <div class="slide slide-2"></div>
                        <nav class="navbar">
                            <img src="/exosys.png" alt="ExoSystems" class="navbar-logo"/>
                            <div class="navbar-menu">
                                <A href="/">"Home"</A>
                                {show.then(|| view! { <A href="/models">"Models"</A> })}
                                <A href="/settings">"Settings"</A>
                                <A href="/signin">"Sign in"</A>
                            </div>
                        </nav>
                        <section class="hero">
                            <span class="hero-brand">"ExoSystems"</span>
                            <div class="tagline-block">
                                <p>"Hello, world."</p>
                                <p>"Private by design. Connected by nature."</p>
                            </div>
                        </section>
                    </header>
                    <main class="main">
                        <section class="family">
                            {CARDS
                                .iter()
                                .map(|(title, body)| {
                                    view! {
                                        <div class="family-card">
                                            <h3>{*title}</h3>
                                            <p>{*body}</p>
                                        </div>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </section>
                    </main>
                }
            })}
        </Suspense>
    }
}

/// The models page renders its own top navigation bar carrying the gadgetry
/// with no carousel. Sign-in runs only on the main page; this route
/// recognizes the main-page session instead of running its own sign-in.
#[component]
fn ModelsNavbar(show_models: bool) -> impl IntoView {
    view! {
        <nav class="navbar">
            <span class="navbar-brand">"Model Updates"</span>
            <div class="navbar-menu">
                <A href="/">"Home"</A>
                {show_models.then(|| view! { <A href="/models">"Models"</A> })}
                <A href="/settings">"Settings"</A>
                <A href="/signin">"Sign in"</A>
            </div>
        </nav>
    }
}

/// Sign the open challenge with the identity secret, inside the browser.
/// The secret never leaves the device.
#[cfg(target_arch = "wasm32")]
fn sign_challenge(secret_b58: &str, nonce: &str) -> Result<String, String> {
    let sk_bytes = bs58::decode(secret_b58.trim()).into_vec().map_err(|_| "Secret does not decode.".to_string())?;
    let sk_arr: [u8; 32] = sk_bytes.try_into().map_err(|_| "Secret is not 32 bytes.".to_string())?;
    let signing_key = ed25519_dalek::SigningKey::from_bytes(&sk_arr);
    use ed25519_dalek::Signer;
    Ok(bs58::encode(signing_key.sign(nonce.as_bytes()).to_bytes()).into_string())
}

#[cfg(not(target_arch = "wasm32"))]
fn sign_challenge(_secret_b58: &str, _nonce: &str) -> Result<String, String> {
    Err("Signing runs in the browser.".to_string())
}

#[component]
fn SigninPage() -> impl IntoView {
    let (did_input, set_did_input) = signal(String::new());
    let (secret_input, set_secret_input) = signal(String::new());
    let (notice, set_notice) = signal(String::new());
    let (answered, set_answered) = signal(String::new());
    let navigate = use_navigate();
    let nonce_action = Action::new(|did: &String| {
        let did = did.clone();
        async move { request_nonce(did).await }
    });
    let create_action = Action::new(|_: &()| async move { create_identity().await });

    Effect::new_sync(move |_| {
        if let Some(Ok(nonce)) = nonce_action.value().get() {
            if answered.get() == nonce {
                return;
            }
            set_answered.set(nonce.clone());
            let did = did_input.get();
            let nonce_send = nonce.clone();
            let nav = navigate.clone();
            match sign_challenge(&secret_input.get(), &nonce) {
                Ok(sig) => {
                    leptos::task::spawn_local(async move {
                        match answer_nonce(did, nonce_send, sig).await {
                            Ok(token) => {
                                store_token(&token);
                                nav("/docs", Default::default());
                            }
                            Err(e) => set_notice.set(format!("Not signed in: {e}")),
                        }
                    });
                }
                Err(e) => set_notice.set(e),
            }
        }
        if let Some(Ok(fresh)) = create_action.value().get() {
            store_token(&fresh.token);
            set_notice.set(format!("New identity {} signed in. The owner grants powers on the settings page.", fresh.did));
        }
    });

    view! {
        <ModelsNavbar show_models=false/>
        <main class="main">
            <section class="hero">
                <h1>"Sign in."</h1>
                <p class="hero-tagline">"Present the identity and sign the challenge with its secret."</p>
            </section>
            <section class="family">
                <div class="family-card">
                    <h3>"Peer identity"</h3>
                    <Input
                        name="did".to_string()
                        placeholder="did:peer:…".to_string()
                        on_change=Callback::new(move |v: String| set_did_input.set(v))
                    />
                    <p><input type="password" placeholder="secret" on:input=move |ev| set_secret_input.set(event_target_value(&ev))/></p>
                    <p><RadixButton on_click=Callback::new(move |_| {
                        set_notice.set(String::new());
                        nonce_action.dispatch(did_input.get());
                    })>"Sign in"</RadixButton></p>
                    <p>{move || notice.get()}</p>
                </div>
                <div class="family-card">
                    <h3>"No identity yet"</h3>
                    <p>"Generate a peer identity here, then it signs in with fresh powers pending."</p>
                    <p><RadixButton on_click=Callback::new(move |_| {
                        create_action.dispatch(());
                    })>"Generate identity"</RadixButton></p>
                </div>
            </section>
        </main>
    }
}

/// Gated docs: only a signed-in, approved identity sees the content.
#[component]
fn DocsPage() -> impl IntoView {
    let gate = Resource::new(
        || (),
        |_| async move {
            match read_token() {
                Some(token) => docs_content(token).await.map(Some),
                None => Ok(None),
            }
        },
    );
    let models_gate = Resource::new(
        || (),
        |_| async move {
            match read_token() {
                Some(t) => gate_check(t).await.unwrap_or(None),
                None => None,
            }
        },
    );
    let logout = Action::new(|token: &String| {
        let token = token.clone();
        async move { sign_out(token).await }
    });
    Effect::new_sync(move |_| {
        if logout.value().get().is_some_and(|r| r.is_ok()) {
            clear_token();
            gate.refetch();
        }
    });

    view! {
        <Suspense fallback=|| view! { <p>"Checking identity…"</p> }>
            {move || {
                let show = models_gate.get().flatten().map(|v| v.model_updates).unwrap_or(false);
                view! {
                    <ModelsNavbar show_models=show/>
                    <main class="main">
                        <section class="hero">
                            <h1>"Docs."</h1>
                        </section>
                        <section class="family">
                            {move || Suspend::new(async move {
                                match gate.get() {
                                    None => view! { <p>"Checking identity…"</p> }.into_any(),
                                    Some(Ok(Some(view))) => {
                                        if view.approved {
                                            view! {
                                                <div class="family-card">
                                                    <h3>"Gated docs"</h3>
                                                    <p>{format!("Signed in as {}. This page is visible only to approved identities.", view.did)}</p>
                                                    <p><RadixButton on_click=Callback::new(move |_| {
                                                        logout.dispatch(read_token().unwrap_or_default());
                                                    })>"Sign out"</RadixButton></p>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <div class="family-card">
                                                    <h3>"Not approved yet"</h3>
                                                    <p>{format!("{} is signed in but not on the approved list.", view.did)}</p>
                                                    <p><A href="/signin">"Back to sign in"</A></p>
                                                </div>
                                            }.into_any()
                                        }
                                    }
                                    Some(Ok(None)) => view! {
                                        <div class="family-card">
                                            <h3>"Sign in to read"</h3>
                                            <p><A href="/signin">"Go to sign in"</A></p>
                                        </div>
                                    }.into_any(),
                                    Some(Err(e)) => view! {
                                        <div class="family-card">
                                            <h3>"Sign in to read"</h3>
                                            <p>{format!("{e}")}</p>
                                            <p><A href="/signin">"Go to sign in"</A></p>
                                        </div>
                                    }.into_any(),
                                }
                            })}
                        </section>
                    </main>
                }
            }}
        </Suspense>
    }
}

#[component]
fn ModelsPage() -> impl IntoView {
    let gate = Resource::new(
        || (),
        |_| async move {
            match read_token() {
                Some(t) => gate_check(t).await.unwrap_or(None),
                None => None,
            }
        },
    );
    let rows = Resource::new(|| (), |_| async move { registry_rows().await.unwrap_or_default() });
    view! {
        <Suspense fallback=|| view! { <p>"Checking identity…"</p> }>
            {move || gate.get().map(|g| match g {
                Some(v) if v.model_updates => view! {
                    <ModelsNavbar show_models=true/>
                    <main class="main">
                        <section class="runs"><h2>"Runs"</h2></section>
                        <section class="models">
                            <h2>"Models"</h2>
                            <Suspense fallback=|| view! { <p>"Loading registry…"</p> }>
                                {move || rows.get().map(|list: Vec<ModelRow>| view! {
                                    <table>
                                        <tr><th>"Model"</th><th>"Provider"</th><th>"Context"</th><th>"Last tested"</th></tr>
                                        {list.into_iter().map(|m| view! {
                                            <tr><td>{m.model_id}</td><td>{m.provider}</td><td>{m.context_window.map(|c| c.to_string()).unwrap_or_default()}</td><td>{m.last_tested.unwrap_or_default()}</td></tr>
                                        }).collect::<Vec<_>>()}
                                    </table>
                                })}
                            </Suspense>
                        </section>
                    </main>
                }.into_any(),
                _ => view! {
                    <ModelsNavbar show_models=false/>
                    <main class="main"><p><A href="/signin">"Sign in with a delegated identity to read the registry."</A></p></main>
                }.into_any(),
            })}
        </Suspense>
    }
}

/// Display controls shared by every visitor, signed in or not.
/// Choices persist in browser storage immediately and, for signed-in
/// visitors, in the server-side settings copy through apply.
#[component]
fn DisplayControls(
    appearance: ReadSignal<String>,
    set_appearance: WriteSignal<String>,
    width: ReadSignal<String>,
    set_width: WriteSignal<String>,
    apply: impl Fn() + Clone + Send + 'static,
) -> impl IntoView {
    let apply_first = apply.clone();
    let apply_second = apply;
    view! {
        <div class="family-card">
            <h3>"Display"</h3>
            <p><select
                prop:value=move || appearance.get()
                on:change=move |ev| { set_appearance.set(event_target_value(&ev)); apply_first.clone()(); }
            >
                <option value="system">"System"</option>
                <option value="light">"Light"</option>
                <option value="dark">"Dark"</option>
            </select></p>
            <p><select
                prop:value=move || width.get()
                on:change=move |ev| { set_width.set(event_target_value(&ev)); apply_second.clone()(); }
            >
                <option value="normal">"Normal width"</option>
                <option value="wide">"Wide"</option>
            </select></p>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
fn try_load_browser_settings(set_appearance: &WriteSignal<String>, set_width: &WriteSignal<String>) {
    let get = |k: &str| {
        web_sys::window()?.local_storage().ok()?? .get_item(k).ok()?
    };
    if let Some(a) = get("mu_appearance") {
        set_appearance.set(a);
    }
    if let Some(w) = get("mu_width") {
        set_width.set(w);
    }
    let _ = (set_appearance, set_width);
}

#[cfg(not(target_arch = "wasm32"))]
fn try_load_browser_settings(_a: &WriteSignal<String>, _w: &WriteSignal<String>) {}

fn apply_browser_settings(appearance: &str, width: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("mu_appearance", appearance);
                let _ = storage.set_item("mu_width", width);
            }
            if let Some(document) = window.document() {
                if let Some(el) = document.document_element() {
                    let _ = el.set_attribute("data-appearance", appearance);
                    let _ = el.set_attribute("data-width", width);
                }
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (appearance, width);
}

#[component]
fn SettingsPage() -> impl IntoView {
    let gate = Resource::new(
        || (),
        |_| async move {
            match read_token() {
                Some(t) => gate_check(t).await.unwrap_or(None),
                None => None,
            }
        },
    );
    let (appearance, set_appearance) = signal("system".to_string());
    let (width, set_width) = signal("normal".to_string());
    let (note, set_note) = signal(String::new());

    try_load_browser_settings(&set_appearance, &set_width);

    let apply = move || {
        apply_browser_settings(&appearance.get(), &width.get());
        if let Some(t) = read_token() {
            let s = VisitorSettings { appearance: appearance.get(), width: width.get() };
            leptos::task::spawn_local(async move {
                let _ = save_settings(t, s).await;
            });
        }
    };
    let server_settings = Resource::new(
        || (),
        |_| async move {
            match read_token() {
                Some(t) => read_settings(t).await.ok(),
                None => None,
            }
        },
    );
    let (settings_loaded, set_settings_loaded) = signal(false);
    Effect::new_sync(move |_| {
        if !settings_loaded.get() {
            if let Some(Some(s)) = server_settings.get() {
                if !s.appearance.is_empty() {
                    set_appearance.set(s.appearance);
                }
                if !s.width.is_empty() {
                    set_width.set(s.width);
                }
                set_settings_loaded.set(true);
            }
        }
    });
    let pending = Resource::new(
        || (),
        |_| async move {
            match read_token() {
                Some(t) => pending_list(t).await.unwrap_or_default(),
                None => vec![],
            }
        },
    );
    let (grant_did, set_grant_did) = signal(String::new());
    let (grant_level, set_grant_level) = signal("read".to_string());
    let grant_action = Action::new(|input: &(String, String, String)| {
        let (token, did, level) = input.clone();
        async move { grant_access(token, did, level).await }
    });
    let revoke_action = Action::new(|input: &(String, String)| {
        let (token, did) = input.clone();
        async move { revoke_access(token, did).await }
    });
    Effect::new_sync(move |_| {
        if let Some(r) = grant_action.value().get() {
            match r {
                Ok(()) => {
                    set_note.set("Granted.".to_string());
                    pending.refetch();
                }
                Err(e) => set_note.set(format!("{e}")),
            }
        }
        if let Some(r) = revoke_action.value().get() {
            match r {
                Ok(()) => {
                    set_note.set("Revoked.".to_string());
                    pending.refetch();
                }
                Err(e) => set_note.set(format!("{e}")),
            }
        }
    });
    let logout = Action::new(|token: &String| {
        let token = token.clone();
        async move { sign_out(token).await }
    });

    view! {
        <Suspense fallback=|| view! { <p>"Checking identity…"</p> }>
            {move || gate.get().map(|g| match g {
                Some(v) => {
                    let admin = v.admin;
                    let did = v.did.clone();
                    view! {
                        <ModelsNavbar show_models=v.model_updates/>
                        <main class="main">
                            <section class="hero"><h1>"Settings."</h1></section>
                            <p>{format!("Signed in as {did}")}</p>
                            <p><button on:click=move |_| {
                                if let Some(t) = read_token() { logout.dispatch(t); }
                                clear_token();
                                gate.refetch();
                            }>"Sign out"</button></p>
                            <section class="family">
                                <DisplayControls appearance=appearance set_appearance=set_appearance width=width set_width=set_width apply=apply/>
                            </section>
                            {admin.then(|| view! {
                                <section class="family">
                                    <div class="family-card">
                                        <h3>"Owner tools"</h3>
                                        <p><input placeholder="did:peer:…" on:input=move |ev| set_grant_did.set(event_target_value(&ev))/></p>
                                        <p><select on:change=move |ev| set_grant_level.set(event_target_value(&ev))>
                                            <option value="read">"read"</option>
                                            <option value="write">"write"</option>
                                            <option value="admin">"admin"</option>
                                        </select></p>
                                        <p><button on:click=move |_| {
                                            if let Some(t) = read_token() { grant_action.dispatch((t, grant_did.get(), grant_level.get())); }
                                        }>"Grant"</button></p>
                                        <p>{move || note.get()}</p>
                                        <h3>"Waiting identities"</h3>
                                        {move || pending.get().map(|list| view! {
                                            <ul>
                                            {list.into_iter().map(|p| {
                                                let d = p.did.clone();
                                                view! { <li>{d.clone()} <button on:click=move |_| {
                                                    if let Some(t) = read_token() { revoke_action.dispatch((t, d.clone())); }
                                                }>"Revoke"</button></li> }
                                            }).collect::<Vec<_>>()}
                                            </ul>
                                        })}
                                    </div>
                                </section>
                            })}
                        </main>
                    }.into_any()
                }
                None => view! {
                    <ModelsNavbar show_models=false/>
                    <main class="main">
                        <section class="hero"><h1>"Settings."</h1></section>
                        <section class="family">
                            <DisplayControls appearance=appearance set_appearance=set_appearance width=width set_width=set_width apply=apply/>
                        </section>
                        <p><A href="/signin">"Go to sign in"</A></p>
                    </main>
                }.into_any(),
            })}
        </Suspense>
    }
}
