use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    hooks::use_navigate,
    StaticSegment,
};

use crate::auth::{
    active_did, answer_nonce, approve_petition, clear_token, create_identity, delete_identity,
    docs_content, drop_session, gate_check, get_flags, grant_access, live_sessions, list_petitions,
    pending_list, petition_access, read_saved_secret, read_settings, read_token, request_nonce,
    refuse_petition, revoke_access, save_settings, set_flags, sign_out, stash_session,
    store_saved_secret, switch_session, update_profile, user_avatar, user_display_name,
    users_list, VisitorSettings,
};
use crate::mail::{create_mailbox, mailbox_list, remove_mailbox};
use crate::store::{registry_rows, ModelRow};

/// The visitor's stored display choices, applied by the browser to the root
/// element before the first paint.
///
/// The server renders static HTML and cannot know what the visitor's browser
/// holds, so without this the page paints in the default theme at the default
/// content width and then jumps when the client corrects it. The direction the
/// site takes is a persisted choice applied before the first paint, and this
/// is where that happens.
///
/// It is browser-only code: it ships in the markup and runs in the browser,
/// and it never runs during server rendering. Single quotes keep the text free
/// of the characters an HTML pass would rewrite.
const DISPLAY_BOOTSTRAP: &str = "(function(){try{var a=localStorage.getItem('mu_appearance');if(a){document.documentElement.setAttribute('data-appearance',a);}var w=localStorage.getItem('mu_width');if(w){document.documentElement.setAttribute('data-width',w);}}catch(e){}})();";

/// The display bootstrap as it is rendered into the document head.
///
/// The head is rendered by the server, and the browser build never renders
/// it — hydration mounts the body alone — so the Content Security Policy
/// nonce belongs to the server rendering. The server build asks Leptos for
/// the nonce it is rendering under, and the browser build renders the
/// script without one.
#[cfg(feature = "ssr")]
fn display_bootstrap() -> impl IntoView {
    view! { <script nonce=use_nonce()>{DISPLAY_BOOTSTRAP}</script> }
}

