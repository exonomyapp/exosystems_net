use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    hooks::use_navigate,
    StaticSegment,
};

use crate::auth::{
    active_did, answer_nonce, clear_token, create_identity, delete_identity, docs_content,
    drop_session, gate_check, get_flags, grant_access, live_sessions, pending_list, read_settings,
    read_token, request_nonce, revoke_access, save_settings, set_flags, sign_out, stash_session,
    switch_session, update_profile, user_avatar, user_display_name, users_list, VisitorSettings,
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

const CARDS: [(&str, &str, &str); 8] = [
    ("ExoTalk", "/assets/cards/exotalk.png",
     "The Exosystem's flagship app: private, encrypted messaging that runs natively on your phone and desktop. It is your personal node on the network, carrying your identity and keeping every conversation peer to peer — connecting directly when possible and passing through blind relays only when it must. Every other Exosystem app unfolds inside it, arriving as subscriptions you manage in your settings, and it can even create and manage Conscia service nodes on your behalf. Because apps update through signed blueprints delivered over the mesh, the interface of everything you subscribe to evolves the moment its author publishes. Sign-ins, subscriptions, and app updates all flow through it, making one app the solid front door to the whole family."),
    ("ExoSoul", "/assets/cards/exosoul.png",
     "The studio where every Exosystem app is authored — the press where each app is formed into a signed blueprint. One identity authors here; ExoTalk users subscribe, and the app unfolds on their devices with its interface driven by the blueprint, so a published change reaches every subscriber instantly. Every blueprint is a canonical app payload whose exact bytes are pinned by a digest, so what unfolds on a device is always precisely what was signed. Blueprints compose other blueprints only through digest-pinned includes with parent-owned overrides, keeping every app's lineage exact and verifiable. ExoSoul itself is a native app, and Conscia nodes host and relay its blueprints as a service."),
    ("Conscia", "/assets/cards/conscia.png",
     "The lifeline of the Exosystem: always-on nodes that relay messages, discover peers, answer petitions, and keep data reachable while your devices are offline — opt-in helper services. Every node app runs peer to peer independently; when one is present it can carry relay, indexing, discovery, persistence, authentication, observability, and federation, routing geographically and by latency. Its relationship worker handles petitions, federation, and indexing while its operator console administers each node, in small, community, and large clustered flavors — with a large node planned at exosystems.net itself — and a Conscierge worker coordinates the nodes with one another and reaches beyond the mesh. Blind indexing can also surface community circles and public directories through one unified search surface."),
    ("Exonomy", "/assets/cards/exonomy.png",
     "Social voucher exchange. Members — Exonomists — mint and trade vouchers that stand for real goods and services, optionally bought or sold for everyday money; the voucher itself carries the value. It arrives as an app inside ExoTalk, and while its vouchers can optionally fund Exocracy tasks, it operates entirely independently by default — a fully realized ecosystem of structured value exchange. Native apps carry it onto phones and desktops for managing heavy voucher ledgers, and an indexing node hosts encrypted voucher payloads blindly, surfacing only the metadata its owners choose to publish."),
    ("Exocracy", "/assets/cards/exocracy.png",
     "Transparent project governance — the governance layer of the mesh. Communities run projects with open proposals, votes, and milestones, funded with ordinary money or, optionally, Exonomy vouchers; that funding choice is engineered interoperability, and a project runs entirely on ordinary money when its manager prefers. Built for high-trust environments where organizational transparency and independent project management matter most, it connects with ExoTalk, Exonomy, and the rest of the family while standing entirely on its own. Native apps put real-time voting and task tracking in your pocket, and an indexing node keeps open tasks and bounties publicly searchable."),
    ("RepubLet", "/assets/cards/republet.png",
     "Decentralized scientific publishing and peer review — a modern Republic of Letters, the knowledge ledger of the mesh. Researchers' work lives on permanent, hash-anchored records with cryptographically verifiable authorship, and author-facing paywalls offer a decentralized alternative to subscription platforms for earning from one's own findings — settled in ordinary money, cryptocurrency, or, optionally, Exonomy vouchers. A scientist who initiates an Exocracy project to manage and fund research is stringing together independent modular tools. Native apps carry reading and peer-review notifications onto phones and desktops, and a citation relay keeps open-science metadata publicly searchable."),
    ("ThreeSteps (3S)", "/assets/cards/threesteps.png",
     "A private, offline-first social app for Catholic families and communities — helping children grow in faith through Baptism, First Communion, and Confirmation. Here social means relationships, guidance, and formation, and privacy means total discretion for those practicing in regions where faith requires it. Conversations connect directly when possible and pass through blind relays when they must, with local-first storage and cryptographic access control — so it keeps working through limited connectivity. Identity flows through ExoAuth, the family's shared identity layer, and missionaries, catechists, and parish groups meet the wider Exosystem through Conscia nodes."),
    ("InterNovel", "/assets/cards/internovel.png",
     "Collaborative storytelling for young authors. Each writer's contributions are digitally signed — every paragraph permanently linked to the identity that wrote it — so authorship stays verifiable and intellectual property stays with its creator. Characters can travel between stories through the Travel Trunk, the Novel Spine holds each story's structure, and novels are written together under unanimous consent. Flexible collaboration, clear attribution, and protected ownership make it a writing home that grows with its authors, and each novel unfolds inside ExoTalk as one of the family's apps."),
];

const ICON_MONITOR: &str = "M2 3a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v7a1 1 0 0 1-1 1H9.5v1.5h2a.5.5 0 0 1 0 1h-7a.5.5 0 0 1 0-1h2V11H3a1 1 0 0 1-1-1V3zm1 0v7h10V3H3z";
const ICON_SUN: &str = "M8 4.5A3.5 3.5 0 1 0 8 11.5 3.5 3.5 0 0 0 8 4.5zM8 0a.5.5 0 0 1 .5.5v1.5a.5.5 0 0 1-1 0V.5A.5.5 0 0 1 8 0zm0 13.5a.5.5 0 0 1 .5.5v1.5a.5.5 0 0 1-1 0V14a.5.5 0 0 1 .5-.5zM0 8a.5.5 0 0 1 .5-.5H2a.5.5 0 0 1 0 1H.5A.5.5 0 0 1 0 8zm13.5 0a.5.5 0 0 1 .5-.5h1.5a.5.5 0 0 1 0 1H14a.5.5 0 0 1-.5-.5zM2.3 2.3a.5.5 0 0 1 .7 0l1.1 1a.5.5 0 1 1-.7.8l-1.1-1a.5.5 0 0 1 0-.8zm9.6 9.6a.5.5 0 0 1 .7 0l1.1 1a.5.5 0 0 1-.7.8l-1.1-1a.5.5 0 0 1 0-.8zm1.8-9.6a.5.5 0 0 1 0 .7l-1 1.1a.5.5 0 1 1-.8-.7l1-1.1a.5.5 0 0 1 .8 0zM3.9 11.9a.5.5 0 0 1 0 .7l-1 1.1a.5.5 0 1 1-.7-.8l1-1.1a.5.5 0 0 1 .7 0z";
const ICON_MOON: &str = "M6 1a7 7 0 0 0 0 14c3.5 0 6.5-2.6 7-6a.5.5 0 0 0-.85-.43A4.5 4.5 0 0 1 6.43 3a.5.5 0 0 0-.43-.85V1z";
const ICON_WIDTH: &str = "M21 11V3h-8l3.29 3.29-10 10L3 13v8h8l-3.29-3.29 10-10L21 11z";

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

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
struct IdentityState {
    signed_in: bool,
    did: String,
    display_name: String,
    avatar: String,
    model_updates: bool,
    admin: bool,
}

fn load_identity() -> Resource<Option<IdentityState>> {
    Resource::new(
        || (),
        |_| async move {
            let token = read_token()?;
            let gate = gate_check(token).await.ok()??;
            let avatar = user_avatar(gate.did.clone()).await.unwrap_or_default();
            let display_name = user_display_name(gate.did.clone()).await.unwrap_or_default();
            Some(IdentityState {
                signed_in: true,
                did: gate.did.clone(),
                display_name,
                avatar,
                model_updates: gate.model_updates,
                admin: gate.admin,
            })
        },
    )
}

#[component]
fn FloatingGadgets(identity: Resource<Option<IdentityState>>) -> impl IntoView {
    let users = Resource::new(|| (), |_| async move { users_list().await.unwrap_or_default() });
    let (chip_open, set_chip_open) = signal(false);
    let (appearance, set_appearance) = signal("system".to_string());
    let (width, set_width) = signal("normal".to_string());
    try_load_browser_settings(&set_appearance, &set_width);
    let apply = apply_display(appearance, width);

    view! {
        <div class="navbar">
            <div class="brandrow">
            <img src="/exosys.png" alt="ExoSystems" class="navbar-logo"/>
            <div class="navbar-menu">
                {move || identity.get().flatten().filter(|id| id.model_updates).map(|_| view! { <A href="/models">"Models"</A> })}
                <A href="/settings">"Settings"</A>
                <DisplayGadgets appearance=appearance set_appearance=set_appearance width=width set_width=set_width apply=apply.clone()/>
                {move || match identity.get().flatten() {
                    Some(id) => view! {
                        <span class="chip-wrap">
                        <span class="identity-chip" on:click=move |_| set_chip_open.set(!chip_open.get())>
                            {if id.avatar.is_empty() { view! { <span class="avatar-fallback">{id.display_name.chars().take(1).collect::<String>()}</span> }.into_any() } else { view! { <img class="avatar" src=id.avatar.clone() alt="profile"/> }.into_any() }}
                            <span class="chip-name">{if id.display_name.is_empty() { id.did.clone() } else { id.display_name.clone() }}</span>
                        </span>
                        {chip_open.get().then(|| view! {
                            <span class="chip-dropdown">
                                <A href="/settings">"Profile gear"</A>
                                {move || users.get().map(|list| list.into_iter().map(|u| {
                                    let (did, name) = (u.did.clone(), u.display_name.clone());
                                    view! {
                                        <span class="chip-user" on:click=move |_| {
                                            if switch_session(&did) { set_chip_open.set(false); }
                                        }>{name}</span>
                                    }
                                }).collect::<Vec<_>>())}
                                <A href="/signin">"Add identity"</A>
                            </span>
                        })}
                        </span>
                    }.into_any(),
                    None => view! { <A href="/signin">"Sign in"</A> }.into_any(),
                }}
            </div>
            </div>
        </div>
    }
}

fn theme_icon(v: &str) -> &'static str {
    match v {
        "light" => ICON_SUN,
        "dark" => ICON_MOON,
        _ => ICON_MONITOR,
    }
}

