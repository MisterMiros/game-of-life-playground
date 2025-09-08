using CommandLine;

namespace GameOfLifeConsole;

public class Options
{
    [Option(
        'f',
        "file",
        Required = false,
        HelpText =
            "File to read the initial set of active cells from. If omitted, cells will be read from standard input.")]
    public string? File { get; init; }

    [Option('s', "size", Required = false, HelpText = "Size of the square grid", Default = 1000)]
    public int Size { get; init; }
}