/// The display bootstrap as it is rendered into the document head, without a
/// Content Security Policy nonce, which the browser build does not carry.
#[cfg(not(feature = "ssr"))]
fn display_bootstrap() -> impl IntoView {
    view! { <script>{DISPLAY_BOOTSTRAP}</script> }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" data-appearance="system" data-width="normal">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0, viewport-fit=cover"/>
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link href="https://fonts.googleapis.com/css2?family=Inter:opsz,wght@14..32,300;14..32,400;14..32,500;14..32,600;14..32,700&family=Nunito:wght@200..1000&display=swap" rel="stylesheet"/>
                <AutoReload options=options.clone() />
                {display_bootstrap()}
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
    ("ExoTalk", "/cards/exotalk.png",
     "The Exosystem's flagship app: private, encrypted messaging that runs natively on your phone and desktop. It is your personal node on the network, carrying your identity and keeping every conversation peer to peer — connecting directly when possible and passing through blind relays only when it must. Every other Exosystem app unfolds inside it, arriving as subscriptions you manage in your settings, and it can even create and manage Conscia service nodes on your behalf. Because apps update through signed blueprints delivered over the mesh, the interface of everything you subscribe to evolves the moment its author publishes. Sign-ins, subscriptions, and app updates all flow through it, making one app the solid front door to the whole family."),
    ("ExoSoul", "/cards/exosoul.png",
     "The studio where every Exosystem app is authored — the press where each app is formed into a signed blueprint. One identity authors here; ExoTalk users subscribe, and the app unfolds on their devices with its interface driven by the blueprint, so a published change reaches every subscriber instantly. Every blueprint is a canonical app payload whose exact bytes are pinned by a digest, so what unfolds on a device is always precisely what was signed. Blueprints compose other blueprints only through digest-pinned includes with parent-owned overrides, keeping every app's lineage exact and verifiable. ExoSoul itself is a native app, and Conscia nodes host and relay its blueprints as a service."),
    ("Conscia", "/cards/conscia.png",
     "The lifeline of the Exosystem: always-on nodes that relay messages, discover peers, answer petitions, and keep data reachable while your devices are offline — opt-in helper services. Every node app runs peer to peer independently; when one is present it can carry relay, indexing, discovery, persistence, authentication, observability, and federation, routing geographically and by latency. Its relationship worker handles petitions, federation, and indexing while its operator console administers each node, in small, community, and large clustered flavors — with a large node planned at exosystems.net itself — and a Conscierge worker coordinates the nodes with one another and reaches beyond the mesh. Blind indexing can also surface community circles and public directories through one unified search surface."),
    ("Exonomy", "/cards/exonomy.png",
     "Social voucher exchange. Members — Exonomists — mint and trade vouchers that stand for real goods and services, optionally bought or sold for everyday money; the voucher itself carries the value. It arrives as an app inside ExoTalk, and while its vouchers can optionally fund Exocracy tasks, it operates entirely independently by default — a fully realized ecosystem of structured value exchange. Native apps carry it onto phones and desktops for managing heavy voucher ledgers, and an indexing node hosts encrypted voucher payloads blindly, surfacing only the metadata its owners choose to publish."),
    ("Exocracy", "/cards/exocracy.png",
     "Transparent project governance — the governance layer of the mesh. Communities run projects with open proposals, votes, and milestones, funded with ordinary money or, optionally, Exonomy vouchers; that funding choice is engineered interoperability, and a project runs entirely on ordinary money when its manager prefers. Built for high-trust environments where organizational transparency and independent project management matter most, it connects with ExoTalk, Exonomy, and the rest of the family while standing entirely on its own. Native apps put real-time voting and task tracking in your pocket, and an indexing node keeps open tasks and bounties publicly searchable."),
    ("RepubLet", "/cards/republet.png",
     "Decentralized scientific publishing and peer review — a modern Republic of Letters, the knowledge ledger of the mesh. Researchers' work lives on permanent, hash-anchored records with cryptographically verifiable authorship, and author-facing paywalls offer a decentralized alternative to subscription platforms for earning from one's own findings — settled in ordinary money, cryptocurrency, or, optionally, Exonomy vouchers. A scientist who initiates an Exocracy project to manage and fund research is stringing together independent modular tools. Native apps carry reading and peer-review notifications onto phones and desktops, and a citation relay keeps open-science metadata publicly searchable."),
    ("ThreeSteps (3S)", "/cards/threesteps.png",
     "A private, offline-first social app for Catholic families and communities — helping children grow in faith through Baptism, First Communion, and Confirmation. Here social means relationships, guidance, and formation, and privacy means total discretion for those practicing in regions where faith requires it. Conversations connect directly when possible and pass through blind relays when they must, with local-first storage and cryptographic access control — so it keeps working through limited connectivity. Identity flows through ExoAuth, the family's shared identity layer, and missionaries, catechists, and parish groups meet the wider Exosystem through Conscia nodes."),
    ("InterNovel", "/cards/internovel.png",
     "Collaborative storytelling for young authors. Each writer's contributions are digitally signed — every paragraph permanently linked to the identity that wrote it — so authorship stays verifiable and intellectual property stays with its creator. Characters can travel between stories through the Travel Trunk, the Novel Spine holds each story's structure, and novels are written together under unanimous consent. Flexible collaboration, clear attribution, and protected ownership make it a writing home that grows with its authors, and each novel unfolds inside ExoTalk as one of the family's apps."),
];

/* Display-gadget icons from Lucide (ISC licensed), drawn as strokes on a
 * 24-unit grid so they hold their shape at every size. */
