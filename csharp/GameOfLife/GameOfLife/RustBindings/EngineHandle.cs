namespace GameOfLife.RustBindings;

using System;
using System.Runtime.InteropServices;

internal sealed class EngineHandle : SafeHandle
{
    public EngineHandle() : base(IntPtr.Zero, ownsHandle: true) { }

    public void Init(uint cols, uint rows)
    {
        SetHandle(EngineNativeMethods.engine_new(cols, rows));
    }

    public override bool IsInvalid => handle == IntPtr.Zero;

    protected override bool ReleaseHandle()
    {
        EngineNativeMethods.engine_free(handle);
        // After freeing, set to zero to avoid double-free on finalization
        SetHandle(IntPtr.Zero);
        return true;
    }
}