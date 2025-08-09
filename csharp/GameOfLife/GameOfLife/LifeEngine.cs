namespace GameOfLife;

public class LifeEngine : ILifeEngine
{
    private HashSet<Cell> _activeCells = [];
    private HashSet<Cell> _potentialCells = [];

    public int Cols { get; }
    public int Rows { get; }

    private readonly IReadOnlyList<Cell> _neighbourShifts =
    [
        new(0, -1),
        new(0, 1),
        new(-1, 0),
        new(1, 0),
        new(-1, -1),
        new(-1, 1),
        new(1, -1),
        new(1, 1)
    ];

    public LifeEngine(int cols, int rows)
    {
        Cols = cols;
        Rows = rows;
    }

    public LifeEngine(int cols, int rows, HashSet<Cell> initialCells)
    {
        Cols = cols;
        Rows = rows;
        _activeCells.UnionWith(initialCells);
        _potentialCells.UnionWith(GetPotentialCells(initialCells));
    }

    private HashSet<Cell> GetPotentialCells(HashSet<Cell> activeCells)
    {
        var potentialCells = new HashSet<Cell>(activeCells.Capacity * 8);
        foreach (var activeCell in activeCells)
        {
            potentialCells.Add(activeCell);
            foreach (var neighbour in _neighbourShifts)
            {
                potentialCells.Add(activeCell.Shift(neighbour));
            }
        }

        return potentialCells;
    }

    public void Next()
    {
        var activeCellsNext = new HashSet<Cell>(_activeCells.Capacity);
        var potentialCellsNext = new HashSet<Cell>(_potentialCells.Capacity);

        foreach (var potentialCell in _potentialCells)
        {
            var isAlive = _activeCells.Contains(potentialCell);
            var neighbours = _neighbourShifts
                .Select(potentialCell.Shift)
                .Where(IsWithinBounds)
                .ToList();
            var aliveNeighboursCount = neighbours
                .Where(_activeCells.Contains)
                .Count();
            if (isAlive)
            {
                var shouldLive = aliveNeighboursCount is 2 or 3;
                if (shouldLive)
                {
                    activeCellsNext.Add(potentialCell);
                }
                else
                {
                    potentialCellsNext.Add(potentialCell);
                    foreach (var neighbour in neighbours)
                    {
                        potentialCellsNext.Add(neighbour);
                    }
                }
            }
            else if (aliveNeighboursCount is 3)
            {
                activeCellsNext.Add(potentialCell);
                potentialCellsNext.Add(potentialCell);
                foreach (var neighbour in neighbours)
                {
                    potentialCellsNext.Add(neighbour);
                }
            }
        }

        _activeCells = activeCellsNext;
        _potentialCells = potentialCellsNext;
    }

    private bool IsWithinBounds(Cell point)
    {
        return point.X >= 0 && point.X < Rows && point.Y >= 0 && point.Y < Cols;
    }

    public bool IsCellAlive(int x, int y)
    {
        return _activeCells.Contains(new Cell(x, y));
    }

    public IEnumerable<Cell> GetActiveCells()
    {
        return _activeCells;
    }

    public void GenerateRandomSquare(Cell topLeft, uint size)
    {
        return;
    }

    public void PrepareForCellInflux(int count)
    {
        _activeCells.EnsureCapacity(_activeCells.Capacity + count);
        _potentialCells.EnsureCapacity(_potentialCells.Capacity + count * 8);
    }

    public void ActivateCell(int x, int y)
    {
        var activeCell = new Cell(x, y);
        _activeCells.Add(activeCell);
        _potentialCells.Add(activeCell);
        foreach (var neighbour in _neighbourShifts)
        {
            _potentialCells.Add(activeCell.Shift(neighbour));
        }
    }
}