fn theme_label(v: &str) -> &'static str {
    match v {
        "light" => "Light theme",
        "dark" => "Dark theme",
        _ => "System theme",
    }
}

#[component]
fn ThemeDrop(
    appearance: ReadSignal<String>,
    set_appearance: WriteSignal<String>,
    apply: impl Fn() + Clone + Send + 'static,
) -> impl IntoView {
    let (drop_open, set_drop_open) = signal(false);
    let apply_panel = apply;
    view! {
        <span class="icon-drop-wrap">
            <button type="button" class="ctl"
                title=move || theme_label(&appearance.get())
                aria-label=move || theme_label(&appearance.get())
                on:click=move |_| set_drop_open.set(!drop_open.get())>
                <svg viewBox="0 0 16 16" width="16" height="16" fill="currentColor" aria-hidden="true">
                    <path d=move || theme_icon(&appearance.get())/>
                </svg>
            </button>
            {move || drop_open.get().then(|| {
                let current = appearance.get();
                view! {
                    <span class="drop-backdrop" on:click=move |_| set_drop_open.set(false)></span>
                    <span class="icon-drop-panel">
                        {["system", "light", "dark"].into_iter().filter(move |v| *v != current).map(|v| {
                            let a = apply_panel.clone();
                            view! {
                                <button type="button" class="ctl"
                                    title=theme_label(v) aria-label=theme_label(v)
                                    on:click=move |_| {
                                        set_appearance.set(v.to_string());
                                        a();
                                        set_drop_open.set(false);
                                    }>
                                    <svg viewBox="0 0 16 16" width="16" height="16" fill="currentColor" aria-hidden="true">
                                        <path d=theme_icon(v)/>
                                    </svg>
                                </button>
                            }
                        }).collect::<Vec<_>>()}
                    </span>
                }
            })}
        </span>
    }
}

