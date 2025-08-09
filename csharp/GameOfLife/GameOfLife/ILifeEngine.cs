namespace GameOfLife;

public interface ILifeEngine
{
    int Cols { get; }
    int Rows { get; }
    void Next();
    
    void PrepareForCellInflux(int count);
    void ActivateCell(int x, int y);
    
    IEnumerable<Cell> GetActiveCells();

}