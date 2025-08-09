namespace GameOfLife;

public record Cell(int X, int Y)
{
    public Cell Shift(Cell shift)
    {
        return new Cell(X + shift.X, Y + shift.Y);
    }
}