// macOS-only venster-uiterlijk voor het systeemthema (#240).
//
// Een webview is één vlak NSView. Om de zijbalk Liquid Glass te geven legt dit
// module een native view ONDER de webview, precies op de rechthoek van de zijbalk,
// en laat de webview daar doorkijken: hij tekent zijn eigen achtergrond niet meer
// (drawsBackground = NO, dezelfde sleutel die wry gebruikt), en de pagina maakt in
// het systeemthema alleen de zijbalk doorzichtig. De rest blijft ondoorzichtig.
//
// macOS 26+: NSGlassEffectView (Liquid Glass). Daarvoor: NSVisualEffectView met
// het materiaal `sidebar` (klassieke vibrancy). Alles hier draait op de main thread.

use std::cell::RefCell;
use std::ffi::c_void;

use objc2::rc::Retained;
use objc2::runtime::AnyClass;
use objc2::{msg_send, MainThreadMarker};
use objc2_app_kit::{
    NSColor, NSColorSpace, NSGlassEffectView, NSView, NSVisualEffectBlendingMode,
    NSVisualEffectMaterial, NSVisualEffectState, NSVisualEffectView, NSWindowOrderingMode,
};
use objc2_foundation::{NSNumber, NSPoint, NSRect, NSSize, NSString};

thread_local! {
    // De view achter de zijbalk. Eén per venster; Taurus heeft er één.
    static BACKDROP: RefCell<Option<Retained<NSView>>> = const { RefCell::new(None) };
    // Dezelfde view als NSGlassEffectView, als het er een is (voor de hoekradius).
    static GLASS: RefCell<Option<Retained<NSGlassEffectView>>> = const { RefCell::new(None) };
}

// De accentkleur die de gebruiker in Systeeminstellingen koos, als #rrggbb.
pub fn accent_hex() -> Option<String> {
    let c = NSColor::controlAccentColor();
    let c = c.colorUsingColorSpace(&NSColorSpace::sRGBColorSpace())?;
    let to = |v: f64| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    Some(format!(
        "#{:02x}{:02x}{:02x}",
        to(c.redComponent()),
        to(c.greenComponent()),
        to(c.blueComponent())
    ))
}

pub fn glass_supported() -> bool {
    AnyClass::get(c"NSGlassEffectView").is_some()
}

// Zet de achter-view op `rect` (CSS-pixels = punten, oorsprong linksboven van de
// webview), of verberg hem bij None. Geeft terug of er nu een achter-view staat.
//
// SAFETY: `webview` is de WKWebView-pointer die Tauri's with_webview levert, en
// deze functie draait op de main thread (with_webview garandeert dat).
pub unsafe fn set_sidebar_backdrop(
    webview: *mut c_void,
    rect: Option<(f64, f64, f64, f64, f64, f64)>,
    see_through: bool,
) -> bool {
    let Some(mtm) = MainThreadMarker::new() else { return false };
    let wv: &NSView = &*(webview as *const NSView);
    let Some(parent) = wv.superview() else { return false };

    // vh = window.innerHeight: de hoogte van de PAGINA. De webview zelf loopt door
    // onder de titelbalk (GEMETEN: frame 660 pt, pagina 628 pt), dus rekenen vanaf
    // de onderkant van de pagina, niet vanaf de bovenkant van de webview.
    let Some((x, y, w, h, radius, vh)) = rect else {
        BACKDROP.with(|b| {
            if let Some(v) = b.borrow().as_ref() {
                v.setHidden(true);
            }
        });
        set_window_see_through(wv, false);
        return false;
    };

    // De webview laat voortaan door waar de pagina doorzichtig is. Onschuldig in
    // de andere thema's: daar is elke pagina-achtergrond ondoorzichtig.
    let no = NSNumber::new_bool(false);
    let key = NSString::from_str("drawsBackground");
    let _: () = msg_send![wv, setValue: &*no, forKey: &*key];
    // GEMETEN: daarna werd de titelbalk wit in donkere modus. De webview loopt
    // door onder de titelbalk, en daar tekende hij zijn underPageBackgroundColor
    // (licht). Doorzichtig maken laat de vensterachtergrond van het systeem zien.
    let clear = NSColor::clearColor();
    let _: () = msg_send![wv, setUnderPageBackgroundColor: &*clear];

    BACKDROP.with(|b| {
        let mut slot = b.borrow_mut();
        if slot.is_none() {
            let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(w, h));
            let view: Retained<NSView> = if glass_supported() {
                let g = NSGlassEffectView::initWithFrame(mtm.alloc(), frame);
                g.setCornerRadius(radius);
                GLASS.with(|s| *s.borrow_mut() = Some(g.clone()));
                Retained::into_super(g)
            } else {
                let v = NSVisualEffectView::initWithFrame(mtm.alloc(), frame);
                v.setMaterial(NSVisualEffectMaterial::Sidebar);
                v.setBlendingMode(NSVisualEffectBlendingMode::BehindWindow);
                v.setState(NSVisualEffectState::FollowsWindowActiveState);
                Retained::into_super(v)
            };
            parent.addSubview_positioned_relativeTo(&view, NSWindowOrderingMode::Below, Some(wv));
            *slot = Some(view);
        }
        let view = slot.as_ref().unwrap();
        // NSView-coördinaten lopen van onder naar boven, tenzij de ouder geflipt is.
        let wf = wv.frame();
        let ny = if parent.isFlipped() {
            wf.origin.y + (wf.size.height - vh) + y
        } else {
            wf.origin.y + vh - (y + h)
        };
        view.setFrame(NSRect::new(NSPoint::new(wf.origin.x + x, ny), NSSize::new(w, h)));
        GLASS.with(|s| {
            if let Some(g) = s.borrow().as_ref() {
                g.setCornerRadius(radius);
            }
        });
        view.setHidden(false);
    });
    set_window_see_through(wv, see_through);
    true
}

// Glas buigt en vervaagt wat erachter ligt. Boven de effen vensterachtergrond
// valt er niets te buigen; met een doorzichtig venster ligt het glas boven het
// bureaublad (instelling "Doorzichtige zijbalk", #249). Alleen waar de pagina
// zelf doorzichtig is (de zijbalk); vensters erachter schemeren dan ook door.
unsafe fn set_window_see_through(wv: &NSView, on: bool) {
    let Some(win) = wv.window() else { return };
    let color = if on { NSColor::clearColor() } else { NSColor::windowBackgroundColor() };
    let _: () = msg_send![&*win, setOpaque: !on];
    let _: () = msg_send![&*win, setBackgroundColor: &*color];
}
