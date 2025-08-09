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

    public void PrepareForCellInflux(int count)
    {
    }

    public void ActivateCell(int x, int y)
    {
        EngineNativeMethods.engine_activate_cell(_engineHandle.DangerousGetHandle(), (uint)x, (uint)y);
    }

    public IEnumerable<Cell> GetActiveCells()
    {
        using var iteratorHandle = new IteratorHandle();
        iteratorHandle.Init(_engineHandle);
        var next = EngineNativeMethods.engine_cells_iterator_next(iteratorHandle.DangerousGetHandle());
        while (next != IntPtr.Zero)
        {
            var native = Marshal.PtrToStructure<EngineNativeMethods.NativeCell>(next);
            yield return new Cell((int)native.x, (int)native.y);
            next = EngineNativeMethods.engine_cells_iterator_next(iteratorHandle.DangerousGetHandle());
        }
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