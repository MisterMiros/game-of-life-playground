namespace GameOfLife.RustBindings;

using System;
using System.Runtime.InteropServices;

internal sealed class CellsIteratorHandle : SafeHandle
{
    public CellsIteratorHandle() : base(IntPtr.Zero, ownsHandle: true) { }

    public override bool IsInvalid => handle == IntPtr.Zero;

    public void Init(EngineHandle engine)
    {
        SetHandle(EngineNativeMethods.engine_alive_cells_iterator_get(engine.DangerousGetHandle()));
    }

    protected override bool ReleaseHandle()
    {
        EngineNativeMethods.engine_alive_cells_iterator_free(handle);
        SetHandle(IntPtr.Zero);
        return true;
    }
}