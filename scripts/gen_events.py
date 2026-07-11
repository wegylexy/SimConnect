"""Regenerates simconnect-proto/src/events/client/*.rs from the official
SimConnect key-event docs.

Cross-references the MSFS2020 SDK docs (docs.flightsimulator.com/html/...)
against the MSFS2024 SDK docs (docs.flightsimulator.com/msfs2024/html/...)
so names present in both are ungated and names new in 2024 get
`#[cfg(feature = "sunrise")]`. Parses raw HTML directly (via parse_events.py)
-- NOT a summarizing fetch tool, which previously mangled a large fraction
of these names (see events/client/mod.rs's doc comment for the story).

Usage:
    python scripts/gen_events.py --fetch   # download the doc pages, then regenerate
    python scripts/gen_events.py           # regenerate .rs files from the local cache

scripts/event_docs_cache/ is gitignored (raw vendor HTML, not source) -- run
with `--fetch` at least once after a fresh checkout, or whenever a new MSFS
SDK docs revision ships and you want to pick up additions/renames/removals.
Without `--fetch`, this only touches the local cache dir, never the network.
"""
import sys, re, os, html as htmlmod, urllib.request

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from parse_events import parse_file

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
DIR = os.path.join(SCRIPT_DIR, 'event_docs_cache')
OUT = os.path.join(SCRIPT_DIR, '..', 'simconnect-proto', 'src', 'events', 'client')

PAGE_2020 = {
    'autopilot': 'Aircraft_Autopilot_Flight_Assist_Events',
    'electrical': 'Aircraft_Electrical_Events',
    'engine': 'Aircraft_Engine_Events',
    'flight_control': 'Aircraft_Flight_Control_Events',
    'fuel_system': 'Aircraft_Fuel_System_Events',
    'instrumentation': 'Aircraft_Instrumentation_Events',
    'aircraft_misc': 'Aircraft_Misc_Events',
    'radio_navigation': 'Aircraft_Radio_Navigation_Events',
    'helicopter': 'Helicopter_Specific_Events',
    'miscellaneous': 'Miscellaneous_Events',
    'view_camera': 'View_Camera_Events',
}
PAGE_2024 = dict(PAGE_2020, general_systems='Aircraft_General_Systems', balloon='Balloon_Airship_Events')

def fetch_all():
    os.makedirs(DIR, exist_ok=True)
    req_headers = {'User-Agent': 'Mozilla/5.0'}
    for cat, page in PAGE_2020.items():
        url = f"https://docs.flightsimulator.com/html/Programming_Tools/Event_IDs/{page}.htm"
        _fetch(url, os.path.join(DIR, f"2020_{cat}.html"), req_headers)
    for cat, page in PAGE_2024.items():
        url = f"https://docs.flightsimulator.com/msfs2024/html/6_Programming_APIs/Key_Events/{page}.htm"
        _fetch(url, os.path.join(DIR, f"2024_{cat}.html"), req_headers)

def _fetch(url, dest, headers):
    print(f"fetching {url}")
    req = urllib.request.Request(url, headers=headers)
    with urllib.request.urlopen(req) as resp:
        data = resp.read()
    with open(dest, 'wb') as f:
        f.write(data)

CATS_2020 = ['autopilot','electrical','engine','flight_control','fuel_system',
             'instrumentation','aircraft_misc','radio_navigation','helicopter',
             'miscellaneous','view_camera']
CATS_2024_ONLY = ['general_systems','balloon']

def unesc(s):
    return htmlmod.unescape(s)

def load(cat, year):
    path = os.path.join(DIR, f"{year}_{cat}.html")
    try:
        return parse_file(path)
    except FileNotFoundError:
        return None

# Build global maps: name -> dict(desc, params, deprecated, heading, cat)
def build_global(cats, year):
    g = {}
    for cat in cats:
        sections = load(cat, year)
        if sections is None:
            continue
        for s in sections:
            for name, desc, params in s['events']:
                g[name] = {
                    'desc': unesc(desc), 'params': unesc(params),
                    'deprecated': s['deprecated'], 'heading': s['heading'], 'cat': cat,
                }
    return g

def rust_escape(s):
    return s.replace('\\', '\\\\').replace('"', '\\"')

