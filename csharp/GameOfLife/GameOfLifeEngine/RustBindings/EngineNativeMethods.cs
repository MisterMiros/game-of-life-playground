using System.Runtime.InteropServices;

namespace GameOfLife.RustBindings;

internal static class EngineNativeMethods
{
    private const string DllName = "game_of_life_ffi";

    // Engine lifecycle
    [DllImport(DllName, CallingConvention = CallingConvention.StdCall, ExactSpelling = true)]
    internal static extern IntPtr engine_new(uint cols, uint rows);

    [DllImport(DllName, CallingConvention = CallingConvention.StdCall, ExactSpelling = true)]
    internal static extern void engine_free(IntPtr engine);

    // Engine operations
    [DllImport(DllName, CallingConvention = CallingConvention.StdCall, ExactSpelling = true)]
    internal static extern void engine_next(IntPtr engine);

    [DllImport(DllName, CallingConvention = CallingConvention.StdCall, ExactSpelling = true)]
    internal static extern void engine_activate_cell(IntPtr engine, uint x, uint y);

    // Iterator over alive cells
    [DllImport(DllName, CallingConvention = CallingConvention.StdCall, ExactSpelling = true,
        EntryPoint = "engine_alive_cells_iterator_get")]
    internal static extern IntPtr engine_alive_cells_iterator_get(IntPtr engine);

    [DllImport(DllName, CallingConvention = CallingConvention.StdCall, ExactSpelling = true,
        EntryPoint = "engine_alive_cells_iterator_free")]
    internal static extern void engine_alive_cells_iterator_free(IntPtr it);

    [DllImport(DllName, CallingConvention = CallingConvention.StdCall, ExactSpelling = true,
        EntryPoint = "engine_alive_cells_iterator_next")]
    internal static extern IntPtr engine_alive_cells_iterator_next(IntPtr it);

    [DllImport(DllName, CallingConvention = CallingConvention.StdCall, ExactSpelling = true,
        EntryPoint = "engine_generate_random_square")]
    internal static extern void engine_generate_random_square(IntPtr engine, uint topLeftX, uint topLeftY,
        uint size);
}