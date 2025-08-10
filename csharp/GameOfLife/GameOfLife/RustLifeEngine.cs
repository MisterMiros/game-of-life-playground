using System.Runtime.InteropServices;
using GameOfLife.RustBindings;

namespace GameOfLife;

public class RustLifeEngine : ILifeEngine, IDisposable
{
    public int Cols { get; }
    public int Rows { get; }

    private readonly EngineHandle _engineHandle;

    public RustLifeEngine(int cols, int rows)
    {
        Cols = cols;
        Rows = rows;
        _engineHandle = new EngineHandle();
        _engineHandle.Init((uint)cols, (uint)rows);
    }

    public void Next()
    {
        EngineNativeMethods.engine_next(_engineHandle.DangerousGetHandle());
    }
    public void ActivateCell(int x, int y)
    {
        EngineNativeMethods.engine_activate_cell(_engineHandle.DangerousGetHandle(), (uint)x, (uint)y);
    }

    public IEnumerable<Cell> GetActiveCells()
    {
        return GetActiveNativeCells().Select(c => new Cell((int)c.x, (int)c.y));
    }
    
    public IEnumerable<NativeCell> GetActiveNativeCells()
    {
        using var iteratorHandle = new CellsIteratorHandle();
        iteratorHandle.Init(_engineHandle);
        var next = EngineNativeMethods.engine_alive_cells_iterator_next(iteratorHandle.DangerousGetHandle());
        while (next != IntPtr.Zero)
        {
            yield return Marshal.PtrToStructure<NativeCell>(next);
            next = EngineNativeMethods.engine_alive_cells_iterator_next(iteratorHandle.DangerousGetHandle());
        }
    }

    public void GenerateRandomSquare(Cell topLeft, uint size)
    {
        EngineNativeMethods.engine_generate_random_square(_engineHandle.DangerousGetHandle(), (uint)topLeft.X, (uint)topLeft.Y, size);
    }

    public void Dispose()
    {
        _engineHandle.Dispose();
    }

    ~RustLifeEngine()
    {
        Dispose();
    }
}