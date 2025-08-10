using System.Runtime.InteropServices;

namespace GameOfLife.RustBindings;

[StructLayout(LayoutKind.Sequential)]
public struct NativeCell
{
    public uint x;
    public uint y;
}