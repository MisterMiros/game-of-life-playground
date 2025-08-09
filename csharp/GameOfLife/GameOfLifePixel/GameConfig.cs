namespace GameOfLifePixel;

public record GameConfig(
    int Cols,
    int Rows,
    float CellSize = 2f,
    float GameIterationsPerSecond = 10f,
    float MinZoom = 0.1f,
    float MaxZoom = 40f
)
{
    public int CellSizeInt => (int)CellSize;
}