const ICON_MONITOR: &str = "<rect width=\"20\" height=\"14\" x=\"2\" y=\"3\" rx=\"2\"/><line x1=\"8\" x2=\"16\" y1=\"21\" y2=\"21\"/><line x1=\"12\" x2=\"12\" y1=\"17\" y2=\"21\"/>";
const ICON_SUN: &str = "<circle cx=\"12\" cy=\"12\" r=\"4\"/><path d=\"M12 2v2\"/><path d=\"M12 20v2\"/><path d=\"m4.93 4.93 1.41 1.41\"/><path d=\"m17.66 17.66 1.41 1.41\"/><path d=\"M2 12h2\"/><path d=\"M20 12h2\"/><path d=\"m6.34 17.66-1.41 1.41\"/><path d=\"m19.07 4.93-1.41 1.41\"/>";
const ICON_MOON: &str = "<path d=\"M20.985 12.486a9 9 0 1 1-9.473-9.472c.405-.022.617.46.402.803a6 6 0 0 0 8.268 8.268c.344-.215.825-.004.803.401\"/>";
const ICON_WIDTH: &str = "<path d=\"M8 3H5a2 2 0 0 0-2 2v3\"/><path d=\"M21 8V5a2 2 0 0 0-2-2h-3\"/><path d=\"M3 16v3a2 2 0 0 0 2 2h3\"/><path d=\"M16 21h3a2 2 0 0 0 2-2v-3\"/>";
const ICON_WIDTH_FIT: &str = "<path d=\"M8 3v3a2 2 0 0 1-2 2H3\"/><path d=\"M21 8h-3a2 2 0 0 1-2-2V3\"/><path d=\"M3 16h3a2 2 0 0 1 2 2v3\"/><path d=\"M16 21v-3a2 2 0 0 1 2-2h3\"/>";
const ICON_GEAR: &str = "<path d=\"M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915\"/><circle cx=\"12\" cy=\"12\" r=\"3\"/>";

/// Whether the client has taken over from the server's markup.
///
/// The server renders static HTML and cannot know what the visitor's browser
/// holds, so the first client render has to match what the server sent or
/// hydration leaves the two disagreeing. Anything that renders a value the
/// server cannot know waits for this flag before it shows the real thing.
/// There is one flag for the whole application: a per-component copy would
/// drift, and the drift is invisible until a page is reloaded.
#[derive(Clone, Copy)]
pub struct ClientReady(pub ReadSignal<bool>);

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let prefs = display_prefs();
    provide_context(prefs);
    let (client_ready, set_client_ready) = signal(false);
    Effect::new(move |_| set_client_ready.set(true));
    provide_context(ClientReady(client_ready));

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
    models: bool,
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
                models: gate.models,
                admin: gate.admin,
            })
        },
    )
}

#[component]
fn FloatingGadgets(identity: Resource<Option<IdentityState>>) -> impl IntoView {
    let users = Resource::new(|| (), |_| async move { users_list().await.unwrap_or_default() });
    let (chip_open, set_chip_open) = signal(false);
    let prefs = shared_prefs();

    view! {
        
        <div class="navbar">
            <div class="brandrow">
            <img src="/exosys.png" alt="ExoSystems" class="navbar-logo"/>
            <div class="navbar-menu">
                {move || identity.get().flatten().filter(|id| id.models).map(|_| view! { <A href="/models">"Models"</A> })}
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
        <div class="carousel-tools">
            <DisplayGadgets prefs=prefs show_settings=true/>
        </div>
        
    }
}

/// The three themes, each with the glyph that stands for it and the name it
/// carries. The name is the one the root element holds in `data-appearance`,
/// the one the stylesheet keys on and the one the browser stores, so a theme
/// is named in one place and cannot drift from its own name.
const THEMES: [(&str, &str, &str); 3] = [
    ("light", ICON_SUN, "Light theme"),
    ("dark", ICON_MOON, "Dark theme"),
    ("system", ICON_MONITOR, "System theme"),
];

/// The name, glyph and label of a theme.
fn theme(v: &str) -> (&'static str, &'static str, &'static str) {
    THEMES
        .iter()
        .find(|(name, _, _)| *name == v)
        .copied()
        .unwrap_or(THEMES[2])
}

fn theme_label(v: &str) -> &'static str {
    theme(v).2
}

/// The glyph for a theme, marked with the theme's name so the stylesheet can
/// show the one the visitor's choice names. The class prefix separates the
/// trigger's glyphs, which the stylesheet switches between, from the options'
/// glyphs, which are always shown.
fn theme_glyph(v: &str, prefix: &str) -> impl IntoView {
    let (name, glyph, _) = theme(v);
    view! { <g class=format!("{prefix} {prefix}-{name}") inner_html=glyph/> }
}

