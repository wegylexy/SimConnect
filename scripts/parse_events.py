import sys, re
from html.parser import HTMLParser

def expand_id(raw_id):
    # strip disambiguating suffixes like "AXIS_STEERING_SET(Helicopters)"
    name = re.sub(r'\([^)]*\)$', '', raw_id).strip()
    # expand literal range placeholders like "DEBUG_A-Z" or "DEBUG_0-9"
    m = re.match(r'^(.+_)([A-Z0-9])-([A-Z0-9])$', name)
    if m:
        prefix, lo, hi = m.groups()
        if lo.isdigit() and hi.isdigit():
            return [f"{prefix}{n}" for n in range(int(lo), int(hi) + 1)]
        if lo.isalpha() and hi.isalpha():
            return [f"{prefix}{chr(c)}" for c in range(ord(lo), ord(hi) + 1)]
    return [name]

class EventPageParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.sections = []
        self.in_h4 = False
        self.h4_text = []
        self.in_table = False
        self.in_tr = False
        self.in_td = False
        self.td_index = -1
        self.cur_ids = []       # ids collected in td[0]
        self.cur_td_text = []
        self.header_row = False

    def handle_starttag(self, tag, attrs):
        d = dict(attrs)
        if tag in ('h3', 'h4'):
            self.in_h4 = True
            self.h4_text = []
        elif tag == 'table':
            self.in_table = True
        elif tag == 'tr' and self.in_table:
            self.in_tr = True
            self.td_index = -1
            self.header_row = False
            self.row_tds = []
        elif tag == 'th':
            self.header_row = True
        elif tag == 'td' and self.in_tr:
            self.in_td = True
            self.td_index += 1
            self.cur_td_text = []
            if self.td_index == 0:
                self.cur_ids = []
        elif tag == 'a' and self.in_td and self.td_index == 0:
            if 'id' in d:
                for name in expand_id(d['id']):
                    self.cur_ids.append(name)

    def handle_endtag(self, tag):
        if tag in ('h3', 'h4'):
            self.in_h4 = False
            text = ''.join(self.h4_text).strip()
            text = re.sub(r'\s+', ' ', text)
            deprecated = 'deprecated' in text.lower()
            heading = re.sub(r'\(deprecated\)', '', text, flags=re.I).strip()
            self.sections.append({'heading': heading or text, 'deprecated': deprecated, 'events': []})
        elif tag == 'table':
            self.in_table = False
        elif tag == 'tr':
            self.in_tr = False
            if not self.header_row and self.cur_ids:
                desc = self.row_tds[-1] if self.row_tds else ''
                # Middle columns (everything between Event ID and Description) may
                # include a "Key Event" column (keybinding name(s), always all-KEY_
                # tokens) -- identify and skip it by content, since its presence/
                # position varies row to row (omitted entirely when there's also no
                # Parameters column).
                middle = self.row_tds[1:-1]
                is_key_event_col = lambda t: bool(t) and all(
                    tok.startswith('KEY_') for tok in t.split())
                params_parts = [t for t in middle if t and t.upper() != 'N/A' and not is_key_event_col(t)]
                params_text = ' '.join(params_parts)
                if not self.sections:
                    self.sections.append({'heading': None, 'deprecated': False, 'events': []})
                for name in self.cur_ids:
                    self.sections[-1]['events'].append((name, desc, params_text))
            self.cur_ids = []
        elif tag == 'td':
            self.in_td = False
            text = ''.join(self.cur_td_text).strip()
            text = re.sub(r'\s+', ' ', text)
            self.row_tds.append(text)

    def handle_data(self, data):
        if self.in_h4:
            self.h4_text.append(data)
        if self.in_td:
            self.cur_td_text.append(data)

def parse_file(path):
    with open(path, encoding='utf-8') as f:
        content = f.read()
    p = EventPageParser()
    p.feed(content)
    return p.sections

if __name__ == '__main__':
    path = sys.argv[1]
    sections = parse_file(path)
    total = 0
    for s in sections:
        print(f"## {s['heading']} (deprecated={s['deprecated']}) -- {len(s['events'])} events")
        for name, desc, params in s['events'][:3]:
            print(f"   {name!r}: {desc[:70]!r}  params={params!r}")
        total += len(s['events'])
    print("TOTAL:", total)
