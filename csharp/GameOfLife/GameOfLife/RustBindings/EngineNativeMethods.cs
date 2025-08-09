using System.Runtime.InteropServices;

namespace GameOfLife.RustBindings;

public class EngineNativeMethods
{
    private const string DllName = "game_of_life_engine";

    // Engine lifecycle
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern IntPtr engine_new(uint cols, uint rows);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern void engine_free(IntPtr engine);

    // Engine operations
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern void engine_next(IntPtr engine);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern void engine_activate_cell(IntPtr engine, uint x, uint y);

    // Iterator over alive cells
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "engine_alive_cells_iterator_get")]
    internal static extern IntPtr engine_alive_cells_iterator_get(IntPtr engine);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "engine_cells_iterator_free")]
    internal static extern void engine_cells_iterator_free(IntPtr it);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "engine_cells_iterator_next")]
    internal static extern IntPtr engine_cells_iterator_next(IntPtr it);

    // Native layout of Cell (repr(C) with two u32 fields)
    [StructLayout(LayoutKind.Sequential)]
    internal struct NativeCell
    {
        public uint x;
        public uint y;
    }

}