# Hand-written doc overrides for constants where this crate has
# crate-specific rationale beyond the vendor description (e.g. cross-links
# to other constants/modules) -- keyed by Event ID. Applied verbatim in
# place of the scraped description, so they survive re-generation.
DOC_OVERRIDES = {
    'COM_RADIO_SET': (
        "COM1, legacy 25 kHz `FrequencyBcd16`-encoded frequency. Prefer "
        "`COM_RADIO_SET_HZ` for new code -- it works for both 25 kHz and "
        "8.33 kHz-spaced radios, see `crate::bcd`'s module docs for why."),
    'COM2_RADIO_SET': (
        "COM2, legacy 25 kHz `FrequencyBcd16`-encoded frequency. Prefer "
        "`COM2_RADIO_SET_HZ` for new code."),
    'COM3_RADIO_SET': (
        "COM3, legacy 25 kHz `FrequencyBcd16`-encoded frequency. Prefer "
        "`COM3_RADIO_SET_HZ` for new code."),
    'COM_RADIO_SET_HZ': "COM1, exact Hz -- works for both 25 kHz and 8.33 kHz-spaced radios.",
    'COM2_RADIO_SET_HZ': "COM2, exact Hz.",
    'COM3_RADIO_SET_HZ': "COM3, exact Hz.",
}

def escape_doc_brackets(text):
    # rustdoc treats [x] as an intra-doc link attempt; escape so it renders literally
    return text.replace('[', '\\[').replace(']', '\\]')

def clean_desc(desc, params):
    desc = re.sub(r'\s+', ' ', desc).strip()
    if not desc:
        desc = '(no description provided by the vendor docs)'
    if params:
        desc = f"{desc} Parameters: {params}."
    return escape_doc_brackets(desc)

def wrap_doc(text, width=90):
    words = text.split(' ')
    lines, cur = [], ''
    for w in words:
        if len(cur) + len(w) + 1 > width:
            lines.append(cur)
            cur = w
        else:
            cur = (cur + ' ' + w).strip()
    if cur:
        lines.append(cur)
    return lines

def main():
    if '--fetch' in sys.argv:
        fetch_all()

    g2020 = build_global(CATS_2020, '2020')
    g2024 = build_global(CATS_2020 + CATS_2024_ONLY, '2024')
    only_2020 = set(g2020) - set(g2024)
    only_2024 = set(g2024) - set(g2020)
    both = set(g2020) & set(g2024)
    print(f"2020 total: {len(g2020)}  2024 total: {len(g2024)}")
    print(f"both: {len(both)}  only-2024(new/sunrise): {len(only_2024)}  "
          f"only-2020(dropped from 2024 docs): {len(only_2020)}")
    if only_2020:
        print("dropped (present in 2020 docs, absent from 2024 docs):", sorted(only_2020))

    os.makedirs(OUT, exist_ok=True)
    # regenerate each category file (using the 2024 docs' organization, since
    # that's this crate's current module layout)
    for cat in CATS_2020 + CATS_2024_ONLY:
        sections = load(cat, '2024')
        if sections is None:
            continue
        out_lines = []
        seen = set()
        for s in sections:
            heading = s['heading'] or 'Miscellaneous'
            entries = []
            for name, desc, params in s['events']:
                if name in seen:
                    continue
                seen.add(name)
                is_new = name in only_2024
                is_deprecated = s['deprecated']
                if name in DOC_OVERRIDES:
                    final_desc = escape_doc_brackets(DOC_OVERRIDES[name])
                else:
                    final_desc = clean_desc(unesc(desc), unesc(params))
                entries.append((name, final_desc, is_new, is_deprecated))
            if not entries:
                continue
            out_lines.append(f"// {heading}\n")
            for name, desc, is_new, is_deprecated in entries:
                for l in wrap_doc(desc):
                    out_lines.append(f"/// {l}\n")
                if is_deprecated:
                    out_lines.append('#[deprecated(note = "marked deprecated in the vendor SDK docs; do not use in new code")]\n')
                if is_new:
                    out_lines.append('#[cfg(feature = "sunrise")]\n')
                ident = f"_{name}" if name[0].isdigit() else name
                out_lines.append(f'pub const {ident}: &str = "{rust_escape(name)}";\n')
            out_lines.append("\n")

        with open(os.path.join(OUT, f"{cat}.rs"), 'w', encoding='utf-8') as f:
            f.writelines(out_lines)
        print(f"wrote {cat}.rs: {len(seen)} consts")

if __name__ == '__main__':
    main()
