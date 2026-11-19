//! Prints what the DWARF index knows about some types of a program.
//!
//! ```text
//! cargo run -p crawrad-dap-rust --example dwarf_dump -- <program> <type name | --find=part>...
//! ```

use std::path::Path;
use std::time::Instant;

use crawrad_dap_rust::dwarf::Kind;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some((program, names)) = args.split_first() else {
        eprintln!("usage: dwarf_dump <program> <type name>...");
        std::process::exit(2);
    };
    let started = Instant::now();
    let Some(index) = crawrad_dap_rust::load_layouts(Path::new(program)) else {
        println!("no DWARF or PDB for {program}");
        return;
    };
    println!("{} types indexed in {:?}", index.len(), started.elapsed());
    for name in names {
        if let Some(part) = name.strip_prefix("--find=") {
            for (_, ty) in index.iter().filter(|(_, t)| t.name.contains(part)) {
                println!("{} (size {})", ty.name, ty.size);
            }
            continue;
        }
        match index.by_name(name) {
            None => println!("{name}: NOT FOUND"),
            Some((_, ty)) => {
                println!("{name}: size {}", ty.size);
                match &ty.kind {
                    Kind::Struct { members, variants } => {
                        for m in members {
                            let t = m.ty.map(|t| index.get(t).name.clone()).unwrap_or_default();
                            println!("  +{:<3} {}: {t}", m.offset, m.name);
                        }
                        if let Some(v) = variants {
                            if let Some(d) = &v.discr {
                                let t = d.ty.map(|t| index.get(t).name.clone()).unwrap_or_default();
                                println!("  discriminant +{} ({t})", d.offset);
                            }
                            for arm in &v.arms {
                                let t = arm
                                    .member
                                    .ty
                                    .map(|t| index.get(t).name.clone())
                                    .unwrap_or_default();
                                println!(
                                    "  arm {:?} -> {} (+{}) {t}",
                                    arm.discr_value, arm.member.name, arm.member.offset
                                );
                            }
                        }
                    }
                    other => println!("  {other:?}"),
                }
            }
        }
    }
}
