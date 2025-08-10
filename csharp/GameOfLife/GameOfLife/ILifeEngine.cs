namespace GameOfLife;

public interface ILifeEngine
{
    int Cols { get; }
    int Rows { get; }
    void Next();
    
    void ActivateCell(int x, int y);
    
    IEnumerable<Cell> GetActiveCells();
    
    void GenerateRandomSquare(Cell topLeft, uint size);

}