/// All three glyphs, one of them shown: the stylesheet shows the one the root
/// element's `data-appearance` names. The visitor's choice reaches the root
/// element before the first paint, so the icon is right from the first frame
/// and the client never has to correct the server's markup.
fn theme_icons() -> impl IntoView {
    view! {
        {THEMES.iter().map(|(name, glyph, _)| view! {
            <g class=format!("ti ti-{name}") inner_html=*glyph/>
        }).collect::<Vec<_>>()}
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
    let client = use_context::<ClientReady>().expect("ClientReady provided at App root");
    // The tooltip and the accessible name are text the server cannot know, so
    // they wait for the client. The glyph beside them does not: the
    // stylesheet reads it from the root element, which the browser has
    // already set before the first paint.
    let label = move || {
        if client.0.get() {
            theme_label(&appearance.get())
        } else {
            theme_label("system")
        }
    };
    view! {
        <span class="icon-drop-wrap">
            <button type="button" class="ctl ctl-theme"
                title=move || label()
                aria-label=move || label()
                on:click=move |_| set_drop_open.set(!drop_open.get())>
                <svg viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                    {theme_icons()}
                </svg>
            </button>
            {move || drop_open.get().then(|| {
                let current = appearance.get();
                view! {
                    <span class="drop-backdrop" on:click=move |_| set_drop_open.set(false)></span>
                    <span class="icon-drop-panel">
                        {["system", "light", "dark"].into_iter().map(|v| {
                            let a = apply_panel.clone();
                            let is_current = v == current.as_str();
                            view! {
                                <button type="button" class="ctl" class:current=move || is_current
                                    title=theme_label(v) aria-label=theme_label(v)
                                    on:click=move |_| {
                                        set_appearance.set(v.to_string());
                                        a();
                                        set_drop_open.set(false);
                                    }>
                                    <svg viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                        {theme_glyph(v, "oi")}
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
    // The server renders defaults, so the button trusts that markup on load
    // and would keep showing it until the next click. After the client has
    // taken over, the live values take over, keeping the tooltip true to the
    // persisted choice on every load.
    let client = use_context::<ClientReady>().expect("ClientReady provided at App root");
    view! {
        <button type="button" class="ctl ctl-width" class:active=move || client.0.get() && width.get() == "wide"
            title=move || if client.0.get() && width.get() == "wide" { "Fit to margins" } else { "Full width" }
            aria-label=move || if client.0.get() && width.get() == "wide" { "Fit to margins" } else { "Full width" }
            on:click=move |_| { set_width.set(if width.get() == "wide" { "normal".into() } else { "wide".into() }); apply(); }>
            <svg viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                {width_icons()}
            </svg>
        </button>
    }
}

/// Both width glyphs, one of them shown: the stylesheet shows the one the
/// root element's `data-width` names, the same way the theme glyph follows
/// `data-appearance`.
fn width_icons() -> impl IntoView {
    view! {
        <g class="wi wi-wide" inner_html=ICON_WIDTH/>
        <g class="wi wi-normal" inner_html=ICON_WIDTH_FIT/>
    }
}

#[component]
fn SettingsGear() -> impl IntoView {
    view! {
        <a href="/settings" class="ctl" title="Settings" aria-label="Settings">
            <svg viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                <g inner_html=ICON_GEAR/>
            </svg>
        </a>
    }
}

/// Display preferences are shared by every gadget instance on the page, so the
/// gadgets in the top bar and the gadgets inside a page card always agree.
#[derive(Clone, Copy)]
pub struct DisplayPrefs {
    pub appearance: ReadSignal<String>,
    pub width: ReadSignal<String>,
    pub set_appearance: WriteSignal<String>,
    pub set_width: WriteSignal<String>,
}

/// One set of display preferences for the whole application, so every gadget
/// instance on every surface reads and writes the same state. The pair is
/// created once in `App` and shared by context: one pair per render tree,
/// which stays SSR-safe because no signal outlives its request owner.
fn shared_prefs() -> DisplayPrefs {
    use_context::<DisplayPrefs>().expect("DisplayPrefs provided at App root")
}

fn display_prefs() -> DisplayPrefs {
    let (appearance, set_appearance) = signal("system".to_string());
    let (width, set_width) = signal("normal".to_string());
    try_load_browser_settings(&set_appearance, &set_width);
    let prefs = DisplayPrefs {
        appearance,
        width,
        set_appearance,
        set_width,
    };
    let a = appearance;
    let w = width;
    let _ = Effect::new(move |_| {
        apply_browser_settings(&a.get(), &w.get());
    });
    prefs
}

#[component]
fn DisplayGadgets(
    prefs: DisplayPrefs,
    show_settings: bool,
) -> impl IntoView {
    let appearance = prefs.appearance;
    let set_appearance = prefs.set_appearance;
    let width = prefs.width;
    let set_width = prefs.set_width;
    let apply = apply_display(appearance, width);
    view! {
        <span class="gadget-row">
            <ThemeDrop appearance=appearance set_appearance=set_appearance apply=apply.clone()/>
            <WidthToggle width=width set_width=set_width apply=apply/>
            {show_settings.then(|| view! { <SettingsGear/> }.into_any())}
        </span>
    }
}

#[component]
fn Tagline() -> impl IntoView {
    view! {
        <div class="tagline-block">
            <p><span class="t-stmt"><span class="t1">"Hello World! We're doing something new."</span>{" "}<span class="t2">"Singular by design, connected by nature, and conscious by consensus."</span></span><span class="t-sub">"... for the software was made for the man, not the man for the software!"</span></p>
        </div>
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
                <Tagline/>
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
        <div class="flip" on:click=move |_| {
            set_flipped.set(!flipped.get());
            #[cfg(target_arch = "wasm32")]
            if let Some(el) = full_ref.get() {
                if let Ok(ev) = web_sys::Event::new("scroll") {
                    let _ = el.dispatch_event(&ev);
                }
            }
        }>
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
/// The models page carries its own solid top navigation bar with the
/// gadgetry and no carousel. Sign-in runs only on the main page; this
/// route recognizes the main-page session instead of running its own.
/// The current route never lists a menu item pointing to itself.
fn ModelsNavbar(show_models: bool, display_name: String, current: &'static str) -> impl IntoView {
    let prefs = shared_prefs();
    view! {
        <nav class="navbar navbar-solid">
            <img src="/exosys.png" alt="ExoSystems" class="navbar-logo"/>
            <div class="navbar-menu">
                <A href="/">"Home"</A>
                {show_models.then(|| if current != "models" {
                    view! { <A href="/models">"Models"</A> }.into_any()
                } else { view! {}.into_any() })}
                {if display_name.is_empty() {
                    if current != "signin" {
                        view! { <A href="/signin">"Sign in"</A> }.into_any()
                    } else { view! {}.into_any() }
                } else { view! { <span class="chip-name">{display_name}</span> }.into_any() }}
                <DisplayGadgets prefs=prefs show_settings=current != "settings"/>
            </div>
        </nav>
    }
}

/// Uniform template for the section pages: the solid top bar plus the
/// page-main shell. Each page's heading and content ride inside as children,
/// so every section page shares one appearance and one layout.
#[component]
fn SectionPage(
    show_models: bool,
    display_name: String,
    current: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <ModelsNavbar show_models=show_models display_name=display_name current=current/>
        <main class="main page-main">{children()}</main>
    }
}

/// The flat section heading the account pages share: title over one line.
#[component]
fn SectionHead(title: &'static str, subtitle: Option<String>) -> impl IntoView {
    view! {
        <header class="page-head">
            <h1>{title}</h1>
            {subtitle.map(|s| view! { <p>{s}</p> })}
        </header>
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
    let (save_secret, set_save_secret) = signal(false);
    let (notice, set_notice) = signal(String::new());
    let (answered, set_answered) = signal(String::new());
    let (petition_did, set_petition_did) = signal(String::new());
    let (petition_open, set_petition_open) = signal(false);
    let navigate = use_navigate();
    let nonce_action = Action::new(|did: &String| {
        let did = did.clone();
        async move { request_nonce(did).await }
    });
    let create_action = Action::new(|input: &(String, String)| {
        let (name, avatar) = input.clone();
        async move { create_identity(name, avatar).await }
    });
    let petition_action = Action::new(move |did: &String| {
        let did = did.clone();
        let set_petition_open = set_petition_open;
        let set_petition_did = set_petition_did;
        let set_notice = set_notice;
        async move {
            match petition_access(did.clone(), String::new()).await {
                Ok(()) => {
                    set_petition_open.set(false);
                    set_petition_did.set(String::new());
                }
                Err(e) => set_notice.set(format!("Petition not sent: {e}")),
            }
        }
    });

    Effect::new_sync(move |_| {
        let nav = navigate.clone();
        if let Some(Ok(nonce)) = nonce_action.value().get() {
            if answered.get() == nonce {
                return;
            }
            set_answered.set(nonce.clone());
            let did = did_input.get();
            let nonce_send = nonce.clone();
            let should_save = save_secret.get();
            match sign_challenge(&secret_input.get(), &nonce) {
                Ok(sig) => {
                    leptos::task::spawn_local(async move {
                        match answer_nonce(did.clone(), nonce_send, sig).await {
                            Ok(result) => {
                                stash_session(&did, &result.token);
                                if should_save {
                                    store_saved_secret(&did, &secret_input.get());
                                }
                                if result.needs_approval {
                                    set_petition_did.set(did);
                                    set_petition_open.set(true);
                                } else {
                                    nav("/", Default::default());
                                }
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
            if save_secret.get() {
                store_saved_secret(&fresh.did, &fresh.secret);
            }
            set_notice.set(format!(
                "New identity {} signed in. Its secret is {}. It is shown once, and it is the key to this identity. The owner grants powers on the settings page.",
                fresh.did, fresh.secret
            ));
        } else if let Some(Err(e)) = create_action.value().get() {
            set_notice.set(format!("The identity was not created: {e}"));
        }
    });

    view! {
        <SectionPage show_models=false display_name=String::new() current="signin">
            <SectionHead title="Sign in." subtitle=Some("Your identity lives with you. Sign in with your peer identity and stay on ExoSystems.net.".to_string())/>
            <section class="page-cards">
                <div class="family-card">
                    <h3>"Known identities"</h3>
                    <p>"Identities the local Willow store knows. Pick one, prove its secret, and continue."</p>
                    <div class="id-list">
                        {move || users.get().map(|list| list.into_iter().map(|u| {
                                let did = u.did.clone();
                                let name = u.display_name.clone();
                                let initial = name.chars().next().unwrap_or('?').to_uppercase().to_string();
                                let saved = read_saved_secret(&did);
                                view! {
                                    <button on:click=move |_| {
                                        set_notice.set(String::new());
                                        set_did_input.set(did.clone());
                                        if let Some(saved_secret) = &saved {
                                            set_secret_input.set(saved_secret.clone());
                                            set_save_secret.set(true);
                                        }
                                        nonce_action.dispatch(did.clone());
                                    }>
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
                    <div class="field">
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=save_secret
                                on:change=move |_| set_save_secret.update(|v| *v = !*v)
                            />
                            "Save this secret on this device"
                        </label>
                        <p class="notice">"Saved in browser storage. Anyone with access to this device can use it."</p>
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
                            <div class="field">
                                <label>
                                    <input
                                        type="checkbox"
                                        prop:checked=save_secret
                                        on:change=move |_| set_save_secret.update(|v| *v = !*v)
                                    />
                                    "Save this secret on this device"
                                </label>
                                <p class="notice">"Saved in browser storage. Anyone with access to this device can use it."</p>
                            </div>
                        }.into_any()
                    })}
                </div>
            </section>
            {move || {
                if petition_open.get() {
                    let did = petition_did.get();
                    view! {
                        <div class="petition-modal">
                            <h3>"Petition this node for access"</h3>
                            <p>
                                "Your identity "
                                <strong>{did.clone()}</strong>
                                " proved its secret, but this Conscia node does not know you yet. "
                                "Access requires approval from the node owner. "
                                "Petition now and the owner can approve or refuse your request."
                            </p>
                            <div class="btn-row">
                                <button on:click=move |_| {
                                    petition_action.dispatch(did.clone());
                                }>"Send petition"</button>
                                <button on:click=move |_| {
                                    set_petition_open.set(false);
                                    set_petition_did.set(String::new());
                                }>"Cancel"</button>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view!{}.into_any()
                }
            }}
        </SectionPage>
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
                let named = models_gate.get().flatten().map(|v| (v.models, v.did)).unwrap_or((false, String::new()));
                view! {
                    <SectionPage show_models=named.0 display_name=named.1 current="docs">
                        <SectionHead title="Docs." subtitle=None/>
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
                    </SectionPage>
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
                Some(v) if v.models => {
                    let name = v.did.clone();
                    view! {
                        <SectionPage show_models=true display_name=name current="models">
                            <section class="runs"><h2>"Runs"</h2></section>
                            <section class="models">
                                <h2>"Models"</h2>
                                <ModelsTable/>
                            </section>
                        </SectionPage>
                    }.into_any()
                }
                _ => view! {
                    <SectionPage show_models=false display_name=String::new() current="models">
                        <p><A href="/signin">"Sign in with a delegated identity to read the registry."</A></p>
                    </SectionPage>
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
                    <div class="table-scroll">
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
                    </div>
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
    let prefs = shared_prefs();
    let appearance = prefs.appearance;
    let set_appearance = prefs.set_appearance;
    let width = prefs.width;
    let set_width = prefs.set_width;
    let (note, set_note) = signal(String::new());
    let (name_edit, set_name_edit) = signal(String::new());

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
    let petitions = Resource::new(
        || (),
        |_| async move {
            match read_token() {
                Some(t) => list_petitions(t).await.unwrap_or_default(),
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
    let approve_action = Action::new(|input: &(String, String)| {
        let (token, did) = input.clone();
        async move { approve_petition(token, did).await }
    });
    let refuse_action = Action::new(|input: &(String, String)| {
        let (token, did) = input.clone();
        async move { refuse_petition(token, did).await }
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
    let mailboxes = Resource::new(
        || (),
        |_| async move {
            match read_token() {
                Some(t) => mailbox_list(t).await.unwrap_or_default(),
                None => vec![],
            }
        },
    );
    let (mail_local, set_mail_local) = signal(String::new());
    let (mail_name, set_mail_name) = signal(String::new());
    let (mail_msg, set_mail_msg) = signal(String::new());
    let (mail_err, set_mail_err) = signal(false);
    let create_mailbox_action = Action::new(|input: &(String, String, String)| {
        let (token, local_part, display_name) = input.clone();
        async move { create_mailbox(token, local_part, display_name).await }
    });
    let remove_mailbox_action = Action::new(|input: &(String, String)| {
        let (token, local_part) = input.clone();
        async move { remove_mailbox(token, local_part).await }
    });
    Effect::new_sync(move |_| {
        if let Some(Ok(created)) = create_mailbox_action.value().get() {
            set_mail_msg.set(format!(
                "{} created. Password: {}. It is shown once.",
                created.address, created.password
            ));
            set_mail_err.set(false);
            set_mail_local.set(String::new());
            set_mail_name.set(String::new());
            mailboxes.refetch();
        } else if let Some(Err(e)) = create_mailbox_action.value().get() {
            set_mail_msg.set(format!("{e}"));
            set_mail_err.set(true);
        }
    });
    Effect::new_sync(move |_| {
        if let Some(Ok(())) = remove_mailbox_action.value().get() {
            set_mail_msg.set("Mailbox removed. The mail it received stays on the instance as the archive.".to_string());
            set_mail_err.set(false);
            mailboxes.refetch();
        } else if let Some(Err(e)) = remove_mailbox_action.value().get() {
            set_mail_msg.set(format!("{e}"));
            set_mail_err.set(true);
        }
    });
    Effect::new_sync(move |_| {
        for r in [grant_action.value().get(), revoke_action.value().get(), approve_action.value().get(), refuse_action.value().get(), profile_action.value().get(), remove_action.value().get(), flag_action.value().get()].into_iter().flatten() {
            match r {
                Ok(()) => {
                    set_note.set("Saved.".to_string());
                    pending.refetch();
                    petitions.refetch();
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
                    let named = (v.models, v.did.clone());
                    let apply = apply.clone();
                    view! {
<SectionPage show_models=named.0 display_name=named.1 current="settings">
                        <SectionHead title="Settings." subtitle=Some(format!("Signed in as {did}"))/>
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
                                        <button class="secondary" on:click=move |_| {
                                            let t = read_token().unwrap_or_default();
                                            let d = active_did().unwrap_or_default();
                                            logout.dispatch((t, d));
                                        }>"Sign out"</button>
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
                                {admin.then(|| view! {
                                    <div class="family-card">
                                        <h3>"Pending petitions"</h3>
                                        <p>"Identities that proved their secret but need your approval before they can sign in."</p>
                                        <div class="row-list">
                                            {move || petitions.get().map(|list| list.into_iter().map(|p| {
                                                let d = p.did.clone();
                                                let name = p.display_name.clone().unwrap_or_default();
                                                let d_approve = d.clone();
                                                let d_refuse = d.clone();
                                                view! {
                                                    <div class="row">
                                                        <span>{format!("{} ({})", d, if name.is_empty() { "no name" } else { &name })}</span>
                                                        <button class="secondary small" on:click=move |_| {
                                                            if let Some(t) = read_token() { approve_action.dispatch((t, d_approve.clone())); }
                                                        }>"Approve"</button>
                                                        <button class="secondary small" on:click=move |_| {
                                                            if let Some(t) = read_token() { refuse_action.dispatch((t, d_refuse.clone())); }
                                                        }>"Refuse"</button>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>())}
                                        </div>
                                    </div>
                                })}
                                {admin.then(|| view! {
                                    <div class="family-card">
                                        <h3>"Mailboxes"</h3>
                                        <p>"The addresses on this domain that receive mail. Each one is an account on the instance, with its own mail store and its own webmail user."</p>
                                        <div class="row-list">
                                            {move || mailboxes.get().map(|list| list.into_iter().map(|m| {
                                                let lp = m.local_part.clone();
                                                let address = m.address.clone();
                                                let name = m.display_name.clone();
                                                view! {
                                                    <div class="row">
                                                        <span>{address}<small>{name}</small></span>
                                                        <button class="secondary small" on:click=move |_| {
                                                            if let Some(t) = read_token() { remove_mailbox_action.dispatch((t, lp.clone())); }
                                                        }>"Remove"</button>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>())}
                                        </div>
                                        <div class="field">
                                            <label>"Address"</label>
                                            <input placeholder="billing" on:input=move |ev| set_mail_local.set(event_target_value(&ev))/>
                                        </div>
                                        <div class="field">
                                            <label>"Display name"</label>
                                            <input placeholder="ExoBilling" on:input=move |ev| set_mail_name.set(event_target_value(&ev))/>
                                        </div>
                                        <div class="btn-row">
                                            <button on:click=move |_| {
                                                if let Some(t) = read_token() { create_mailbox_action.dispatch((t, mail_local.get(), mail_name.get())); }
                                            }>"Add mailbox"</button>
                                        </div>
                                        {move || {
                                            let m = mail_msg.get();
                                            if m.is_empty() { view!{}.into_any() } else {
                                                let cls = if mail_err.get() { "notice".to_string() } else { "notice ok".to_string() };
                                                view!{ <p class=cls>{m}</p> }.into_any()
                                            }
                                        }}
                                    </div>
                                })}
                            </section>
                        </SectionPage>
                    }.into_any()
                }
                None => {
                    let apply = apply.clone();
                    view! {
                    <SectionPage show_models=false display_name=String::new() current="settings">
                        <SectionHead title="Settings." subtitle=Some("Choices persist in browser storage for this browser.".to_string())/>
                        <section class="page-cards">
                            <div class="family-card">
                                <h3>"Display"</h3>
                                <p>"Appearance follows the system or forces light or dark; content width fits the margins or fills the screen."</p>
                                <div class="field">
                                    <label>"Appearance"</label>
                                    <ThemeDrop appearance=appearance set_appearance=set_appearance apply=apply.clone()/>
                                </div>
                                <div class="field">
                                    <label>"Content width"</label>
                                    <WidthToggle width=width set_width=set_width apply=apply.clone()/>
                                </div>
                                <p><A href="/signin">"Go to sign in"</A></p>
                            </div>
                        </section>
                    </SectionPage>
                    }
                    .into_any()
                }
            })}
        </Suspense>
    }
}
