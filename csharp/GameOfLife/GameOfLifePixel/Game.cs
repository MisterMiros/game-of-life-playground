using System.Numerics;
using GameOfLife;
using Raylib_cs;

namespace GameOfLifePixel;

public class Game
{
    private readonly ILifeEngine _engine;
    private Camera2D _camera;

    private bool _isRunning = false;
    private readonly float _step;
    private float _accumulator = 0f;

    private readonly int _randomCellsSquareSide;

    public GameConfig Config { get; set; }

    private const KeyboardKey RunKey = KeyboardKey.Space;
    private const KeyboardKey NextKey = KeyboardKey.Enter;
    private const MouseButton MoveButton = MouseButton.Right;
    private const MouseButton GenerateSquareButton = MouseButton.Left;

    public Game(GameConfig config)
    {
        _engine = new RustLifeEngine(config.Cols, config.Rows);
        foreach (var point in MakeTestPoints())
        {
            _engine.ActivateCell(point.X, point.Y);
        }

        _camera = new Camera2D()
        {
            Target = new Vector2(0, 0),
            Offset = new Vector2(0, 0),
            Rotation = 0.0f,
            Zoom = 1.0f,
        };
        Config = config;
        _step = 1f / Config.GameIterationsPerSecond;
        var smallerSide = Math.Min(Config.Cols, Config.Rows);
        _randomCellsSquareSide = Math.Min(smallerSide / 10, 1000);
    }

    private HashSet<Cell> MakeTestPoints()
    {
        var result = new HashSet<Cell>();
        for (var i = 0; i < 250; i += 9)
        {
            for (var j = 0; j < 250; j += 9)
            {
                result.UnionWith(new HashSet<Cell>()
                {
                    new(0 + i, 1 + j),
                    new(1 + i, 2 + j),
                    new(2 + i, 0 + j),
                    new(2 + i, 1 + j),
                    new(2 + i, 2 + j)
                });
            }
        }

        return result;
    }

    public void Start()
    {
        Raylib.SetConfigFlags(ConfigFlags.ResizableWindow);
        Raylib.InitWindow(1024, 1024, "Game of Life");
        Raylib.SetTargetFPS(60);

        while (!Raylib.WindowShouldClose())
        {
            OnUpdate();
        }

        Raylib.CloseWindow();
    }

    private void OnUpdate()
    {
        HandleRunToggle();
        HandleMove();
        HandleZoom();
        HandleGenerateSquare();
        HandleNextGeneration();
        
        RunEngine();
        DrawCells();
    }

    private void HandleRunToggle()
    {
        if (!Raylib.IsKeyPressed(RunKey))
        {
            return;
        }

        // toggle running state
        _isRunning = !_isRunning;

        // reset accumulator when start running the game
        if (_isRunning)
        {
            _accumulator = 0f;
        }
    }

    private void HandleNextGeneration()
    {
        if (_isRunning || !Raylib.IsKeyPressed(NextKey))
        {
            return;
        }

        _engine.Next();
    }

    private void RunEngine()
    {
        if (!_isRunning)
        {
            return;
        }

        _accumulator += Raylib.GetFrameTime();
        while (_accumulator >= _step)
        {
            _engine.Next();
            _accumulator -= _step;
        }
    }

    private void HandleMove()
    {
        if (!Raylib.IsMouseButtonDown(MoveButton))
        {
            return;
        }

        var delta = Raylib.GetMouseDelta();
        _camera.Target -= delta / _camera.Zoom;
    }

    private void HandleZoom()
    {
        var mouseWheel = Raylib.GetMouseWheelMove();
        if (mouseWheel == 0f)
        {
            return;
        }

        var mouseScreen = Raylib.GetMousePosition();
        var mouseWorldBefore = Raylib.GetScreenToWorld2D(mouseScreen, _camera);

        var zoomFactor = (1f + mouseWheel / 10f);
        _camera.Zoom = Math.Clamp(_camera.Zoom * zoomFactor, Config.MinZoom, Config.MaxZoom);

        var mouseWorldAfter = Raylib.GetScreenToWorld2D(mouseScreen, _camera);
        _camera.Target += mouseWorldBefore - mouseWorldAfter;
    }

    private void HandleGenerateSquare()
    {
        if (!Raylib.IsMouseButtonPressed(GenerateSquareButton))
        {
            return;
        }

        var mouseScreen = Raylib.GetMousePosition();
        var mouseWorld = Raylib.GetScreenToWorld2D(mouseScreen, _camera);
        var center = ToCell(mouseWorld);

        var topLeftX = Math.Clamp(center.X - _randomCellsSquareSide / 2, 0, Config.Cols - 1);
        var topLeftY = Math.Clamp(center.Y - _randomCellsSquareSide / 2, 0, Config.Cols - 1);
        
        _engine.GenerateRandomSquare(new Cell(topLeftX, topLeftY), (uint)_randomCellsSquareSide);
    }

    private void DrawCells()
    {
        Raylib.BeginDrawing();
        Raylib.ClearBackground(Color.Black);

        Raylib.BeginMode2D(_camera);

        var topLeftWorld = Raylib.GetScreenToWorld2D(new Vector2(0, 0), _camera);
        var bottomRightWorld = Raylib.GetScreenToWorld2D(
            new Vector2(Raylib.GetScreenWidth(), Raylib.GetScreenHeight()), _camera);

        var topLeft = ToCell(topLeftWorld);
        var bottomRight = ToCell(bottomRightWorld);
        foreach (var cell in _engine.GetActiveCells())
        {
            if (cell.X < topLeft.X || cell.X > bottomRight.X || cell.Y < topLeft.Y || cell.Y > bottomRight.Y)
            {
                continue;
            }

            var px = (int)(cell.X * Config.CellSize);
            var py = (int)(cell.Y * Config.CellSize);
            Raylib.DrawRectangle(px, py, Config.CellSizeInt, Config.CellSizeInt, Color.White);
        }

        Raylib.DrawRectangleLines(0, 0, Config.Cols * Config.CellSizeInt, Config.Rows * Config.CellSizeInt,
            Color.White);

        Raylib.DrawFPS(10, 10);
        Raylib.DrawText(_isRunning ? "RUN" : "PAUSE", 10, 30, 20, Color.Lime);

        Raylib.EndMode2D();
        Raylib.EndDrawing();
    }

    private Cell ToCell(Vector2 point)
    {
        var x = Math.Clamp((int)MathF.Floor(point.X / Config.CellSize), 0, Config.Cols - 1);
        var y = Math.Clamp((int)MathF.Floor(point.Y / Config.CellSize), 0, Config.Cols - 1);
        return new Cell(x, y);
    }
}