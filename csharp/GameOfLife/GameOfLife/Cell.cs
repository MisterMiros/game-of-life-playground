namespace GameOfLife;

public readonly record struct Cell(int X, int Y)
{
    public Cell Shift(Cell shift)
    {
        return new Cell(X + shift.X, Y + shift.Y);
    }
}