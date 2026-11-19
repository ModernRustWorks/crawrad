"""Generates crates/crawrad-mcp/src/dialogs_table.rs: what is inside each dialog the
shell draws, read from its DSL, for `view.get_dialog` (#339).

Run from the repository root:  python tools/gen_dialog_table.py
The test `every_generated_dialog_matches_its_dsl` (crates/crawrad-mcp) fails when the
DSL and this table drift apart, in either direction: run this again.

New Project, Build and Options are written by hand in dialogs.rs (they have
repeated rows with the label that carries their text); every other dialog that
`dsl_main.rs` instantiates is generated here.
"""
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SHELL = os.path.join(ROOT, 'crates', 'crawrad-ide', 'src', 'ide_app')
HAND_WRITTEN = {'newproject_dialog', 'build_dialog', 'options_dialog'}


def kind_of(ty):
    """The control kind a widget type is read as, or None when it is not a control."""
    if ty == 'CrawradItemsRow':
        return 'Row'
    if 'Button' in ty or 'Radio' in ty:
        return 'Button'
    if 'Check' in ty:
        return 'Check'
    if 'Drop' in ty:
        return 'Drop'
    if ty in ('CrawradCampoDenso', 'CrawradPropInput', 'CrawradTextInput', 'CrawradLinesText', 'TextInput') or 'Input' in ty:
        return 'Text'
    if ty in ('CrawradDialogTitle', 'CrawradLabel', 'CrawradLabelMuted', 'CrawradMonoLabel', 'CrawradMonoLabelMuted', 'Label'):
        return 'Label'
    return None


def template_block(text, template):
    start = text.index(f'pub {template} = ')
    nxt = re.search(r'\n\s*pub Crawrad[A-Za-z]+ = ', text[start + 1:])
    return text[start: start + 1 + nxt.start()] if nxt else text[start:]


def dialogs():
    main = open(os.path.join(SHELL, 'dsl_main.rs'), encoding='utf-8').read()
    dsl_dir = os.path.join(SHELL, 'dialogs', 'dsl')
    sources = {f: open(os.path.join(dsl_dir, f), encoding='utf-8').read() for f in sorted(os.listdir(dsl_dir))}
    for instance, template in re.findall(r'\b([a-z_]+_dialog) = <(Crawrad[A-Za-z]+Dialog)>', main):
        if instance in HAND_WRITTEN:
            continue
        found = [(f, t) for f, t in sources.items() if f'pub {template} = ' in t]
        if not found:
            continue
        f, text = found[0]
        block = template_block(text, template)
        if 'dlg = <' not in block:
            continue
        controls = []
        for name, ty in re.findall(r'\b([a-z][a-z_0-9]*) = <([A-Za-z]+)', block):
            k = kind_of(ty)
            if k and name != 'dlg' and name not in [c[0] for c in controls]:
                controls.append((name, k))
        yield instance, template, f, controls


NOTES = os.path.join(ROOT, 'crates', 'crawrad-mcp', '.crawrad', 'notes')


def note_of(nid, title, signature, body):
    """Writes the note of a generated item when it has none, and keeps its signature current."""
    path = os.path.join(NOTES, nid + '.md')
    if os.path.exists(path):
        text = open(path, encoding='utf-8').read()
        text = re.sub(r'(?m)^signature: ".*"$', lambda _: f'signature: "{signature}"', text, count=1)
    elif body is None:
        return
    else:
        text = (f'---\nid: {nid}\nkind: doc\ntarget_file: src/dialogs_table.rs\nline_hint: 1\n'
                f'author:\ncreated:\ntitle: "{title}"\ntags: []\nsignature: "{signature}"\n---\n\n{body}')
    open(path, 'w', encoding='utf-8', newline='\n').write(text)


def main():
    out = ['//! **Generated** by `tools/gen_dialog_table.py` from the shell\'s dialog DSL:',
           '//! what is inside each dialog `view.get_dialog` reads (#339). Do not edit;',
           '//! run the generator, then `cargo fmt`. `every_generated_dialog_matches_its_dsl`',
           '//! fails when',
           '//! this table and the DSL drift apart.',
           '',
           'use super::dialogs::{ControlKind, ControlSpec, DialogSpec};',
           'use crawrad_note::note;',
           '',
           'const fn one(name: &\'static str, kind: ControlKind) -> ControlSpec {',
           '    ControlSpec { name, kind, count: 1, text_of: "" }',
           '}',
           '']
    names = []
    for instance, template, f, controls in dialogs():
        const = instance.replace('_dialog', '').upper()
        names.append(const)
        nid = 'dialogs-table-' + const.lower().replace('_', '-')
        out.append(f'#[note("{nid}")]')
        note_of(nid, const, f'pub const {const} : DialogSpec = DialogSpec',
                f'`{template}` (`dialogs/dsl/{f}`).\n\n# Examples\n\n```rust\n'
                f'use crawrad_mcp::dialogs_table::{const};\n\n'
                f'assert_eq!({const}.name, "{instance.replace("_dialog", "")}");\n```\n')
        out.append(f'pub const {const}: DialogSpec = DialogSpec {{')
        out.append(f'    name: "{instance.replace("_dialog", "")}",')
        out.append(f'    root: ["{instance}", "dlg"],')
        out.append('    controls: &[')
        for name, k in controls:
            out.append(f'        one("{name}", ControlKind::{k}),')
        out.append('    ],')
        out.append('};')
        out.append('')
    out.append('#[note("dialogs-table-generated")]')
    sig = f'pub const GENERATED : [DialogSpec ; {len(names)}] = [' + ''.join(f'{n} , ' for n in names).rstrip(' ') + ']'
    note_of('dialogs-table-generated', 'GENERATED', sig, None)
    out.append(f'pub const GENERATED: [DialogSpec; {len(names)}] = [{", ".join(names)}];')
    path = os.path.join(ROOT, 'crates', 'crawrad-mcp', 'src', 'dialogs_table.rs')
    open(path, 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
    print(f'{len(names)} dialogs written to {path}')


if __name__ == '__main__':
    main()
