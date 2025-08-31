#!/usr/bin/env python3
import argparse
import csv
import os
import random
import re
import subprocess
import sys
import time
import statistics as stats
from datetime import datetime
from pathlib import Path
from typing import List, Tuple


class TestDataSample:
    def __init__(self, cols: int, rows: int, alive_square_size: int, steps: int):
        self.cols = cols
        self.rows = rows
        self.alive_square_size = alive_square_size
        self.steps = steps
        self.initial_cells = self.generate_initial_cells_sample()

    def generate_initial_cells_sample(self) -> List[Tuple[int,int]]:
        result = []
        top_left = (self.cols // 2 - self.alive_square_size // 2, self.rows // 2 - self.alive_square_size // 2)
        for x in range(top_left[0], top_left[0] + self.alive_square_size):
            for y in range(top_left[1] + x % 2, top_left[1] + self.alive_square_size, 2):
                result.append((x, y))
        return result

NEXT_LINE_RE = re.compile(
    r"Next generation is ready\. Active cells: (\d+)\. Elapsed time: ([0-9]+(?:\.[0-9]+)?) ms"
)
INIT_LINE_RE = re.compile(r"Initial alive cells:\s*(\d+)")

def build_stdin(sample: TestDataSample) -> str:
    lines = []
    lines.append(f"{sample.cols},{sample.rows}")
    for x, y in sample.initial_cells:
        lines.append(f"{x},{y}")
    lines.append("END")

    # drive N steps, then quit
    for _ in range(sample.steps):
        lines.append("N")
    lines.append("Q")
    return "\n".join(lines) + "\n"

def run_once(
    exe_path: str,
    label: str,
    sample: TestDataSample,
    out_dir: str
) -> Tuple[Path, List[float], int, int]:
    """
    Returns:
      log_path, per_step_ms (list), initial_count, last_active_cells
    """
    stdin_payload = build_stdin(sample)

    base = f"{label}_grid{sample.cols}x{sample.rows}_cells{len(sample.initial_cells)}_steps{sample.steps}"
    log_path = f"{out_dir}/{base}.log"

    # Run the program
    try:
        proc = subprocess.run(
            [exe_path],
            input=stdin_payload,
            capture_output=True,
            text=True,
            timeout=300,
        )
    except subprocess.TimeoutExpired as e:
        print(f"[ERROR] {label} timed out: {e}", file=sys.stderr)
        return "", [], 0

    # Write raw outputs to log
    with open(log_path, "w", encoding="utf-8") as f:
        f.write("\n=== STDOUT ===\n")
        f.write(proc.stdout or "")
        f.write("\n=== STDERR ===\n")
        f.write(proc.stderr or "")
    
    per_step_ms = []
    last_active = -1
    for m in NEXT_LINE_RE.finditer(proc.stdout or ""):
        last_active = int(m.group(1))
        per_step_ms.append(float(m.group(2)))

    return log_path, per_step_ms, last_active

def generate_test_data() -> List[TestDataSample]:
    return [
        TestDataSample(100, 100, 10, 10),
        TestDataSample(100, 100, 100, 10),
        TestDataSample(1000, 1000, 100, 10),
        TestDataSample(1000, 10000, 1000, 10),
        TestDataSample(1000, 10000, 2000, 10),
        TestDataSample(10000, 10000, 5000, 1),
        TestDataSample(100000, 100000, 10000, 1),
    ]
    

def main():
    parser = argparse.ArgumentParser(
        description="Benchmark Conway's Game of Life console implementations."
    )
    parser.add_argument(
        "--exe",
        action="append",
        nargs=2,
        metavar=("LABEL", "PATH"),
        required=True,
        help="Implementation label and path, e.g. --exe rust ./gol_rust --exe csharp ./gol_csharp.exe",
    )
    args = parser.parse_args()
    out_dir = f"./results_{datetime.now().strftime('%Y%m%d-%H%M%S')}"
    os.makedirs(out_dir, exist_ok=True)

    # CSV summary
    csv_path = f"{out_dir}/summary_{datetime.now().strftime('%Y%m%d-%H%M%S')}.csv"
    with open(csv_path, "w", newline="", encoding="utf-8") as csvfile:
        writer = csv.writer(csvfile)
        writer.writerow([
            "label","cols","rows","initial_cell_count","steps",
            "mean_ms","median_ms","min_ms","max_ms",
            "initial_alive","final_alive","log_file"
        ])

        test_data = generate_test_data()
        for label, exe_path in args.exe:
            # quick sanity check
            if not Path(exe_path).exists():
                print(f"[WARN] Executable not found for '{label}': {exe_path}", file=sys.stderr)

            for sample in test_data:
                log_path, per_step_ms, last_alive = run_once(exe_path, label, sample, out_dir)

                # Compute stats
                if per_step_ms:
                    mean_ms = stats.fmean(per_step_ms)
                    median_ms = stats.median(per_step_ms)
                    min_ms = min(per_step_ms)
                    max_ms = max(per_step_ms)
                else:
                    mean_ms = median_ms = min_ms = max_ms = std_ms = -1.0

                writer.writerow([
                    label, sample.cols, sample.rows, len(sample.initial_cells), sample.steps,
                    f"{mean_ms:.3f}", f"{median_ms:.3f}", f"{min_ms:.3f}", f"{max_ms:.3f}",
                    last_alive, str(log_path)
                ])
                csvfile.flush()
                print(f"[OK] {label} {sample.cols}x{sample.rows} cells={len(sample.initial_cells)} steps={sample.steps}")

    print(f"\nDone. Logs + CSV: {csv_path}")

if __name__ == "__main__":
    main()
