# Use `just work day-01 part1` to work on the specific binary for a specific day's problems
work day part:
    cargo watch -w {{day}} -x "check -p {{day}}" -s "just test {{day}} {{part}}" -s "just lint {{day}}"
lint day:
    cargo clippy -p {{day}}
test day part arg:
    cargo nextest run -p {{day}} {{part}} {{arg}}
bench-all:
    cargo bench -q > benchmarks.txt
bench day part:
    cargo bench --bench {{day}}-bench {{part}} >> benchmark-results/{{day}}.bench.txt
create day:
    cargo generate --path ./daily-template --name {{day}}