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

class TestCase:
    def __init__(self, size: int, alive_square_size: int, steps: int):
        self.size = size
        self.alive_square_size = alive_square_size
        self.steps = steps
        self.initial_cells = self.generate_initial_cells_sample()

    def generate_initial_cells_sample(self) -> List[Tuple[int,int]]:
        result = []
        top_left = (self.size // 2 - self.alive_square_size // 2, self.size // 2 - self.alive_square_size // 2)
        for x in range(top_left[0], top_left[0] + self.alive_square_size):
            for y in range(top_left[1] + x % 2, top_left[1] + self.alive_square_size, 2):
                result.append((x, y))
        return result
    

def test_cases() -> List[TestCase]:
    return [
        TestCase(100, 10, 10),
        TestCase(100, 100, 10),
        TestCase(1000, 100, 10),
        TestCase(10000, 1000, 10),
        TestCase(10000, 2000, 10),
        TestCase(10000, 5000, 1),
        TestCase(100000, 10000, 1),
    ]

def get_test_case_file(dir: Path, case: TestCase) -> Path:
    filename =  f"case_grid{case.size}_cells{len(case.initial_cells)}_steps{case.steps}.txt"
    path = dir / filename
    if not path.exists():
        with open(path, "w", encoding="utf-8") as f:
            for x, y in case.initial_cells:
                f.write(f"{x},{y}\n")
    return path.absolute()


NEXT_LINE_RE = re.compile(
    r"Next generation is ready\. Active cells: (\d+)\. Elapsed time: ([0-9]+(?:\.[0-9]+)?) ms"
)
INIT_LINE_RE = re.compile(r"Initial alive cells:\s*(\d+)")

def build_stdin(sample: TestCase) -> str:
    return "N\n" * sample.steps + "Q\n"

def run_once(
    exe_path: str,
    label: str,
    case: TestCase,
    out_dir: str,
    input_file: Path
) -> Tuple[Path, List[float], int, int]:
    """
    Returns:
      log_path, per_step_ms (list), initial_count, last_active_cells
    """
    stdin_payload = build_stdin(case)

    base = f"{label}_grid{case.size}x_cells{len(case.initial_cells)}_steps{case.steps}"
    log_path = f"{out_dir}/{base}.log"

    # Run the program
    
    with open(log_path, "w", encoding="utf-8") as f:
        try:
            proc = subprocess.run(
                [exe_path, "-s", str(case.size), "-f", str(input_file)],
                input=stdin_payload,
                capture_output=True,
                text=True,
                timeout=200,
            )
            
            f.write("\n=== STDOUT ===\n")
            f.write(proc.stdout or "")
            f.write("\n=== STDERR ===\n")
            f.write(proc.stderr or "")
            
            per_step_ms = []
            last_active = -1
            for m in NEXT_LINE_RE.finditer(proc.stdout or ""):
                last_active = int(m.group(1))
                per_step_ms.append(float(m.group(2)))

            print(f"[OK] {label} {case.size}x{case.size} cells={len(case.initial_cells)} steps={case.steps}")

            return log_path, per_step_ms, last_active

        except subprocess.TimeoutExpired as e:
            print(f"[ERROR] {label} {case.size}x{case.size} cells={len(case.initial_cells)} steps={case.steps}", file=sys.stderr)

            f.write("\n=== STDOUT ===\n")
            f.write(e.stdout or "")
            f.write("\n=== STDERR ===\n")
            f.write(e.stderr or "")

            return "", [], 0
    


    

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
    
    test_case_dir = Path("./test_cases")
    os.makedirs(test_case_dir, exist_ok=True)
    
    out_dir = f"./results_{datetime.now().strftime('%Y%m%d-%H%M%S')}"
    os.makedirs(out_dir, exist_ok=True)
    summary_path = f"{out_dir}/summary_{datetime.now().strftime('%Y%m%d-%H%M%S')}.csv"

    with open(summary_path, "w", newline="", encoding="utf-8") as csvfile:
        writer = csv.writer(csvfile)
        writer.writerow([
            "label","cols","rows","initial_cell_count","steps",
            "mean_ms","median_ms","min_ms","max_ms",
            "initial_alive","final_alive","log_file"
        ])

        for label, exe_path in args.exe:
            # quick sanity check
            if not Path(exe_path).exists():
                print(f"[WARN] Executable not found for '{label}': {exe_path}", file=sys.stderr)
                continue

            for case in test_cases():
                filename = get_test_case_file(test_case_dir, case)
                log_path, per_step_ms, last_alive = run_once(exe_path, label, case, out_dir, filename)

                # Compute stats
                if per_step_ms:
                    mean_ms = stats.fmean(per_step_ms)
                    median_ms = stats.median(per_step_ms)
                    min_ms = min(per_step_ms)
                    max_ms = max(per_step_ms)
                else:
                    mean_ms = median_ms = min_ms = max_ms = std_ms = -1.0

                writer.writerow([
                    label, case.size, case.size, len(case.initial_cells), case.steps,
                    f"{mean_ms:.3f}", f"{median_ms:.3f}", f"{min_ms:.3f}", f"{max_ms:.3f}",
                    last_alive, str(log_path)
                ])
                csvfile.flush()

    print(f"\nDone. Logs + CSV: {summary_path}")

if __name__ == "__main__":
    main()
