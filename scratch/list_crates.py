import os, glob

with open('codex-rs/Cargo.toml', 'r', encoding='utf-8') as f:
    content = f.read()

members_part = content.split('members = [')[1].split(']')[0]
members = [x.strip().strip('"').strip("'") for x in members_part.split('\n') if x.strip() and not x.strip().startswith('#')]
members = [m.strip(',').strip('"').strip("'") for m in members if m.strip(',').strip('"').strip("'")]

print(f"Total remaining workspace members: {len(members)}")

crate_analysis = []
for m in sorted(members):
    crate_dir = f"codex-rs/{m}"
    if os.path.exists(crate_dir):
        rs_files = glob.glob(f"{crate_dir}/**/*.rs", recursive=True)
        total_loc = 0
        for rf in rs_files:
            try:
                total_loc += len(open(rf, encoding='utf-8', errors='ignore').readlines())
            except Exception:
                pass
        crate_analysis.append((m, len(rs_files), total_loc))

print("\n--- Workspace Crates Inventory ---")
for name, num_files, loc in crate_analysis:
    print(f"Crate: {name:32s} | Files: {num_files:4d} | LoC: {loc:6d}")