#[component]
fn WidthToggle(
    width: ReadSignal<String>,
    set_width: WriteSignal<String>,
    apply: impl Fn() + Clone + Send + 'static,
) -> impl IntoView {
    view! {
        <button type="button" class="ctl" class:active=move || width.get() == "wide"
            title=move || if width.get() == "wide" { "Fit to margins" } else { "Full width" }
            aria-label=move || if width.get() == "wide" { "Fit to margins" } else { "Full width" }
            on:click=move |_| { set_width.set(if width.get() == "wide" { "normal".into() } else { "wide".into() }); apply(); }>
            <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor" aria-hidden="true">
                <path d=ICON_WIDTH/>
            </svg>
        </button>
    }
}

#[component]
fn DisplayGadgets(
    appearance: ReadSignal<String>,
    set_appearance: WriteSignal<String>,
    width: ReadSignal<String>,
    set_width: WriteSignal<String>,
    apply: impl Fn() + Clone + Send + 'static,
) -> impl IntoView {
    let apply_theme = apply.clone();
    let apply_width = apply;
    view! {
        <span class="gadget-row">
            <ThemeDrop appearance=appearance set_appearance=set_appearance apply=apply_theme/>
            <WidthToggle width=width set_width=set_width apply=apply_width/>
        </span>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let identity = load_identity();
    view! {
        <header class="masthead">
                    <div class="slide slide-1"></div>
                    <div class="slide slide-2"></div>
                    <div class="screen"></div>
                    <FloatingGadgets identity=identity/>
            <section class="hero">
                <div class="tagline-block">
                    <p><span class="t1">"Hello, World! We're building something new."</span>{" "}<span class="t2">"Private by design. Connected by nature."</span><span class="t-sub">"... for the software was made for the man, not the man for the software!"</span></p>
                </div>
            </section>
        </header>
        <main class="main">
            <section class="family">
                {CARDS.iter().map(|(title, art, full)| view! { <FlipCard title=*title art=*art full=*full/> }).collect::<Vec<_>>()}
            </section>
        </main>
    }
}

#[component]
fn FlipCard(title: &'static str, art: &'static str, full: &'static str) -> impl IntoView {
    let (flipped, set_flipped) = signal(false);
    let full_ref: NodeRef<leptos::html::Div> = NodeRef::new();
    let (show_top, set_show_top) = signal(false);
    let (show_bottom, set_show_bottom) = signal(false);
    let (fade_top, set_fade_top) = signal(false);
    let (fade_bottom, set_fade_bottom) = signal(false);

    let on_scroll = move |_| {
        if let Some(el) = full_ref.get() {
            let top = el.scroll_top();
            let height = el.client_height();
            let total = el.scroll_height();
            let scrollable = total > height + 8;
            set_show_top.set(top > 8);
            set_fade_top.set(scrollable && top > 8);
            set_fade_bottom.set(scrollable && top + height < total - 8);
            set_show_bottom.set(scrollable && top + height < total - 8);
        }
    };
    let scroll_to_top = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        if let Some(el) = full_ref.get() {
            el.set_scroll_top(0);
        }
    };
    let scroll_to_bottom = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        if let Some(el) = full_ref.get() {
            el.set_scroll_top(el.scroll_height());
        }
    };

    view! {
        <div class="flip" on:click=move |_| set_flipped.set(!flipped.get())>
            <div class="flip-box" class:flipped=move || flipped.get()>
                <div class="face front">
                    <div class="art">
                        <img src=art alt=title/>
                    </div>
                    <h3>{title}</h3>
                </div>
                <div class="face back">
                    <div class="full" node_ref=full_ref on:scroll=on_scroll
                        class:ftop=move || fade_top.get()
                        class:fbot=move || fade_bottom.get()>
                        <p>{full}</p>
                    </div>
                    <h3 class="backlabel">{title}</h3>
                    <button type="button" class="tobot" class:show=move || show_bottom.get() on:click=scroll_to_bottom aria-label="Jump to bottom">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14"/><path d="m19 12-7 7-7-7"/></svg>
                    </button>
                    <button type="button" class="totop" class:show=move || show_top.get() on:click=scroll_to_top aria-label="Back to top">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 19V5"/><path d="m5 12 7-7 7 7"/></svg>
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ModelsNavbar(show_models: bool, display_name: String) -> impl IntoView {
    let (appearance, set_appearance) = signal("system".to_string());
    let (width, set_width) = signal("normal".to_string());
    try_load_browser_settings(&set_appearance, &set_width);
    let apply = apply_display(appearance, width);
    view! {
        <nav class="navbar navbar-solid">
            <img src="/exosys.png" alt="ExoSystems" class="navbar-logo"/>
            <div class="navbar-menu">
                <A href="/">"Home"</A>
                {show_models.then(|| view! { <A href="/models">"Models"</A> })}
                <A href="/settings">"Settings"</A>
                {if display_name.is_empty() { view! { <A href="/signin">"Sign in"</A> }.into_any() } else { view! { <span class="chip-name">{display_name}</span> }.into_any() }}
                <DisplayGadgets appearance=appearance set_appearance=set_appearance width=width set_width=set_width apply=apply/>
            </div>
        </nav>
    }
}

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
    let users = Resource::new(|| (), |_| async move { users_list().await.unwrap_or_default() });
    let flags = Resource::new(|| (), |_| async move { get_flags().await.unwrap_or_default() });
    let (did_input, set_did_input) = signal(String::new());
    let (secret_input, set_secret_input) = signal(String::new());
    let (name_input, set_name_input) = signal(String::new());
    let (notice, set_notice) = signal(String::new());
    let (answered, set_answered) = signal(String::new());
    let navigate = use_navigate();
    let nonce_action = Action::new(|did: &String| {
        let did = did.clone();
        async move { request_nonce(did).await }
    });
    let create_action = Action::new(|input: &(String, String)| {
        let (name, avatar) = input.clone();
        async move { create_identity(name, avatar).await }
    });

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
                        match answer_nonce(did.clone(), nonce_send, sig).await {
                            Ok(token) => {
                                stash_session(&did, &token);
                                nav("/", Default::default());
                            }
                            Err(e) => set_notice.set(format!("Not signed in: {e}")),
                        }
                    });
                }
                Err(e) => set_notice.set(e),
            }
        }
        if let Some(Ok(fresh)) = create_action.value().get() {
            stash_session(&fresh.did, &fresh.token);
            set_notice.set(format!("New identity {} signed in. The owner grants powers on the settings page.", fresh.did));
        }
    });

    view! {
        <ModelsNavbar show_models=false display_name=String::new()/>
        <main class="main page-main">
            <header class="page-head">
                <h1>"Sign in."</h1>
                <p>"Your identity lives with you. Sign in with your peer identity and stay on ExoSystems.net."</p>
            </header>
            <section class="page-cards">
                <div class="family-card">
                    <h3>"Known identities"</h3>
                    <p>"Identities the local Willow store knows. Pick one, prove its secret, and continue."</p>
                    <div class="id-list">
                        {move || users.get().map(|list| list.into_iter().map(|u| {
                            let did = u.did.clone();
                            let name = u.display_name.clone();
                            let initial = name.chars().next().unwrap_or('?').to_uppercase().to_string();
                            view! {
                                <button on:click=move |_| { set_notice.set(String::new()); set_did_input.set(did.clone()); nonce_action.dispatch(did.clone()); }>
                                    <span class="avatar-fallback">{initial.clone()}</span>
                                    <span>{name.clone()}</span>
                                </button>
                            }
                        }).collect::<Vec<_>>())}
                    </div>
                    <div class="field">
                        <label>"Identity"</label>
                        <input placeholder="did:peer:…" on:input=move |ev| set_did_input.set(event_target_value(&ev))/>
                    </div>
                    <div class="field">
                        <label>"Secret"</label>
                        <input type="password" placeholder="secret" on:input=move |ev| set_secret_input.set(event_target_value(&ev))/>
                    </div>
                    <div class="btn-row">
                        <button on:click=move |_| {
                            set_notice.set(String::new());
                            nonce_action.dispatch(did_input.get());
                        }>"Continue"</button>
                    </div>
                    {move || { let n = notice.get(); if n.is_empty() { view!{}.into_any() } else { view!{ <p class="notice">{n}</p> }.into_any() } }}
                </div>
                <div class="family-card">
                    <h3>"No identity yet"</h3>
                    {move || flags.get().map(|f| if f.disable_new_users {
                        view! { <p>"Adding identities is paused by the owner."</p> }.into_any()
                    } else {
                        view! {
                            <p>"Generate a peer identity here with its display name, then it signs in."</p>
                            <div class="field">
                                <label>"Display name"</label>
                                <input placeholder="Display name" on:input=move |ev| set_name_input.set(event_target_value(&ev))/>
                            </div>
                            <div class="btn-row">
                                <button on:click=move |_| {
                                    create_action.dispatch((name_input.get(), String::new()));
                                }>"Generate identity"</button>
                            </div>
                        }.into_any()
                    })}
                </div>
            </section>
        </main>
    }
}

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
    let logout = Action::new(|input: &(String, String)| {
        let (token, did) = input.clone();
        async move { sign_out(token).await.map(|_| did) }
    });
    Effect::new_sync(move |_| {
        if let Some(Ok(did)) = logout.value().get() {
            drop_session(&did);
            clear_token();
            gate.refetch();
        }
    });

    view! {
        <Suspense fallback=|| view! { <p>"Checking identity…"</p> }>
            {move || {
                let named = models_gate.get().flatten().map(|v| (v.model_updates, v.did)).unwrap_or((false, String::new()));
                view! {
                    <ModelsNavbar show_models=named.0 display_name=named.1/>
                    <main class="main page-main">
                        <header class="page-head">
                            <h1>"Docs."</h1>
                        </header>
                        <section class="page-cards">
                            {move || Suspend::new(async move {
                                match gate.get() {
                                    None => view! { <div class="family-card"><p>"Checking identity…"</p></div> }.into_any(),
                                    Some(Ok(Some(view))) => {
                                        if view.approved {
                                            view! {
                                                <div class="family-card">
                                                    <h3>"Gated docs"</h3>
                                                    <p>{format!("Signed in as {}. This page is visible only to approved identities.", view.did)}</p>
                                                    <div class="btn-row">
                                                        <button on:click=move |_| {
                                                            let t = read_token().unwrap_or_default();
                                                            let d = active_did().unwrap_or_default();
                                                            logout.dispatch((t, d));
                                                        }>"Sign out"</button>
                                                    </div>
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
    view! {
        <Suspense fallback=|| view! { <p>"Checking identity…"</p> }>
            {move || gate.get().map(|g| match g {
                Some(v) if v.model_updates => {
                    let name = v.did.clone();
                    view! {
                        <ModelsNavbar show_models=true display_name=name/>
                        <main class="main page-main">
                            <section class="runs"><h2>"Runs"</h2></section>
                            <section class="models">
                                <h2>"Models"</h2>
                                <ModelsTable/>
                            </section>
                        </main>
                    }.into_any()
                }
                _ => view! {
                    <ModelsNavbar show_models=false display_name=String::new()/>
                    <main class="main page-main"><p><A href="/signin">"Sign in with a delegated identity to read the registry."</A></p></main>
                }.into_any(),
            })}
        </Suspense>
    }
}

#[component]
fn ModelsTable() -> impl IntoView {
    let rows = Resource::new(|| (), |_| async move { registry_rows().await.unwrap_or_default() });
    let (query, set_query) = signal(String::new());
    let (picked, set_picked) = signal(std::collections::HashSet::<String>::new());
    let (picked_init, set_picked_init) = signal(false);
    let (sort_col, set_sort_col) = signal(String::new());
    let (sort_dir, set_sort_dir) = signal(0i8);

    Effect::new_sync(move |_| {
        if !picked_init.get() {
            if let Some(list) = rows.get() {
                let mut set = std::collections::HashSet::new();
                for m in &list {
                    set.insert(m.provider.clone());
                }
                set_picked.set(set);
                set_picked_init.set(true);
            }
        }
    });

    let providers = move || {
        let mut names: Vec<String> = rows
            .get()
            .unwrap_or_default()
            .iter()
            .map(|m| m.provider.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        names.sort();
        names
    };

    let shown = move || {
        let q = query.get().to_lowercase();
        let pick = picked.get();
        let mut list: Vec<ModelRow> = rows
            .get()
            .unwrap_or_default()
            .into_iter()
            .filter(|m| pick.contains(&m.provider))
            .filter(|m| {
                q.is_empty()
                    || m.model_id.to_lowercase().contains(&q)
                    || m.provider.to_lowercase().contains(&q)
            })
            .collect();
        let col = sort_col.get();
        let dir = sort_dir.get();
        if dir != 0 && !col.is_empty() {
            list.sort_by(|a, b| {
                let ord = match col.as_str() {
                    "provider" => a.provider.cmp(&b.provider),
                    "context" => a.context_window.cmp(&b.context_window),
                    "tested" => a.last_tested.cmp(&b.last_tested),
                    _ => a.model_id.cmp(&b.model_id),
                };
                if dir < 0 { ord.reverse() } else { ord }
            });
        }
        list
    };

    let cycle = move |col: &'static str| {
        if sort_col.get() == col {
            set_sort_dir.set(if sort_dir.get() == 1 { -1 } else if sort_dir.get() == -1 { 0 } else { 1 });
            if sort_dir.get() == 0 {
                set_sort_col.set(String::new());
            }
        } else {
            set_sort_col.set(col.to_string());
            set_sort_dir.set(1);
        }
    };
    let mark = move |col: &'static str| {
        if sort_col.get() == col {
            if sort_dir.get() == 1 { " ▲" } else if sort_dir.get() == -1 { " ▼" } else { "" }
        } else {
            ""
        }
    };

    view! {
        <Suspense fallback=|| view! { <p>"Loading registry…"</p> }>
            <div class="ops-row">
                <p><input placeholder="Search models…" on:input=move |ev| set_query.set(event_target_value(&ev))/></p>
                <div class="provider-pick">
                    {move || providers().into_iter().map(|p| {
                        let name = p.clone();
                        let checked = picked.get().contains(&p);
                        view! {
                            <label>
                                <input type="checkbox" prop:checked=checked on:change=move |ev| {
                                    let on = event_target_checked(&ev);
                                    let mut set = picked.get();
                                    if on { set.insert(name.clone()); } else { set.remove(&name); }
                                    set_picked.set(set);
                                }/>{p}
                            </label>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
            {move || {
                let list = shown();
                view! {
                    <p>{format!("{} of {} shown", list.len(), rows.get().map(|l| l.len()).unwrap_or(0))}</p>
                    <table>
                        <tr>
                            <th data-sort="1" on:click=move |_| cycle("model")>{move || format!("Model{}", mark("model"))}</th>
                            <th data-sort="1" on:click=move |_| cycle("provider")>{move || format!("Provider{}", mark("provider"))}</th>
                            <th data-sort="1" on:click=move |_| cycle("context")>{move || format!("Context{}", mark("context"))}</th>
                            <th data-sort="1" on:click=move |_| cycle("tested")>{move || format!("Last tested{}", mark("tested"))}</th>
                        </tr>
                        {list.into_iter().map(|m| view! {
                            <tr><td>{m.model_id}</td><td>{m.provider}</td><td>{m.context_window.map(|c| c.to_string()).unwrap_or_default()}</td><td>{m.last_tested.unwrap_or_default()}</td></tr>
                        }).collect::<Vec<_>>()}
                    </table>
                }
            }}
        </Suspense>
    }
}

fn apply_display(appearance: ReadSignal<String>, width: ReadSignal<String>) -> impl Fn() + Clone + Send + 'static {
    move || {
        apply_browser_settings(&appearance.get(), &width.get());
        if let Some(t) = read_token() {
            let s = VisitorSettings { appearance: appearance.get(), width: width.get() };
            leptos::task::spawn_local(async move {
                let _ = save_settings(t, s).await;
            });
        }
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
    let (name_edit, set_name_edit) = signal(String::new());

    try_load_browser_settings(&set_appearance, &set_width);

    let apply = apply_display(appearance, width);
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
    let flags = Resource::new(|| (), |_| async move { get_flags().await.unwrap_or_default() });
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
    let profile_action = Action::new(|input: &(String, String, String)| {
        let (token, name, avatar) = input.clone();
        async move { update_profile(token, name, avatar).await }
    });
    let remove_action = Action::new(|input: &(String, String)| {
        let (token, did) = input.clone();
        async move { delete_identity(token, did).await }
    });
    let flag_action = Action::new(|input: &(String, bool)| {
        let (token, off) = input.clone();
        async move {
            use crate::auth::SiteFlags;
            set_flags(token, SiteFlags { disable_new_users: off }).await
        }
    });
    Effect::new_sync(move |_| {
        for r in [grant_action.value().get(), revoke_action.value().get(), profile_action.value().get(), remove_action.value().get(), flag_action.value().get()].into_iter().flatten() {
            match r {
                Ok(()) => {
                    set_note.set("Saved.".to_string());
                    pending.refetch();
                    flags.refetch();
                    gate.refetch();
                }
                Err(e) => set_note.set(format!("{e}")),
            }
        }
    });
    let logout = Action::new(|input: &(String, String)| {
        let (token, did) = input.clone();
        async move { sign_out(token).await.map(|_| did) }
    });
    Effect::new_sync(move |_| {
        if let Some(Ok(did)) = logout.value().get() {
            drop_session(&did);
            clear_token();
            gate.refetch();
        }
    });

    view! {
        <Suspense fallback=|| view! { <p>"Checking identity…"</p> }>
            {move || gate.get().map(|g| match g {
                Some(v) => {
                    let admin = v.admin;
                    let did = v.did.clone();
                    let named = (v.model_updates, v.did.clone());
                    view! {
                        <ModelsNavbar show_models=named.0 display_name=named.1/>
                        <main class="main page-main">
                            <header class="page-head">
                                <div class="head-row">
                                    <h1>"Settings."</h1>
                                    <p>{format!("Signed in as {did}")}</p>
                                    <button class="secondary" on:click=move |_| {
                                        let t = read_token().unwrap_or_default();
                                        let d = active_did().unwrap_or_default();
                                        logout.dispatch((t, d));
                                    }>"Sign out"</button>
                                </div>
                            </header>
                            <section class="page-cards">
                                <div class="family-card">
                                    <h3>"Display"</h3>
                                    <p>"Appearance follows the system or forces light or dark; content width fits the margins or fills the screen. Choices persist in browser storage and sync to the identity's namespace while signed in."</p>
                                    <div class="field">
                                        <label>"Appearance"</label>
                                        <ThemeDrop appearance=appearance set_appearance=set_appearance apply=apply.clone()/>
                                    </div>
                                    <div class="field">
                                        <label>"Content width"</label>
                                        <WidthToggle width=width set_width=set_width apply=apply.clone()/>
                                    </div>
                                </div>
                                <div class="family-card">
                                    <h3>"Profile"</h3>
                                    <p>"Display name and avatar travel with this identity."</p>
                                    <div class="field">
                                        <label>"Display name"</label>
                                        <input placeholder="Display name" on:input=move |ev| set_name_edit.set(event_target_value(&ev))/>
                                    </div>
                                    <div class="btn-row">
                                        <button on:click=move |_| {
                                            if let Some(t) = read_token() { profile_action.dispatch((t, name_edit.get(), String::new())); }
                                        }>"Save profile"</button>
                                    </div>
                                    <h4>"Signed-in sessions"</h4>
                                    <div class="row-list">
                                        {move || live_sessions().iter().map(|(d, _)| {
                                            let dd = d.clone();
                                            view! {
                                                <div class="row">
                                                    <span>{d.clone()}</span>
                                                    <button class="secondary small" on:click=move |_| {
                                                        if let Some(t) = read_token() { remove_action.dispatch((t, dd.clone())); }
                                                    }>"Remove"</button>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                                {move || flags.get().map(|f| view! {
                                    <div class="family-card">
                                        <h3>"Mesh access"</h3>
                                        <p>{if f.disable_new_users { "Adding identities is paused." } else { "Adding identities is open." }}</p>
                                    </div>
                                })}
                                {admin.then(|| view! {
                                    <div class="family-card">
                                        <h3>"Owner tools"</h3>
                                        {move || flags.get().map(|f| view! {
                                            <div class="btn-row">
                                                <button class="secondary" on:click=move |_| {
                                                    if let Some(t) = read_token() { flag_action.dispatch((t, !f.disable_new_users)); }
                                                }>{if f.disable_new_users { "Reopen adding identities" } else { "Pause adding identities" }}</button>
                                            </div>
                                        })}
                                        <div class="field">
                                            <label>"Grant to identity"</label>
                                            <input placeholder="did:peer:…" on:input=move |ev| set_grant_did.set(event_target_value(&ev))/>
                                        </div>
                                        <div class="field">
                                            <label>"Access level"</label>
                                            <select on:change=move |ev| set_grant_level.set(event_target_value(&ev))>
                                                <option value="read">"read"</option>
                                                <option value="write">"write"</option>
                                                <option value="admin">"admin"</option>
                                            </select>
                                        </div>
                                        <div class="btn-row">
                                            <button on:click=move |_| {
                                                if let Some(t) = read_token() { grant_action.dispatch((t, grant_did.get(), grant_level.get())); }
                                            }>"Grant"</button>
                                        </div>
                                        {move || { let n = note.get(); if n.is_empty() { view!{}.into_any() } else { view!{ <p class="notice">{n}</p> }.into_any() } }}
                                        <h4>"Waiting identities"</h4>
                                        <div class="row-list">
                                            {move || pending.get().map(|list| list.into_iter().map(|p| {
                                                let d = p.did.clone();
                                                view! {
                                                    <div class="row">
                                                        <span>{d.clone()}</span>
                                                        <button class="secondary small" on:click=move |_| {
                                                            if let Some(t) = read_token() { revoke_action.dispatch((t, d.clone())); }
                                                        }>"Revoke"</button>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>())}
                                        </div>
                                    </div>
                                })}
                            </section>
                        </main>
                    }.into_any()
                }
                None => view! {
                    <ModelsNavbar show_models=false display_name=String::new()/>
                    <main class="main page-main">
                        <header class="page-head">
                            <h1>"Settings."</h1>
                        </header>
                        <section class="page-cards">
                            <div class="family-card">
                                <h3>"Display"</h3>
                                <p>"Appearance follows the system or forces light or dark; content width fits the margins or fills the screen. Choices persist in browser storage for this browser."</p>
                                <div class="field">
                                    <label>"Appearance"</label>
                                    <ThemeDrop appearance=appearance set_appearance=set_appearance apply=apply.clone()/>
                                </div>
                                <div class="field">
                                    <label>"Content width"</label>
                                    <WidthToggle width=width set_width=set_width apply=apply.clone()/>
                                </div>
                            </div>
                        </section>
                        <p><A href="/signin">"Go to sign in"</A></p>
                    </main>
                }.into_any(),
            })}
        </Suspense>
    }
}
