using CommandLine;
using GameOfLife;
using GameOfLifeConsole;

var runner = new ConsoleRunner();
Parser.Default.ParseArguments<Options>(args).WithParsed(o => runner.Run(o));