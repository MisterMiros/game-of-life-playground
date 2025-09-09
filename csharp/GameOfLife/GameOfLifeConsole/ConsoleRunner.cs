using System.Diagnostics;
using GameOfLife;

namespace GameOfLifeConsole;

public class ConsoleRunner
{
    private class Reader
    {
        private StreamReader? FileReader { get; init; }
        
        public string? ReadLine()
        {
            if (FileReader == null)
            {
                return Console.ReadLine();
            }

            if (FileReader.EndOfStream)
            {
                return "END";
            }
            return FileReader.ReadLine();
        }

        public static Reader FromFile(string path) => new() {FileReader = new StreamReader(File.OpenRead(path))};
        public static Reader FromConsole() => new();
        public void Dispose() => FileReader?.Dispose();
    }

    private LifeEngine _lifeEngine;

    public void Run(Options options)
    {
        Console.WriteLine("Running Game of Life in console...");

        Console.WriteLine($"Grid size: {options.Size}x{options.Size}");

        Reader reader;
        if (options.File != null)
        {
            Console.WriteLine($"Reading initial cell configuration from file: {options.File}");
            reader = Reader.FromFile(options.File);
        }
        else
        {
            Console.WriteLine("Enter the initial cell configuration using the following format:\n" +
                              "- Each line should contain one cell position as x,y coordinates\n" +
                              "- Type 'END' on a new line when you have finished entering all cells");
            reader = Reader.FromConsole();
        }


        var initialCells = ReadInitialCells(options.Size, reader);
        _lifeEngine = new LifeEngine(options.Size, options.Size, initialCells);

        Console.WriteLine($"Initial alive cells: {initialCells.Count}");

        Console.WriteLine("Press 'N' to run the next generation, 'Q' to quit");

        var stopwatch = new Stopwatch();
        while (true)
        {
            var next = Console.ReadLine()?.Trim();
            if ("N".Equals(next, StringComparison.CurrentCultureIgnoreCase))
            {
                stopwatch.Restart();
                _lifeEngine.Next();
                stopwatch.Stop();
                Console.WriteLine(
                    $"Next generation is ready. Active cells: {_lifeEngine.GetActiveCellCount()}. Elapsed time: {stopwatch.ElapsedMilliseconds} ms"
                );
            }
            else if ("Q".Equals(next, StringComparison.CurrentCultureIgnoreCase))
            {
                break;
            }
        }

        reader.Dispose();
        Console.WriteLine("Game of Life finished");
    }

    private HashSet<Cell> ReadInitialCells(int size, Reader input)
    {
        HashSet<Cell> initialCells = new();
        while (true)
        {
            var cell = input.ReadLine();
            if (string.IsNullOrWhiteSpace(cell))
            {
                continue;
            }

            if ("END".Equals(cell, StringComparison.CurrentCultureIgnoreCase))
            {
                break;
            }

            var splitted = cell.Split(",");
            if (splitted.Length != 2)
            {
                HandleInvalidFormat();
            }

            if (!int.TryParse(splitted[0], out var x))
            {
                HandleInvalidFormat();
            }

            if (!int.TryParse(splitted[1], out var y))
            {
                HandleInvalidFormat();
            }

            if (x < 0 || x >= size || y < 0 || y >= size)
            {
                Console.WriteLine($"Invalid cell position: ({x}, {y}), aborting");
                Environment.Exit(1);
            }

            initialCells.Add(new Cell(x, y));
        }

        return initialCells;

        void HandleInvalidFormat()
        {
            Console.WriteLine($"Invalid cell format, aborting");
            Environment.Exit(1);
        }
    }
}