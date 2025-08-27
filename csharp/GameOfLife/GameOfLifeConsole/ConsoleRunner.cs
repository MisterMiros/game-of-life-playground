using GameOfLife;

namespace GameOfLifeConsole;

public class ConsoleRunner
{
    private LifeEngine _lifeEngine;

    public void Run()
    {
        Console.WriteLine("Running Game of Life in console...");

        Console.WriteLine("Enter the size of the grid (columns and rows) using the following format: cols,rows");

        var (cols, rows) = ReadGridSize();

        Console.WriteLine("Enter the initial cell configuration using the following format:\n" +
                          "- Each line should contain one cell position as x,y coordinates\n" +
                          "- Type 'END' on a new line when you have finished entering all cells");

        var initialCells = ReadInitialCells(cols, rows);
        _lifeEngine = new LifeEngine(cols, rows, initialCells);

        Console.WriteLine("Initial alive cells:");
        Console.WriteLine(FormatActiveCells());
        
        Console.WriteLine("Press 'N' to run the next generation, 'Q' to quit");

        while (true)
        {
            var next = Console.ReadLine();
            if ("N".Equals(next, StringComparison.CurrentCultureIgnoreCase))
            {
                _lifeEngine.Next();
                Console.WriteLine("Next generation:");
                Console.WriteLine(FormatActiveCells());
            }
            
            if ("Q".Equals(next, StringComparison.CurrentCultureIgnoreCase))
            {
                break;
            }
        }
        Console.WriteLine("Game of Life finished");
    }
    
    private (int cols, int rows) ReadGridSize()
    {
        var line = Console.ReadLine();
        if (string.IsNullOrWhiteSpace(line))
        {
            HandleInvalidFormat();
        }

        var splitted = line!.Split(",");
        if (splitted.Length != 2)
        {
            HandleInvalidFormat();
        }

        if (!int.TryParse(splitted[0], out var cols))
        {
            HandleInvalidFormat();
        }

        if (!int.TryParse(splitted[1], out var rows))
        {
            HandleInvalidFormat();
        }

        return (cols, rows);

        void HandleInvalidFormat()
        {
            Console.WriteLine($"Invalid grid format, aborting");
            Environment.Exit(1);
        }
    }

    private HashSet<Cell> ReadInitialCells(int cols, int rows)
    {
        HashSet<Cell> initialCells = new();
        while (true)
        {
            var cell = Console.ReadLine();
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

            if (x < 0 || x >= cols || y < 0 || y >= rows)
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

    private string FormatActiveCells()
    {
        return string.Join("\n", _lifeEngine.GetActiveCells().Select(c => $"{c.X},{c.Y}"));
    }
}