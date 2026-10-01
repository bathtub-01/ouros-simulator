#!/usr/bin/env python3
"""Plot injection rate versus packet latency for the Ouros NoC evaluator.

Run from the repository root:
    python3 plot_noc.py

By default reads simu-out/noc/summary.csv and writes
simu-out/noc/injection_rate_vs_delay.png.

Dependencies:
    python3 -m pip install matplotlib

Example comparisons:
    python3 plot_noc.py --metric e2e
    python3 plot_noc.py --pattern hotspot --mesh 4x4
    python3 plot_noc.py --metric p95 --output simu-out/noc/p95.pdf
"""

import argparse
import csv
from collections import defaultdict
from pathlib import Path

# Evaluator summary.csv column names.
X_FIELDS = {
    "offered": ("offered_rate", "Offered injection rate (packets/node/cycle)"),
    "accepted": (
        "accepted_rate_generation",
        "Accepted injection rate during generation (packets/node/cycle)",
    ),
    "throughput": ("throughput_per_node", "Throughput (packets/node/cycle)"),
}
Y_FIELDS = {
    "network": ("mean_network_delay", "Mean network delay (cycles)"),
    "e2e": ("mean_end_to_end", "Mean end-to-end delay (cycles)"),
    "source": ("mean_source_wait", "Mean source waiting time (cycles)"),
    "p50": ("p50_network_delay", "P50 network delay (cycles)"),
    "p95": ("p95_network_delay", "P95 network delay (cycles)"),
    "p99": ("p99_network_delay", "P99 network delay (cycles)"),
}


def parse_args():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument(
        "--input", type=Path, default=Path("simu-out/noc/summary.csv"),
        help="NoC evaluator summary CSV (default: %(default)s)",
    )
    parser.add_argument(
        "--output", type=Path, default=Path("simu-out/noc/injection_rate_vs_delay.png"),
        help="Output figure: .png, .pdf, or .svg (default: %(default)s)",
    )
    parser.add_argument(
        "--metric", choices=Y_FIELDS, default="network",
        help="Latency statistic to plot (default: %(default)s)",
    )
    parser.add_argument(
        "--x", choices=X_FIELDS, default="offered",
        help="Horizontal-axis rate (default: %(default)s)",
    )
    parser.add_argument(
        "--pattern", choices=("all", "random", "hotspot"), default="all",
        help="Select traffic pattern (default: %(default)s)",
    )
    parser.add_argument(
        "--mesh", default=None, metavar="WxH",
        help="Select mesh size, for example 4x4 (default: all sizes)",
    )
    parser.add_argument(
        "--seed", type=int, default=None,
        help="Select random seed (default: plot each seed separately)",
    )
    parser.add_argument("--dpi", type=int, default=180, help="Raster output DPI (default: %(default)s)")
    parser.add_argument("--show", action="store_true", help="Also show plot in an interactive window")
    return parser.parse_args()


def load_data(path, x_field, y_field, selected_pattern, selected_mesh, selected_seed):
    # Group independent experiments by pattern, mesh, hotspot configuration and seed.
    # If the same experiment/rate appears more than once in an appended summary,
    # prefer its most recent row rather than plotting duplicate x-coordinates.
    curves = defaultdict(dict)
    required = {"pattern", "width", "height", "seed", "offered_rate", x_field, y_field}
    with path.open("r", newline="", encoding="utf-8-sig") as handle:
        reader = csv.DictReader(handle)
        missing = required - set(reader.fieldnames or [])
        if missing:
            raise ValueError(f"Missing CSV columns: {', '.join(sorted(missing))}")

        for line_no, row in enumerate(reader, start=2):
            try:
                pattern = row["pattern"].strip().lower()
                if selected_pattern != "all" and pattern != selected_pattern:
                    continue
                w, h, seed = int(row["width"]), int(row["height"]), int(row["seed"])
                if selected_mesh is not None and (w, h) != selected_mesh:
                    continue
                if selected_seed is not None and seed != selected_seed:
                    continue

                hotspot = None
                if pattern == "hotspot":
                    hotspot = (
                        int(row["hotspot_x"]),
                        int(row["hotspot_y"]),
                        float(row["hotspot_prob"]),
                    )
                key = (pattern, w, h, hotspot, seed)
                curves[key][float(row["offered_rate"])] = (
                    float(row[x_field]), float(row[y_field])
                )
            except (KeyError, ValueError) as error:
                raise ValueError(f"Invalid row {line_no}: {error}") from error

    if not curves:
        raise ValueError("No matching data in summary.csv; check --pattern, --mesh, and --seed")
    return curves


def format_label(key):
    pattern, w, h, hotspot, seed = key
    label = f"{pattern.capitalize()} ({w}x{h})"
    if hotspot is not None:
        hx, hy, probability = hotspot
        label += f", hotspot=({hx},{hy}), p={probability:g}"
    return f"{label}, seed={seed}"


def main():
    args = parse_args()
    if args.mesh is not None:
        try:
            w, h = map(int, args.mesh.lower().split("x"))
            if w < 1 or h < 1:
                raise ValueError()
            mesh = (w, h)
        except ValueError:
            raise SystemExit("--mesh must be WxH (e.g. --mesh 4x4)")
    else:
        mesh = None

    # Non-interactive mode works on remote servers without a display.
    import matplotlib
    if not args.show:
        matplotlib.use("Agg")
    import matplotlib.pyplot as plt

    x_field, x_label = X_FIELDS[args.x]
    y_field, y_label = Y_FIELDS[args.metric]
    try:
        curves = load_data(args.input, x_field, y_field, args.pattern, mesh, args.seed)
    except (OSError, ValueError) as error:
        raise SystemExit(f"noc plot: {error}") from error

    fig, ax = plt.subplots(figsize=(7.5, 4.8))
    for key, observations in sorted(curves.items(), key=lambda item: str(item[0])):
        points = [observations[rate] for rate in sorted(observations)]
        ax.plot(
            [p[0] for p in points],
            [p[1] for p in points],
            marker="o",
            linewidth=1.8,
            markersize=4,
            label=format_label(key),
        )

    ax.set_xlabel(x_label)
    ax.set_ylabel(y_label)
    sizes = {(key[1], key[2]) for key in curves}
    if len(sizes) == 1:
        w, h = next(iter(sizes))
        ax.set_title(f"NoC injection rate vs. delay ({w}x{h} mesh)")
    else:
        ax.set_title("NoC injection rate vs. delay")
    ax.set_xlim(left=0)
    ax.set_ylim(bottom=0)
    ax.grid(True, alpha=0.3)
    ax.legend(fontsize=8, frameon=False)
    fig.tight_layout()

    args.output.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(args.output, dpi=args.dpi, bbox_inches="tight")
    print(f"Wrote {args.output} ({sum(len(points) for points in curves.values())} points, {len(curves)} curves)")
    if args.show:
        plt.show()
    plt.close(fig)


if __name__ == "__main__":
    main()
